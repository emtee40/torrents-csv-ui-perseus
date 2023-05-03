use crate::{
  components::{
    head_common::{HeadCommon, HeadCommon_Props},
    scaffold::{Scaffold, ScaffoldProps, SearchField, SearchField_Props},
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
  // On reload, set loading to false
  state.loading.set(false);

  component(|| {
    Scaffold(
      cx,
      ScaffoldProps::builder()
        .show_button(false)
        .search(Some(&state.search))
        .loading(Some(&state.loading))
        .children(
          section()
            .class("section pt-1")
            .c(
              div()
                .class("container")
                .c(component(|| {
                  SearchField(
                    cx,
                    SearchField_Props::builder()
                      .search(&state.search)
                      .loading(&state.loading)
                      .show_button(true)
                      .is_expanded(true)
                      .build(),
                  )
                }))
                .c(component(|| IntroText(cx))),
            )
            .view(cx)
            .into(),
        )
        .build(),
    )
  })
}

#[component]
fn IntroText<G: Html>(cx: Scope) -> View<G> {
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
  component(|| {
    HeadCommon(
      cx,
      HeadCommon_Props::builder()
        .page_title("Torrents-csv")
        .build(),
    )
  })
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
