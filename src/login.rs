use cfg_if::cfg_if;
use leptos::{*, ev::SubmitEvent};
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Session {
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

		async fn create_user(username: String, password: String, school: String) -> Result<Session, ServerFnError> {
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


use leptos::*;
use crate::popup::Popup;

pub fn Login() -> impl IntoView {
    let open = create_rw_signal(true);
    let (username, set_username) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());

    view! {
        <Popup width=MaybeSignal::Static(20) open=open>
            <div class="login-container">
                <h1>Login</h1>
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
                <button class="login-button">Login</button>
            </div>
        </Popup>
    }
}

pub fn LoginPage() -> impl IntoView {
    view! {
        <Login/>
    }
}