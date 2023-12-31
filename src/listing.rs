use cfg_if::cfg_if;
use crate::comment::{Comment, get_comments, self};
use crate::popup::Popup;
use serde::{Deserialize, Serialize};
use crate::{login::*, listing};
use chrono::prelude::*;
use leptos::*;
use leptos::{ev::SubmitEvent, *};
use web_sys::window;

cfg_if! {
	if #[cfg(feature = "ssr")] {
		use crate::db::db;
    }
}

fn get_id_from_url() -> Option<i64> {
    if let Some(window) = window() {
        if let Some(location) = window.location().href().ok() {
            // Extract the ID from the URL.
            // Assuming the URL is something like 'http://example.com/postpage/123'
            let parts: Vec<&str> = location.split('/').collect();
            if parts.len() > 1 {
                return parts.last()?.parse::<i64>().ok();
            }
        }
    }
    None
}

// Listing contains information for a company's internship listing along with a list of comments
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Listing {
    company: String,
    position: String,
    description: String,
    url: String,
    id: i64,
    school: String,
}

// Implementation of getters for comment data
impl Listing {
    pub fn new(company: String, position: String, description: String, url: String, id: i64, school: String) -> Self {
        Listing {
            company,
            position,
            description,
            url,
            id,
            school,
        }
    }

    pub fn get_company(&self) -> &String {
        &self.company
    }

    pub fn get_position(&self) -> &String {
        &self.position
    }

    pub fn get_description(&self) -> &String {
        &self.description
    }

    pub fn get_url(&self) -> &String {
        &self.url
    }

    pub fn get_id(&self) -> i64 {
        self.id
    }

    pub fn get_school(&self) -> &String {
        &self.school
    }
}

#[server(GetListings, "/listings")]
pub async fn get_listings(school: String) -> Result<Result<Vec<Listing>, String>, ServerFnError> {
    let mut conn = db().await?;
    let listings = sqlx::query_as!(Listing, "SELECT * FROM listings WHERE school = $1", school)
        .fetch_all(&mut conn).await?;
    Ok(Ok(listings))
}

#[server(GetAllListings, "/all-listings")]
pub async fn get_all_listings() -> Result<Vec<Listing>, ServerFnError> {
    let mut conn = db().await?;
    let listings = sqlx::query_as!(Listing, "SELECT * FROM listings")
        .fetch_all(&mut conn).await?;
    Ok(listings)
}

#[server(GetListing, "/server")]
pub async fn get_listing(id: i64) -> Result<Option<Listing>, ServerFnError> {
    let mut conn = db().await?;

    // Perform a query that selects a listing by ID. Adjust SQL as needed.
    let result = sqlx::query_as!(Listing, "SELECT * FROM listings WHERE id = $1", id)
        .fetch_one(&mut conn).await?;
    Ok(result)
}

#[server(AddListing, "/add-listing")]
pub async fn add_listing(listing: Listing) -> Result<Result<(), String>, ServerFnError> {
    let mut conn = db().await?;
    let rows = sqlx::query!("INSERT INTO listings (company, position, description, url, id, school) VALUES ($1, $2, $3, $4, $5, $6)",
        listing.company, listing.position, listing.description, listing.url, listing.id, listing.school)
        .execute(&mut conn).await?;
    Ok(Ok(()))
}

