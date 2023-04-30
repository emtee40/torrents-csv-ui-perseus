use crate::{
  components::{
    head_common::{HeadCommon, HeadCommon_Props},
    scaffold::{Scaffold, ScaffoldProps},
  },
  constants::{ANDROID_APP_REPO_URL, REPO_URL},
};
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::{builder::prelude::*, component::Prop, prelude::*};

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "IndexPageStateRx")]
struct IndexPageState {
  search: String,
  loading: bool,
}

#[auto_scope]
fn index_page<G: Html>(cx: Scope, state: &IndexPageStateRx) -> View<G> {
  component(|| {
    Scaffold(
      cx,
      ScaffoldProps::builder()
        .children(
          div()
            .c(
              section().class("section").c(
                div()
                  .class("container")
                  .c(
                    div()
                      .class("field has-addons")
                      .c(
                        div()
                          .class("control is-expanded")
                          .c(view! { cx, input(class = "input", placeholder = "Search...", bind:value = state.search)}),
                      )
                      .c(
                        div().class("control").c(
                          a()
                            .class("button is-info")
                            .dyn_class("is-loading", || *state.loading.get())
                            .attr("rel", "external")
                            // TODO these are pointless because it isn't a form
                            .attr("minLength", "3")
                            .bool_attr("required", true)
                            .on("click", |_| state.loading.set(true))
                            .dyn_bool_attr("disabled", || *state.loading.get())
                            .dyn_attr("href", || Some(format!("search/{}", *state.search)))
                            .t("Search"),
                        ),
                      ),
                  )
                  .c(component(|| IntroText(cx))),
              ),
            )
            .view(cx)
            .into(),
        )
        .build(),
    )
  })
}

#[component]
pub fn IntroText<G: Html>(cx: Scope) -> View<G> {
  div()
    .class("content")
    .c(
      p()
        .c(a().attr("href", REPO_URL).t("Torrents.csv"))
        .t(" is a ")
        .c(i().t("collaborative"))
        .t(" git repository of torrents, consisting of a single, searchable ")
        .c(code().t("torrents.csv"))
        .t(
          " file. Its initially populated with a January
      2017 backup of the pirate bay, and new torrents are periodically added
      from various torrents sites. It comes with a self-hostable webserver,
      a command line search, and a folder scanner to add torrents.",
        ),
    )
    .c(p().c(a().attr("href", REPO_URL).t("Torrents.csv")).t(
      " will only store torrents with at
    least one seeder to keep the file small, will be periodically purged of
    non-seeded torrents, and sorted by seeders descending.",
    ))
    .c(
      p().c(
        a()
          .attr("href", ANDROID_APP_REPO_URL)
          .t("Torrents-csv-android app"),
      ),
    )
    .c(p().t("API: ").c(
      code().t(
        " https://torrents-csv.ml/service/search?q=[QUERY]&size=[NUMBER_OF_RESULTS]&page=[PAGE]",
      ),
    ))
    .c(
      p()
        .t("To request more torrents, or add your own, go ")
        .c(a().attr("href", REPO_URL))
        .t("here."),
    )
    .view(cx)
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
  component(|| HeadCommon(cx, HeadCommon_Props::builder().page_title("Index").build()))
}

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> IndexPageState {
  IndexPageState {
    search: "".to_string(),
    loading: false,
  }
}

pub fn get_template<G: Html>() -> Template<G> {
  Template::build("index")
    .view_with_state(index_page)
    .build_state_fn(get_build_state)
    .head(head)
    .build()
}
