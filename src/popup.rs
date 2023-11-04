use leptos::*;

#[component]
pub fn Popup<'a>(
    header: &'a MaybeSignal<String>,
    footer: &'a MaybeSignal<String>,
    children: ChildrenFn
) -> impl IntoView {
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
                    <h2>{header.get()}</h2>
                </div>
                <div class="popup-body">
                    {
                        children().nodes
                    }
                </div>
                <div class="popup-footer">
                    <h3>{footer.get()}</h3>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn PopupPage() -> impl IntoView {
    view! {
        <Popup header=&MaybeSignal::Static(String::from("TEST HEADER")) footer=&MaybeSignal::Static(String::from("TEST FOOTER"))>
            <p>Popup Test</p>
        </Popup>
    }
}