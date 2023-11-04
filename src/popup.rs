use leptos::*;
use leptos::html::Dialog;

#[component]
pub fn Popup(children: Children) -> impl IntoView {
    let dialog_ref = create_node_ref::<Dialog>();

    let on_close = move |_| {
        let dialog = dialog_ref.get().expect("dialog_ref should be loaded");

        dialog.set_open(false);
    };

    view! {
        <dialog _ref=dialog_ref open>
            <button autofocus on:click=on_close class="dialog-close"></button>
            <div class="dialog-container">
            {
                children().nodes.into_iter().collect::<Vec<_>>()
            }
            </div>
        </dialog>
    }
}