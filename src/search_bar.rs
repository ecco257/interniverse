use leptos::*;

#[component]
pub fn SearchBar(
	search_query: ReadSignal<String>, 
	set_search_query: WriteSignal<String>,
) -> impl IntoView {

    view! {
        <div class="search-bar-wrapper">
            <input
                type="text"
                class="search-bar"
                placeholder="Search for a company or position..."
                on:input=move |event| {
					set_search_query(event_target_value(&event));
				}
				prop:value=search_query
            />
        </div>
    }
}
