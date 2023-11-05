use leptos::*;
use leptos::leptos_dom::logging::console_log;
use crate::popup::Popup;
use crate::session::{end_session, get_session};

#[cfg(feature = "ssr")]
use crate::db::db;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct ProfileData {
    name: String,
    school: String,
}

#[server(GetProfile)]
pub async fn get_profile() -> Result<Option<ProfileData>, ServerFnError> {
    use crate::login::validate_session;

    let session = get_session().await?;

    if session.is_none() {
        return Ok(None)
    }

    let session = session.unwrap();

    let validated = validate_session(session.user_id, session.token).await;

    if validated.is_err() || !validated.unwrap() {
        return Ok(None)
    }

    let mut conn = db().await?;

    let rows = sqlx::query!("SELECT * FROM users WHERE id = $1", session.user_id)
        .fetch_all(&mut conn).await?;

    if rows.len() == 0 {
        return Ok(None);
    }

    let user = &rows[0];

    Ok(Some(
        ProfileData {
            name: user.name.clone(),
            school: user.school.clone(),
        }
    ))
}

#[component]
pub fn Profile(open: RwSignal<bool>, reload_profile: RwSignal<bool>) -> impl IntoView {
    let profile = create_blocking_resource(
        || (),
        |_| async move {
            let profile: Option<ProfileData> = match get_profile().await {
                Ok(profile) => profile,
                Err(_) => None
            };

            profile
        }
    );

    create_effect(move |_| {
        if reload_profile.get() {
            profile.refetch();

            reload_profile.set(false);
        }
    });

    let logout = move |_| {
        spawn_local(async move {
            end_session().await.ok();

            open.set(false);
        });
    };

    view! {
        <Popup width=MaybeSignal::Static(20) open=open>
            <div class="login-container">
                <h1>Profile</h1>
                <Suspense
                    fallback=move || view! { <p>"Loading..."</p> }
                >
                <Show
                    when=move || profile.get().is_some() && profile.get().unwrap().is_some()
                    fallback=move || view! { <p>"Failed to get profile"</p> }
                >
                <p><b>Name: </b>{profile.get().unwrap().unwrap().name}</p>
                <p><b>School: </b>{profile.get().unwrap().unwrap().school}</p>
                <button class="login-button" on:click=logout>Logout</button>
                </Show>
                </Suspense>
            </div>
        </Popup>
    }
}