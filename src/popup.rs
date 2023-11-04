use leptos::*;
use leptos::html::Div;

#[component]
pub fn Popup(children: ChildrenFn) -> impl IntoView {
    let close_icon = "×";

    let (open, set_open) = create_signal(true);
    let on_close = move |_| {
        set_open.set(false);
    };

    view! {
        <Show when=move || open.get()>
            <div class="popup">
                <div class="popup-content">
                    <span on:click=on_close class="popup-close">{close_icon}</span>
                    {
                        children().nodes
                    }
                </div>
            </div>
        </Show>
    }
}