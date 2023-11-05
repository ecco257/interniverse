use leptos::*;

#[component]
pub fn ListingPrev(
    company_name: String,
    position: String,
    num_comments: String,
) -> impl IntoView {
    view! {
        <div class="listing-prev">
			<div class="left-items">
				<div class="listing-company">{company_name}</div>
				<div class="listing-position">{position}</div>
				<div class="listing-comments">{num_comments} " comments"</div>
			</div>
			<div class="right-items">
				<div class="listing-rating">"Rating"</div>
			</div>
        </div>
    }
}
