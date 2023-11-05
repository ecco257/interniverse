use leptos::*;
use crate::popup::Popup;

pub fn Registration() -> impl IntoView {
    let open = create_rw_signal(true);
    let (username, set_username) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());
    let (school, set_school) = create_signal("".to_string());
    let (github, set_github) = create_signal("".to_string());
    let (linkedin, set_linkedin) = create_signal("".to_string());

    view! {
        <Popup width=MaybeSignal::Static(20) open=open>
            <div class="login-container">
                <h1>Registration</h1>
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
                <button class="login-button">Register</button>
            </div>
        </Popup>
    }
}

pub fn RegistrationPage() -> impl IntoView {
    view! {
        <Registration/>
    }
}