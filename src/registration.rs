use leptos::*;
use crate::popup::Popup;
use crate::login::*;
use leptos::{*, ev::SubmitEvent, leptos_dom::logging::console_log};
use crate::session::set_session;

#[component]
pub fn Registration(open: RwSignal<bool>, reload_profile: RwSignal<bool>, login_open: RwSignal<bool>) -> impl IntoView {
    let (username, set_username) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());
    let (school, set_school) = create_signal("".to_string());
    let (github, set_github) = create_signal("".to_string());
    let (linkedin, set_linkedin) = create_signal("".to_string());

    let (status, set_status) = create_signal("".to_string());

    let on_submit = move |_| {
        spawn_local(async move {
            console_log("Registering...");
            set_status("Registering...".to_string());

            let session = create_user(username.get(), password.get(), school.get()).await;

            match session {
                Ok(session) => {
                    set_session(session).await.expect("Failed to set session");
                    console_log("Registered");
                    set_status("".to_string());

                    reload_profile.set(true);
                    open.set(false);
                },
                Err(e) => {
                    console_log(&("Error: ".to_string() + e.to_string().as_str()));
                    set_status("Failed to register: ".to_string() + e.to_string().as_str());
                }
            }
        })
    };

    let on_login = move |_| {
        open.set(false);
        login_open.set(true);
    };

    view! {
        <Popup width=MaybeSignal::Static(20) open=open>
            <div class="login-container">
                <h1>Registration</h1>
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
                <label for="login-school-input"><b>School</b></label>
                <input
                    class="login-input"
                    name="login-school-input"
                    type="text"
                    on:input=move |ev| {
                        set_school(event_target_value(&ev));
                    }

                    prop:value=school
                />
                <label for="login-github-input"><b>GitHub</b></label>
                <input
                    class="login-input"
                    name="login-github-input"
                    type="text"
                    on:input=move |ev| {
                        set_github(event_target_value(&ev));
                    }

                    prop:value=github
                />
                <label for="login-linkedin-input"><b>LinkedIn</b></label>
                <input
                    class="login-input"
                    name="login-linkedin-input"
                    type="text"
                    on:input=move |ev| {
                        set_linkedin(event_target_value(&ev));
                    }

                    prop:value=linkedin
                />
                <button class="login-button" on:click=on_submit>Register</button>
                <button class="login-button" on:click=on_login>Already have an account? Login!</button>
            </div>
        </Popup>
    }
}