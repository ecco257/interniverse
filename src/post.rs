use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Post {
	pub id: i32,
	pub title: String,
	pub body: String,
}

cfg_if! {
	if #[cfg(feature = "ssr")] {
		use crate::db::db;
	}
}

#[server(GetPosts, "/posts")]
pub async fn get_posts() -> Result<Vec<Post>, ServerFnError> {
	println!("Getting db connection...");

	let mut conn = db().await?;
	let rows = sqlx::query_as!(Post, "SELECT * FROM posts_test").fetch_all(&mut conn).await?;

	println!("Got posts: {:?}", rows);

	Ok(rows)
}

/*
#[component]
fn NumericInput() -> impl IntoView {
    let (value, set_value) = create_signal(Ok(0));

    let on_input = move |ev| set_value(event_target_value(&ev).parse::<i32>());

    view! {
        <h1>"Error Handling"</h1>
        <label>
            "Type a number (or something that's not a number!)"
            <input type="number" on:input=on_input/>
            <ErrorBoundary
                // the fallback receives a signal containing current errors
                fallback=|errors| view! {
                    <div class="error">
                        <p>"Not a number! Errors: "</p>
                        // we can render a list of errors as strings, if we'd like
                        <ul>
                            {move || errors.get()
                                .into_iter()
                                .map(|(_, e)| view! { <li>{e.to_string()}</li>})
                                .collect_view()
                            }
                        </ul>
                    </div>
                }
            >
                <p>"You entered " <strong>{value}</strong></p>
            </ErrorBoundary>
        </label>
    }
}
*/

#[component]
pub fn ShowData(posts: Result<Vec<Post>, leptos::ServerFnError>) -> impl IntoView {
	view! {
		<ErrorBoundary
			fallback=|errors| view! {
				<p>"Errors getting Posts: " {errors.get().into_iter()
					.map(|(_, e)| view! { <li>{e.to_string()}</li>})
					.collect_view()
				}</p>
			}
		>
		<ul>
			{posts.unwrap().into_iter().map(|post| view! {
				<li>
					<h2>{post.title}</h2>
					<p>{post.body}</p>
				</li>
			}).collect_view()}
		</ul>
		</ErrorBoundary>
	}
}

#[component]
pub fn Posts() -> impl IntoView {
	let posts = create_resource(
		|| (),
		|_| async move { get_posts().await },
	);

	view! {
		<h1>"My Data"</h1>
		<Suspense
			fallback=move || view! { <p>"Loading..."</p> }
		>
		{move || {
			posts.get().map(|posts| view! { <ShowData posts/> })
		}}
		</Suspense>
		"Leptos is pretty cool"
	}
}