// Renders a navbar structure
#[component]
pub fn Listing(listing_data: ReadSignal<Listing>) -> impl IntoView {
    use leptos::html::Input;
    let mut next_id = 4;
    // Create initial list and store as a signal
    let (comments, set_comments) = create_signal(vec![
        Comment::new(
            String::from("Bob"),
            String::from("Hi. My name is bob. I don't know how to tell you this, but I am the author of the bible. Yep. I am jessus the lord and savior. Just kidding, I'm no god, but Jesus christ is my ni-"),
            Utc::now().timestamp_millis(),
            0.65,
            1,
        ),
        Comment::new(
            String::from("John"),
            String::from("Hi. My name is John. I am a bot."),
            Utc::now().timestamp_millis(),
            0.8,
            2,
        ),
        Comment::new(
            String::from("Jane"),
            String::from("I hate this job"),
            Utc::now().timestamp_millis(),
            0.2,
            3,
        ),
    ] as Vec<Comment>);
    // Average rating, precalled to the above list of sample comments
    let (avg_rating, set_avg_rating) = create_signal(get_avg_rating(&comments.get()));
    // Writing Comments Signals
    let (input_content, set_input_content) =
        create_signal(String::from(""));
    let input_element: NodeRef<Input> = create_node_ref();
    let (star_input, set_star_input) = create_signal(4);

    // On button click, make sure a comment is written and signed in
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let value = input_element().expect("<input> to exist").value();
        // If our string is good we add it as follows:
        set_input_content(value);
        // Add Value
        let new_comment = Comment::new(
            String::from("Guest"),
            input_content.get(),
            Utc::now().timestamp_millis(),
            star_input.get() as f64 / 5.0,
            next_id,
        );
        // Add the comment to the list of comments
        set_comments.update(move |comments| {
            comments.push(new_comment);
        });

        set_avg_rating.update(move |val| {
            *val = get_avg_rating(&comments.get());
        });
        next_id += 1;
    };

    // Gets an average rating given a list of comments
    fn get_avg_rating(comment_list: &Vec<Comment>) -> f64 {
        let mut total_rating = 0.0;
        let mut count = 0.0;
        for c in comment_list.iter() {
            count += 1.0;
            total_rating += c.get_rating();
        }

        total_rating / count
    }

    // Generates stars based on the value of progress
    fn generate_star_avg(filled_stars: f64) -> Vec<impl IntoView> {
        (1..=5).map(|i| {
            if (i as f64) <= filled_stars {
                view! {
                    <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-star-filled" width="44" height="44" viewBox="0 0 24 24" stroke-width="1.5" stroke="#ffbf00" fill="none" stroke-linecap="round" stroke-linejoin="round">
                    <path stroke="none" d="M0 0h24v24H0z" fill="none"/>
                    <path d="M8.243 7.34l-6.38 .925l-.113 .023a1 1 0 0 0 -.44 1.684l4.622 4.499l-1.09 6.355l-.013 .11a1 1 0 0 0 1.464 .944l5.706 -3l5.693 3l.1 .046a1 1 0 0 0 1.352 -1.1l-1.091 -6.355l4.624 -4.5l.078 -.085a1 1 0 0 0 -.633 -1.62l-6.38 -.926l-2.852 -5.78a1 1 0 0 0 -1.794 0l-2.853 5.78z" stroke-width="0" fill="currentColor" />
                </svg>
                }
            } else {
                view! {
                    <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-star" width="44" height="44" viewBox="0 0 24 24" stroke-width="1.5" stroke="#ffbf00" fill="none" stroke-linecap="round" stroke-linejoin="round">
                    <path stroke="none" d="M0 0h24v24H0z" fill="none"/>
                    <path d="M12 17.75l-6.172 3.245l1.179 -6.873l-5 -4.867l6.9 -1l3.086 -6.253l3.086 6.253l6.9 1l-5 4.867l1.179 6.873z" />
                    </svg>
                }
            }
        }).collect()
    }

    // Star input bar signals
    let (star_1, set_star_1) = create_signal(String::from("<svg xmlns='http://www.w3.org/2000/svg' class='icon icon-tabler icon-tabler-star' width='44' height='44' viewBox='0 0 24 24' stroke-width='1.5' stroke='#ffbf00' fill='none' stroke-linecap='round' stroke-linejoin='round'>
    <path stroke='none' d='M0 0h24v24H0z' fill='none'/>
    <path d='M12 17.75l-6.172 3.245l1.179 -6.873l-5 -4.867l6.9 -1l3.086 -6.253l3.086 6.253l6.9 1l-5 4.867l1.179 6.873z' />
    </svg>"));
    let (star_2, set_star_2) = create_signal(String::from("<svg xmlns='http://www.w3.org/2000/svg' class='icon icon-tabler icon-tabler-star' width='44' height='44' viewBox='0 0 24 24' stroke-width='1.5' stroke='#ffbf00' fill='none' stroke-linecap='round' stroke-linejoin='round'>
    <path stroke='none' d='M0 0h24v24H0z' fill='none'/>
    <path d='M12 17.75l-6.172 3.245l1.179 -6.873l-5 -4.867l6.9 -1l3.086 -6.253l3.086 6.253l6.9 1l-5 4.867l1.179 6.873z' />
    </svg>"));
    let (star_3, set_star_3) = create_signal(String::from("<svg xmlns='http://www.w3.org/2000/svg' class='icon icon-tabler icon-tabler-star' width='44' height='44' viewBox='0 0 24 24' stroke-width='1.5' stroke='#ffbf00' fill='none' stroke-linecap='round' stroke-linejoin='round'>
    <path stroke='none' d='M0 0h24v24H0z' fill='none'/>
    <path d='M12 17.75l-6.172 3.245l1.179 -6.873l-5 -4.867l6.9 -1l3.086 -6.253l3.086 6.253l6.9 1l-5 4.867l1.179 6.873z' />
    </svg>"));
    let (star_4, set_star_4) = create_signal(String::from("<svg xmlns='http://www.w3.org/2000/svg' class='icon icon-tabler icon-tabler-star' width='44' height='44' viewBox='0 0 24 24' stroke-width='1.5' stroke='#ffbf00' fill='none' stroke-linecap='round' stroke-linejoin='round'>
    <path stroke='none' d='M0 0h24v24H0z' fill='none'/>
    <path d='M12 17.75l-6.172 3.245l1.179 -6.873l-5 -4.867l6.9 -1l3.086 -6.253l3.086 6.253l6.9 1l-5 4.867l1.179 6.873z' />
    </svg>"));
    let (star_5, set_star_5) = create_signal(String::from("<svg xmlns='http://www.w3.org/2000/svg' class='icon icon-tabler icon-tabler-star' width='44' height='44' viewBox='0 0 24 24' stroke-width='1.5' stroke='#ffbf00' fill='none' stroke-linecap='round' stroke-linejoin='round'>
    <path stroke='none' d='M0 0h24v24H0z' fill='none'/>
    <path d='M12 17.75l-6.172 3.245l1.179 -6.873l-5 -4.867l6.9 -1l3.086 -6.253l3.086 6.253l6.9 1l-5 4.867l1.179 6.873z' />
    </svg>"));

    let on_star_click1 = move |_| {
        let val = if 1 == star_input.get() {
            0
        } else {
            1
        };
        set_star_input(val);
        for i in 1..=5 {
            let svg_choice = if i <= val {
                String::from("<svg xmlns='http://www.w3.org/2000/svg' class='icon icon-tabler icon-tabler-star-filled' width='44' height='44' viewBox='0 0 24 24' stroke-width='1.5' stroke='#ffbf00' fill='none' stroke-linecap='round' stroke-linejoin='round'>
            <path stroke='none' d='M0 0h24v24H0z' fill='none'/>
            <path d='M8.243 7.34l-6.38 .925l-.113 .023a1 1 0 0 0 -.44 1.684l4.622 4.499l-1.09 6.355l-.013 .11a1 1 0 0 0 1.464 .944l5.706 -3l5.693 3l.1 .046a1 1 0 0 0 1.352 -1.1l-1.091 -6.355l4.624 -4.5l.078 -.085a1 1 0 0 0 -.633 -1.62l-6.38 -.926l-2.852 -5.78a1 1 0 0 0 -1.794 0l-2.853 5.78z' stroke-width='0' fill='currentColor' />
        </svg>")
            } else {
                String::from("<svg xmlns='http://www.w3.org/2000/svg' class='icon icon-tabler icon-tabler-star' width='44' height='44' viewBox='0 0 24 24' stroke-width='1.5' stroke='#ffbf00' fill='none' stroke-linecap='round' stroke-linejoin='round'>
        <path stroke='none' d='M0 0h24v24H0z' fill='none'/>
        <path d='M12 17.75l-6.172 3.245l1.179 -6.873l-5 -4.867l6.9 -1l3.086 -6.253l3.086 6.253l6.9 1l-5 4.867l1.179 6.873z' />
        </svg>")
            };
            match i {
                1 => set_star_1(svg_choice),
                2 => set_star_2(svg_choice),
                3 => set_star_3(svg_choice),
                4 => set_star_4(svg_choice),
                5 => set_star_5(svg_choice),
                _ => panic!(),
            }
        }
    };

    let on_star_click2 = move |_| {
        let val = if 2 == star_input.get() {
            0
        } else {
            2
        };
        set_star_input(val);
        for i in 1..=5 {
            let svg_choice = if i <= val {
                String::from("<svg xmlns='http://www.w3.org/2000/svg' class='icon icon-tabler icon-tabler-star-filled' width='44' height='44' viewBox='0 0 24 24' stroke-width='1.5' stroke='#ffbf00' fill='none' stroke-linecap='round' stroke-linejoin='round'>
            <path stroke='none' d='M0 0h24v24H0z' fill='none'/>
            <path d='M8.243 7.34l-6.38 .925l-.113 .023a1 1 0 0 0 -.44 1.684l4.622 4.499l-1.09 6.355l-.013 .11a1 1 0 0 0 1.464 .944l5.706 -3l5.693 3l.1 .046a1 1 0 0 0 1.352 -1.1l-1.091 -6.355l4.624 -4.5l.078 -.085a1 1 0 0 0 -.633 -1.62l-6.38 -.926l-2.852 -5.78a1 1 0 0 0 -1.794 0l-2.853 5.78z' stroke-width='0' fill='currentColor' />
        </svg>")
            } else {
                String::from("<svg xmlns='http://www.w3.org/2000/svg' class='icon icon-tabler icon-tabler-star' width='44' height='44' viewBox='0 0 24 24' stroke-width='1.5' stroke='#ffbf00' fill='none' stroke-linecap='round' stroke-linejoin='round'>
        <path stroke='none' d='M0 0h24v24H0z' fill='none'/>
        <path d='M12 17.75l-6.172 3.245l1.179 -6.873l-5 -4.867l6.9 -1l3.086 -6.253l3.086 6.253l6.9 1l-5 4.867l1.179 6.873z' />
        </svg>")
            };
            match i {
                1 => set_star_1(svg_choice),
                2 => set_star_2(svg_choice),
                3 => set_star_3(svg_choice),
                4 => set_star_4(svg_choice),
                5 => set_star_5(svg_choice),
                _ => panic!(),
            }
        }
    };

    let on_star_click3 = move |_| {
        let val = if 3 == star_input.get() {
            0
        } else {
            3
        };
        set_star_input(val);
        for i in 1..=5 {
            let svg_choice = if i <= val {
                String::from("<svg xmlns='http://www.w3.org/2000/svg' class='icon icon-tabler icon-tabler-star-filled' width='44' height='44' viewBox='0 0 24 24' stroke-width='1.5' stroke='#ffbf00' fill='none' stroke-linecap='round' stroke-linejoin='round'>
            <path stroke='none' d='M0 0h24v24H0z' fill='none'/>
            <path d='M8.243 7.34l-6.38 .925l-.113 .023a1 1 0 0 0 -.44 1.684l4.622 4.499l-1.09 6.355l-.013 .11a1 1 0 0 0 1.464 .944l5.706 -3l5.693 3l.1 .046a1 1 0 0 0 1.352 -1.1l-1.091 -6.355l4.624 -4.5l.078 -.085a1 1 0 0 0 -.633 -1.62l-6.38 -.926l-2.852 -5.78a1 1 0 0 0 -1.794 0l-2.853 5.78z' stroke-width='0' fill='currentColor' />
        </svg>")
            } else {
                String::from("<svg xmlns='http://www.w3.org/2000/svg' class='icon icon-tabler icon-tabler-star' width='44' height='44' viewBox='0 0 24 24' stroke-width='1.5' stroke='#ffbf00' fill='none' stroke-linecap='round' stroke-linejoin='round'>
        <path stroke='none' d='M0 0h24v24H0z' fill='none'/>
        <path d='M12 17.75l-6.172 3.245l1.179 -6.873l-5 -4.867l6.9 -1l3.086 -6.253l3.086 6.253l6.9 1l-5 4.867l1.179 6.873z' />
        </svg>")
            };
            match i {
                1 => set_star_1(svg_choice),
                2 => set_star_2(svg_choice),
                3 => set_star_3(svg_choice),
                4 => set_star_4(svg_choice),
                5 => set_star_5(svg_choice),
                _ => panic!(),
            }
        }
    };

    let on_star_click4 = move |_| {
        let val = if 4 == star_input.get() {
            0
        } else {
            4
        };
        set_star_input(val);
        for i in 1..=5 {
            let svg_choice = if i <= val {
                String::from("<svg xmlns='http://www.w3.org/2000/svg' class='icon icon-tabler icon-tabler-star-filled' width='44' height='44' viewBox='0 0 24 24' stroke-width='1.5' stroke='#ffbf00' fill='none' stroke-linecap='round' stroke-linejoin='round'>
            <path stroke='none' d='M0 0h24v24H0z' fill='none'/>
            <path d='M8.243 7.34l-6.38 .925l-.113 .023a1 1 0 0 0 -.44 1.684l4.622 4.499l-1.09 6.355l-.013 .11a1 1 0 0 0 1.464 .944l5.706 -3l5.693 3l.1 .046a1 1 0 0 0 1.352 -1.1l-1.091 -6.355l4.624 -4.5l.078 -.085a1 1 0 0 0 -.633 -1.62l-6.38 -.926l-2.852 -5.78a1 1 0 0 0 -1.794 0l-2.853 5.78z' stroke-width='0' fill='currentColor' />
        </svg>")
            } else {
                String::from("<svg xmlns='http://www.w3.org/2000/svg' class='icon icon-tabler icon-tabler-star' width='44' height='44' viewBox='0 0 24 24' stroke-width='1.5' stroke='#ffbf00' fill='none' stroke-linecap='round' stroke-linejoin='round'>
        <path stroke='none' d='M0 0h24v24H0z' fill='none'/>
        <path d='M12 17.75l-6.172 3.245l1.179 -6.873l-5 -4.867l6.9 -1l3.086 -6.253l3.086 6.253l6.9 1l-5 4.867l1.179 6.873z' />
        </svg>")
            };
            match i {
                1 => set_star_1(svg_choice),
                2 => set_star_2(svg_choice),
                3 => set_star_3(svg_choice),
                4 => set_star_4(svg_choice),
                5 => set_star_5(svg_choice),
                _ => panic!(),
            }
        }
    };

    let on_star_click5 = move |_| {
        let val = if 5 == star_input.get() {
            0
        } else {
            5
        };
        set_star_input(val);
        for i in 1..=5 {
            let svg_choice = if i <= val {
                String::from("<svg xmlns='http://www.w3.org/2000/svg' class='icon icon-tabler icon-tabler-star-filled' width='44' height='44' viewBox='0 0 24 24' stroke-width='1.5' stroke='#ffbf00' fill='none' stroke-linecap='round' stroke-linejoin='round'>
            <path stroke='none' d='M0 0h24v24H0z' fill='none'/>
            <path d='M8.243 7.34l-6.38 .925l-.113 .023a1 1 0 0 0 -.44 1.684l4.622 4.499l-1.09 6.355l-.013 .11a1 1 0 0 0 1.464 .944l5.706 -3l5.693 3l.1 .046a1 1 0 0 0 1.352 -1.1l-1.091 -6.355l4.624 -4.5l.078 -.085a1 1 0 0 0 -.633 -1.62l-6.38 -.926l-2.852 -5.78a1 1 0 0 0 -1.794 0l-2.853 5.78z' stroke-width='0' fill='currentColor' />
        </svg>")
            } else {
                String::from("<svg xmlns='http://www.w3.org/2000/svg' class='icon icon-tabler icon-tabler-star' width='44' height='44' viewBox='0 0 24 24' stroke-width='1.5' stroke='#ffbf00' fill='none' stroke-linecap='round' stroke-linejoin='round'>
        <path stroke='none' d='M0 0h24v24H0z' fill='none'/>
        <path d='M12 17.75l-6.172 3.245l1.179 -6.873l-5 -4.867l6.9 -1l3.086 -6.253l3.086 6.253l6.9 1l-5 4.867l1.179 6.873z' />
        </svg>")
            };
            match i {
                1 => set_star_1(svg_choice),
                2 => set_star_2(svg_choice),
                3 => set_star_3(svg_choice),
                4 => set_star_4(svg_choice),
                5 => set_star_5(svg_choice),
                _ => panic!(),
            }
        }
    };

    view! {
        <div class="listing">
            <div class="listing-main">
                <div class="listing-header">
                    <div class="listing-company">
                        {listing_data.get().get_company()}
                    </div>
                    <div class="star-rating">
                        <div class="stars">
                            {move || generate_star_avg(avg_rating.get() * 5.0)}
                        </div>
                        <div class="star-count">
                            {move || (avg_rating.get()*500.0).round()/100.0}
                        </div>
                    </div>
                    <a class ="listing-url" href={listing_data.get().get_url()} target="_blank">
                        <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-link" width="44" height="44" viewBox="0 0 24 24" stroke-width="1.5" stroke="#ffffff" fill="none" stroke-linecap="round" stroke-linejoin="round">
                            <path stroke="none" d="M0 0h24v24H0z" fill="none"/>
                            <path d="M9 15l6 -6" />
                            <path d="M11 6l.463 -.536a5 5 0 0 1 7.071 7.072l-.534 .464" />
                            <path d="M13 18l-.397 .534a5.068 5.068 0 0 1 -7.127 0a4.972 4.972 0 0 1 0 -7.071l.524 -.463" />
                        </svg>
                    </a>
                </div>
                <div class ="listing-position">
                    {listing_data.get().get_position()}
                </div>
                <div class ="listing-description">
                        {listing_data.get().get_description()}
                </div>
            </div>
            <div class="comment-container">
                <For
                each = comments
                key = |c| c.get_listing_id()
                children=move |c: Comment| {
                    view! {
                        <div class="comment-shell">
                            <Comment comment_data=c />
                        </div>
                    }
                }
                />
            </div>

            <form class="comment-form" on:submit=on_submit>
                <input class="comment-box" type="text"
                value = input_content
                placeholder = "Write your comment here."
                node_ref=input_element
                />
                <div class="star-rating">
                    <div class="stars">
                        <div type="button" on:click=on_star_click1 inner_html={star_1}/>
                        <div type="button" on:click=on_star_click2 inner_html={star_2}/>
                        <div type="button" on:click=on_star_click3 inner_html={star_3}/>
                        <div type="button" on:click=on_star_click4 inner_html={star_4}/>
                        <div type="button" on:click=on_star_click5 inner_html={star_5}/>
                    </div>
                </div>
                <input class="comment-submit" type="submit" value="Submit"/>
            </form>
        </div>
    }
}

pub fn ListingPage() -> impl IntoView {
    let id = get_id_from_url().unwrap_or(1); // If ID is not found, default to 1.

    // Create a resource that will fetch the listing data asynchronously.
    let listing_resource = create_resource(
        || (),
		|_| async move { get_listing(id).await },
    );

	let ulisting = listing_resource.get().unwrap();

	let (listing_data, set_listing_data) = create_signal(ulisting);

	view! {
		<Listing listing_data=listing_data />
	}
}
