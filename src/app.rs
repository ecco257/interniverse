use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos::leptos_dom::Text;
use wasm_bindgen::JsCast;
use crate::listing::ListingPage;
use crate::popup::PopupPage;
use crate::header::Header;
use crate::search_bar::SearchBar;
use crate::listing_prev::ListingPrev;
use crate::login::LoginPage;
use leptos::leptos_dom::logging::console_log;
use crate::registration::RegistrationPage;
use crate::session::SessionPage;
use crate::profile::ProfilePage;

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
                    <Route path="/popup-test" view=PopupPage/>
                    <Route path="/login-test" view=LoginPage/>
                    <Route path="/registration-test" view=RegistrationPage/>
                    <Route path="/listing-test" view=ListingPage/>
                    <Route path="/session-test" view=SessionPage/>
                    <Route path="/profile-test" view=ProfilePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Listing {
    pub company_name: String,
    pub position: String,
    pub num_comments: String,
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {

	let (all_listings, set_all_listings) = create_signal(vec![
        Listing {
            company_name: "Company 1".to_string(),
            position: "Position 1".to_string(),
            num_comments: "1".to_string(),
        },
        Listing {
            company_name: "Company 2".to_string(),
            position: "Position 2".to_string(),
            num_comments: "2".to_string(),
        },
        Listing {
            company_name: "Company 3".to_string(),
            position: "Position 3".to_string(),
            num_comments: "3".to_string(),
        },
    ]);

	let (search_query, set_search_query) = create_signal(String::new());

    let (filtered_listings, set_filtered_listings) = create_signal::<View>(View::Text(Text::new("No results".to_string().into())));

    view! {
		<div class="home-page">
            <Header/>
            <SearchBar 
                search_query=search_query 
                set_search_query=set_search_query
            />
            <div>{filtered_listings}</div>
            {
                create_effect(move |_| {
                    let filter_text = search_query.get().to_lowercase();
                        let all_listings = all_listings.get();
						let filtered = all_listings.iter()
							.filter(|listing| listing.company_name.to_lowercase().contains(&filter_text) ||
                                listing.position.to_lowercase().contains(&filter_text))
							.collect::<Vec<&Listing>>();						
						let content = filtered.iter().map(|listing| {
							view! {
								<ListingPrev
									company_name=listing.company_name.clone()
									position=listing.position.clone()
									num_comments=listing.num_comments.clone()
								/>
							}
                        }).collect_view();

                        set_filtered_listings(content);
                });
            }
        </div>
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
