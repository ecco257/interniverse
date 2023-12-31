pub mod app;
pub mod db;
pub mod popup;
pub mod listing;
pub mod comment;
use cfg_if::cfg_if;
pub mod header;
pub mod search_bar;
pub mod listing_prev;
pub mod login;
pub mod registration;
pub mod session;
pub mod profile;

cfg_if! {
if #[cfg(feature = "hydrate")] {

  use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    pub fn hydrate() {
      use app::*;
      use leptos::*;

      console_error_panic_hook::set_once();

      leptos::mount_to_body(App);
    }
}
}
