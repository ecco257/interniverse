use leptos::*;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct SessionModel {
    pub user_id: i32,
    pub token: String
}

#[server(GetSession)]
pub async fn get_session() -> Result<Option<SessionModel>, ServerFnError> {
    use leptos_actix::extract;
    use actix_session::Session;

    Ok(extract(
        |session: Session| async move {
            session.get::<SessionModel>("session")
        }
    ).await??)
}

#[server(SetSession)]
pub async fn set_session(model: SessionModel) -> Result<(), ServerFnError> {
    use leptos_actix::extract;
    use actix_session::Session;
    use actix_web::web::{ Json };

    Ok(extract(
        |session: Session| async move {
            session.insert("session", model.clone())
        }
    ).await??)
}

#[component]
pub fn SessionPage() -> impl IntoView {
    let (current, set_current) = create_signal("".to_string());

    let refresh_session = move |_| {
        spawn_local(async move {
            set_current("Loading...".to_string());

            let s = get_session().await.unwrap();

            if let Some(model) = s {
                set_current(model.token);
            } else {
                set_current("No token".to_string()  )
            }
        })
    };

    let set_thing = move |_| {
        spawn_local(async move {
            let model = SessionModel {
                token: "TEST THING".to_string(),
                user_id: 2,
            };

            set_session(model).await.unwrap();
        })
    };

    view! {
        <p>Session: {current}</p>
        <button on:click=refresh_session>Refresh</button>
        <button on:click=set_thing>Set Thing</button>
    }
}