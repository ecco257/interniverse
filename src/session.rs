use leptos::*;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
struct SessionModel {
    token: String
}

#[server(GetSession)]
pub async fn get_session() -> Result<String, ServerFnError> {
    use leptos_actix::extract;
    use actix_session::Session;

    extract(
        |session: Session| async move {
            match session.get::<String>("message") {
                Ok(message_option) => {
                    match message_option {
                        Some(message) => message,
                        None => "Not Found".to_string(),
                    }
                },
                Err(_) => "Error".to_string(),
            }
        }
    ).await
}

#[server(SetSession)]
pub async fn set_session(model: SessionModel) -> Result<(), ServerFnError> {
    use leptos_actix::extract;
    use actix_session::Session;
    use actix_web::web::{ Json };

    Ok(extract(
        |session: Session| async move {
            session.insert("message", model.token.clone())
        }
    ).await??)
}

pub fn SessionPage() -> impl IntoView {
    let (current, set_current) = create_signal("".to_string());

    let refresh_session = move |_| {
        spawn_local(async move {
            set_current("Loading...".to_string());

            let s = get_session().await.unwrap();

            set_current(s);
        })
    };

    let set_thing = move |_| {
        spawn_local(async move {
            let model = SessionModel {
                token: "TEST THING".to_string(),
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