use super::icon::{Icon, IconAndText, IconAndText_Props, Icon_Props};
use crate::constants::{HERETIC_URL, REPO_URL};
use perseus::prelude::*;
use sycamore::{builder::prelude::*, component::Prop, prelude::*};

#[component]
pub fn Scaffold<'a, G: Html>(cx: Scope<'a>, state: ScaffoldProps<'a, G>) -> View<G> {
  let children = state.children.call(cx);

  // Use your own signal if needed
  let search_field = state.search.unwrap_or(create_signal(cx, String::new()));
  let loading = state.loading.unwrap_or(create_signal(cx, false));

  div()
    .c(component(|| {
      NavBar(
        cx,
        NavBar_Props::builder()
          .show_button(state.show_button)
          .search(search_field)
          .loading(loading)
          .build(),
      )
    }))
    .c(main().c(children))
    .c(component(|| Footer(cx)))
    .view(cx)
}

#[component]
fn Footer<G: Html>(cx: Scope) -> View<G> {
  footer()
    .class("footer")
    .c(
      div().class("content has-text-centered").c(
        p()
          .c(strong().t("Torrents-csv"))
          .t(" by ")
          .c(a().attr("href", HERETIC_URL).t("Heretic"))
          .t(". Made with ")
          .c(a().attr("href", "https://www.rust-lang.org").t("Rust"))
          .t(", ")
          .c(a().attr("href", "https://actix.rs").t("Actix"))
          .t(", ")
          .c(
            a()
              .attr("href", "https://framesurge.sh/perseus/en-US/")
              .t("Perseus"),
          )
          .t(", and ")
          .c(
            a()
              .attr("href", "https://sycamore-rs.netlify.app")
              .t("Sycamore"),
          )
          .t("."),
      ),
    )
    .view(cx)
}

#[component(inline_props)]
pub fn NavBar<'a, G: Html>(
  cx: Scope<'a>,
  show_button: bool,
  search: &'a Signal<String>,
  loading: &'a Signal<bool>,
) -> View<G> {
  let expanded = create_signal(cx, false);
  nav()
    .class("navbar is-transparent")
    .attr("role", "navigation")
    .attr("aria-label", "main navigation")
    .c(
      div()
        .class("navbar-brand")
        .c(a().class("navbar-item").attr("href", "").c(component(|| {
          IconAndText(
            cx,
            IconAndText_Props::builder()
              .text("Torrents-csv".to_string())
              .text_class(None)
              .icon("database")
              .build(),
          )
        })))
        .c(
          button()
            .class("navbar-burger")
            .on("click", |_| expanded.set(!*expanded.get()))
            .dyn_class("is-active", || *expanded.get())
            .attr("role", "button")
            .attr("aria-label", "menu")
            .bool_attr("aria-expanded", *expanded.get())
            .c(span().bool_attr("aria-hidden", true))
            .c(span().bool_attr("aria-hidden", true))
            .c(span().bool_attr("aria-hidden", true)),
        ),
    )
    .c(
      div()
        .class("navbar-menu")
        .dyn_class("is-active", || *expanded.get())
        .c(
          div()
            .class("navbar-end")
            .c(
              a()
                .class("navbar-item")
                .attr("href", REPO_URL)
                .c(component(|| {
                  Icon(cx, Icon_Props::builder().icon("github").build())
                })),
            )
            .c(component(|| {
              SearchField(
                cx,
                SearchField_Props::builder()
                  .search(search)
                  .loading(loading)
                  .show_button(show_button)
                  .navbar_item(true)
                  .build(),
              )
            })),
        ),
    )
    .view(cx)
}

#[component(inline_props)]
pub fn SearchField<'a, G: Html>(
  cx: Scope<'a>,
  search: &'a Signal<String>,
  loading: &'a Signal<bool>,
  show_button: bool,
  #[builder(default)] is_expanded: bool,
  #[builder(default)] navbar_item: bool,
) -> View<G> {
  form()
    .class("field has-addons")
    .on("submit", |_| {
      loading.set(true);
      navigate(&format!("search/{}", *search));
    })
    .dyn_class("is-hidden", move || !show_button)
    .dyn_class("navbar-item", move || navbar_item)
    .c(
      div()
        .class("control")
        .dyn_class("is-expanded", move || is_expanded)
        .c(view! {
          cx,
          input(class = "input", placeholder = "Search...", bind:value = search)
        }),
    )
    .c(
      div().class("control").c(
        button()
          .attr("type", "submit")
          .class("button is-info")
          .dyn_class("is-loading", || *loading.get())
          .dyn_bool_attr("disabled", || *loading.get())
          .t("Search"),
      ),
    )
    .view(cx)
}

#[derive(Prop)]
pub struct ScaffoldProps<'a, G: Html> {
  /// The content to put inside the layout.
  pub children: Children<'a, G>,
  pub show_button: bool,
  #[builder(default)]
  pub search: Option<&'a Signal<String>>,
  #[builder(default)]
  pub loading: Option<&'a Signal<bool>>,
}
