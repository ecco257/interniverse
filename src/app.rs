use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos::leptos_dom::Text;
use wasm_bindgen::JsCast;
use crate::listing::ListingPage;
use crate::listing::Listing;
use crate::listing::get_all_listings;
use crate::header::Header;
use crate::search_bar::SearchBar;
use crate::listing_prev::ListingPrev;
use crate::login::Login;
use leptos::leptos_dom::logging::console_log;
use crate::registration::Registration;
use crate::session::SessionPage;
use crate::profile::Profile;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
 
    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                    <Route path="/listing-test" view=ListingPage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {

	let all_listings = create_resource(
		|| (),
		|_| async move { get_all_listings().await },
	);

	console_log(&format!("all_listings: {:?}", all_listings.get()));

	let (search_query, set_search_query) = create_signal(String::new());

    let (filtered_listings, set_filtered_listings) = create_signal::<View>(View::Text(Text::new("No results".to_string().into())));

    let profile_open = create_rw_signal(false);

    let login_open = create_rw_signal(false);

    let register_open = create_rw_signal(false);

    let reload_profile = create_rw_signal(false);

    view! {
		<div class="home-page">
            <Header profile_open=profile_open login_open=login_open/>
            <SearchBar 
                search_query=search_query 
                set_search_query=set_search_query
            />
			<Suspense fallback=move || view! { <div>"Loading..."</div> }>
            	<div>{filtered_listings}</div>
			</Suspense>
            {
                create_effect(move |_| {
                    let filter_text = search_query.get().to_lowercase();
					let all_listings = match all_listings.get() {
						Some(Ok(listings)) => listings,
						Some(Err(_)) => return,
						None => return,
					};
					if all_listings.is_empty() {
						return;
					}
					let filtered = all_listings.iter().filter(|listing| {
						listing.get_company().to_lowercase().contains(&filter_text) || listing.get_position().to_lowercase().contains(&filter_text)
					}).collect::<Vec<&Listing>>();
					let content = filtered.iter().map(|listing| {
						view! {
							<ListingPrev
								company_name=listing.get_company().clone()
								position=listing.get_position().clone()
								description=listing.get_description().clone()
								id=listing.get_id()
							/>
						}
					}).collect_view();

					set_filtered_listings(content);
                });
            }
        </div>
        <Profile open=profile_open reload_profile=reload_profile/>
        <Login open=login_open reload_profile=reload_profile register_open=register_open/>
        <Registration open=register_open reload_profile=reload_profile login_open=login_open/>
    }
}


/// Renders the new post page of your application.
/// This is where users can create new posts.
#[component]
fn NewPost() -> impl IntoView {
	view! {
		<h1>"New Post"</h1>
	}
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
