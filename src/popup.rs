use leptos::*;

#[component]
pub fn Popup(children: ChildrenFn) -> impl IntoView {
    let close_icon = "×";

    let (open, set_open) = create_signal(true);
    let on_close = move |_| {
        set_open.set(false);
    };

    view! {
        <div class="popup" style:display=move || if open.get() { "block" } else { "none" }>
            <div class="popup-content">
                <div class="popup-header">
                    <span on:click=on_close class="popup-close">{close_icon}</span>
                    {
                        children().nodes
                    }
                    <h2>Modal Header</h2>
                </div>
                <div class="popup-body">
                    <p>Some text in the Modal Body</p>
                    <p>Some other text...</p>
                </div>
                <div class="popup-footer">
                    <h3>Modal Footer</h3>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn PopupPage() -> impl IntoView {
    view! {
        <Popup>
            <p>Popup Test</p>
        </Popup>
    }
}