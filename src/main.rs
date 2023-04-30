mod components;
mod constants;
mod error_views;
mod templates;
mod types;
mod utils;

use perseus::prelude::*;

#[perseus::main(perseus_axum::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
  PerseusApp::new()
    .template(crate::templates::index::get_template())
    .template(crate::templates::search::get_template())
    .error_views(crate::error_views::get_error_views())
}
