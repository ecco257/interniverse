use leptos::*;

#[component]
pub fn ListingPrev(
    company_name: String,
    position: String,
	description: String,
	id: i64,
) -> impl IntoView {
    view! {
		<a href={format!("/listing/{}", id)} style="text-decoration:none">
			<div class="listing-prev">
				<div class="left-items">
					<div class="listing-company">{company_name}</div>
					<div class="listing-position">{position}</div>
				</div>
				<div class="right-items">
					<div class="listing-description">{description}</div>
				</div>
			</div>
		</a>
    }
}
