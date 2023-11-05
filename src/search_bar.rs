use leptos::*;

#[component]
pub fn SearchBar() -> impl IntoView {
    let (search_query, set_search_query) = create_signal(String::new());

    view! {
        <div class="search-bar-wrapper">
            <input
                type="text"
                class="search-bar"
                placeholder="Search for a company or position..."
                value={search_query}
                on:input=move |event| { set_search_query.update(|search_query| *search_query = event.to_string().into()); }
            />
        </div>
    }
}