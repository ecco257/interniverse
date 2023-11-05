use cfg_if::cfg_if;
use leptos::{*, ev::SubmitEvent};
use leptos::leptos_dom::logging::console_log;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Session {
	session_token: String,
	user_id: i32,
	expiry_date: i64
}

cfg_if! {
	if #[cfg(feature = "ssr")] {
		use crate::db::db;
		use tokio;

		use pbkdf2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
		use pbkdf2::Pbkdf2;

		use chrono;

		use rand::rngs::OsRng;

		async fn create_session(id: i32) -> Result<(String, i64), ServerFnError> {
			let mut u128_pool = [0u8; 16];
			u128_pool = rand::random::<[u8; 16]>();

			let session_token = u128::from_le_bytes(u128_pool);
			let expiry_date = (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp_millis();

			let mut conn = db().await?;
			let rows = sqlx::query!("INSERT INTO sessions (session_token, user_id, expiry_date) VALUES ($1, $2, $3)",
				session_token.to_string(), id, expiry_date)
			.execute(&mut conn).await?;

			Ok((session_token.to_string(), expiry_date))
		}

		async fn validate_session(id: i32, session_token: String) -> Result<bool, ServerFnError> {
			let mut conn = db().await?;
			let rows = sqlx::query!("SELECT * FROM sessions WHERE user_id = $1 AND session_token = $2",
				id, session_token)
				.fetch_all(&mut conn).await?;

			if rows.len() == 0 {
				return Ok(false);
			}

			let expiry_date = rows[0].expiry_date;

			if expiry_date < chrono::Utc::now().timestamp_millis() {
				return Ok(false);
			}

			Ok(true)
		}
	}
}

#[server]
pub async fn create_user(username: String, password: String, school: String) -> Result<Session, ServerFnError> {
	let salt = SaltString::generate(&mut OsRng);
	let hashed_password = Pbkdf2.hash_password(password.as_bytes(), &salt).unwrap().to_string();

	let id = rand::random::<i32>();

	let mut conn = db().await?;
	let rows = sqlx::query!("INSERT INTO users (id, name, password, school) VALUES ($1, $2, $3, $4)",
		id, username, hashed_password, school)
		.execute(&mut conn).await?;

	let (session_token, expiry_date) = create_session(id).await?;

	debug_assert!(validate_session(id, session_token.clone()).await?);

	Ok(Session {
		session_token: session_token,
		user_id: id,
		expiry_date: expiry_date,
	})
}

#[server]
pub async fn login_user(username: String, password: String) -> Result<Result<Session, String>, ServerFnError> {
	println!("Logging in user...");

	let mut conn = db().await?;
	let rows = sqlx::query!("SELECT * FROM users WHERE name = $1", username)
		.fetch_all(&mut conn).await?;

	if rows.len() == 0 {
		return Ok(Err("User not found".to_string()));
	}

	let user = &rows[0];
	let hashed_password = PasswordHash::new(&user.password).unwrap();

	match Pbkdf2.verify_password(password.as_bytes(), &hashed_password) {
		Ok(_) => {
			let (session_token, expiry_date) = create_session(user.id).await?;

			debug_assert!(validate_session(user.id, session_token.clone()).await?);
		
			Ok(Ok(Session {
				session_token: session_token,
				user_id: user.id,
				expiry_date: expiry_date,
			}))
		},
		Err(_) => {
			return Ok(Err("Incorrect password".to_string()));
		}
	}
}

#[server(SetSessionCookie)]
pub async fn set_session_cookies(session: Session) -> Result<(), ServerFnError> {
    use actix_web::{cookie::Cookie, http::header, http::header::HeaderValue, http::StatusCode};
    use leptos_actix::ResponseOptions;

    let response = expect_context::<ResponseOptions>();
    response.set_status(StatusCode::OK);

	// Convert session.expiry_date to a date object
	let expiry_date = chrono::DateTime::<chrono::Utc>::from_utc(chrono::NaiveDateTime::from_timestamp(session.expiry_date, 0), chrono::Utc);

	let id_cookie = Cookie::build("user_id", session.user_id.to_string()).path("/").finish();
	let session_cookie = Cookie::build("session", session.session_token).path("/").finish();

    if let (Ok(id_cookie), Ok(session_cookie)) = (HeaderValue::from_str(&id_cookie.to_string()), HeaderValue::from_str(&session_cookie.to_string())) {
		response.append_header(header::SET_COOKIE, id_cookie);
		response.append_header(header::SET_COOKIE, session_cookie);
		Ok(())
	} else {
		return Err(ServerFnError::ServerError("Failed to set cookie".to_string()));
	}
}

use leptos::*;
use crate::popup::Popup;

pub fn Login() -> impl IntoView {
    let open = create_rw_signal(true);
    let (username, set_username) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());

	let (status, set_status) = create_signal("".to_string());

	let on_submit = move |_| {
		spawn_local(async move {
			console_log("Logging in...");
			set_status("Logging in...".to_string());

			let session = login_user(username.get(), password.get()).await;

			match session {
				Ok(Ok(session)) => {
					let response = set_session_cookies(session).await;

					match response {
						Ok(_) => {
							console_log("Set session cookie");
							set_status("Logged in".to_string());
						},
						Err(e) => {
							console_log(&("Error:".to_string() + e.to_string().as_str()));
							set_status("Failed to set session cookie".to_string());
						}
					}
				},
				Ok(Err(e)) => {
					console_log(&("Error: ".to_string() + e.to_string().as_str()));
					set_status("Login failed: ".to_string() + e.to_string().as_str());
				},
				Err(e) => {
					console_log(&("Unknown error during login".to_string() + e.to_string().as_str()));
					set_status("Unknown error during login".to_string() + e.to_string().as_str());
				}
			}
		})
	};

    view! {
        <Popup width=MaybeSignal::Static(20) open=open>
            <div class="login-container">
                <h1>Login</h1>
				<p>{status}</p>
                <label for="login-username-input"><b>Username</b></label>
                <input
                    class="login-input"
                    name="login-username-input"
                    type="text"
                    on:input=move |ev| {
                        set_username(event_target_value(&ev));
                    }

                    prop:value=username
                />
                <label for="login-password-input"><b>Password</b></label>
                <input
                    class="login-input"
                    name="login-password-input"
                    type="password"
                    on:input=move |ev| {
                        set_password(event_target_value(&ev));
                    }

                    prop:value=password
                />
                <button class="login-button" on:click=on_submit>Login</button>
            </div>
        </Popup>
    }
}

#[component]
pub fn LoginPage() -> impl IntoView {
    view! {
        <Login/>
    }
}