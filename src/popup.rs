use leptos::*;
use std::fmt;

#[component]
pub fn Popup(
    open: RwSignal<bool>,
    width: MaybeSignal<u32>,
    children: ChildrenFn
) -> impl IntoView {
    let close_icon = "×";

    let on_close = move |_| {
        open.set(false);
    };

    view! {
        <div class="popup" style:display=move || if open.get() { "block" } else { "none" }>
            <div class="popup-content" style=format!("width: {}%;", width.get())>
                <span on:click=on_close class="popup-close">{close_icon}</span>
                    {
                        children().nodes
                    }
            </div>
        </div>
    }
}

#[component]
pub fn PopupPage() -> impl IntoView {
    let open = create_rw_signal(true);

    let on_open = move |_| {
        open.set(true);
    };

    view! {
        <button on:click=on_open>POPUP!</button>
        <Popup width=MaybeSignal::Static(80) open=open>
            <p>Popup Test</p>
        </Popup>
    }
}