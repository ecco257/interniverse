use leptos::*;
use crate::session::get_session;

#[component]
pub fn Header(profile_open: RwSignal<bool>, login_open: RwSignal<bool>) -> impl IntoView {
    // Signal for the toggle switch
    let (is_toggled, set_is_toggled) = create_signal(false);

    let on_profile = move |_| {
        spawn_local(async move {
            let session = get_session().await;

            if session.is_err() || session.unwrap().is_none() {
                login_open.set(true);
                profile_open.set(false);
            } else {
                profile_open.set(true);
                login_open.set(false);
            }
        });
    };

    view! {
        <header class="main-header">
            <div class="header-left">
                <h1>"Interniverse"</h1>
            </div>
            <div class="header-right">
                // Slider/Toggle
                <label class="switch">
					<input type="checkbox" checked={is_toggled} on:input=move |_| set_is_toggled.update(|is_toggled| *is_toggled = !*is_toggled) />
                    <span class="slider round"></span>
                </label>
                // School Name
                // Replace "YourSchoolName" with the actual name of the school, but if is_toggled is false, then display "All Schools"
				<span class="school-name">{ move || if is_toggled.get() { "RPI" } else { "All Schools" } }</span>
                // Plus Icon for New Post
                <a class="new-post-btn" href="/new-post">
					<svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-circle-plus" width="44" height="44" viewBox="0 0 24 24" stroke-width="1.5" stroke="#ffffff" fill="none" stroke-linecap="round" stroke-linejoin="round">
						<path stroke="none" d="M0 0h24v24H0z" fill="none"/>
						<path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" />
						<path d="M9 12h6" />
						<path d="M12 9v6" />
					</svg>
                </a>
                // Profile Circle Icon
                <div class="profile-menu">
                    <button class="profile-btn" aria-label="User profile" on:click=on_profile>
						<svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-user-circle" width="44" height="44" viewBox="0 0 24 24" stroke-width="1.5" stroke="#ffffff" fill="none" stroke-linecap="round" stroke-linejoin="round">
							<path stroke="none" d="M0 0h24v24H0z" fill="none"/>
							<path d="M12 12m-9 0a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" />
							<path d="M12 10m-3 0a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" />
							<path d="M6.168 18.849a4 4 0 0 1 3.832 -2.849h4a4 4 0 0 1 3.834 2.855" />
						</svg>
                    </button>
                </div>
            </div>
        </header>
    }
}

