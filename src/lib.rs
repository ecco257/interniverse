pub mod app;
pub mod db;
pub mod popup;
pub mod login;

pub mod comment;
use cfg_if::cfg_if;
pub mod header;
pub mod search_bar;

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
