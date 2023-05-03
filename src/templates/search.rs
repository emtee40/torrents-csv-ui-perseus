use crate::{
  components::{
    head_common::{HeadCommon, HeadCommon_Props},
    icon::{IconAndText, IconAndText_Props},
    scaffold::{Scaffold, ScaffoldProps},
  },
  constants::DEFAULT_ENDPOINT,
  types::torrent::Torrent,
  utils::{magnet_link, pretty_date, pretty_num},
};
use human_bytes::human_bytes;
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::{builder::prelude::*, component::Prop, prelude::*};

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "SearchPageStateRx")]
struct SearchPageState {
  torrents: Vec<Torrent>,
}

#[auto_scope]
fn search_page<G: Html>(cx: Scope, state: &SearchPageStateRx) -> View<G> {
  component(|| {
    Scaffold(
      cx,
      ScaffoldProps::builder()
        .children(
          section()
            .class("section pt-1")
            .c(
              div()
                .class("container")
                .c(
                  div().class("block").c(
                    a().attr("href", "").c(IconAndText(
                      cx,
                      IconAndText_Props::builder()
                        .icon("arrow-left")
                        .text("Back".into())
                        .text_class(None)
                        .build(),
                    )),
                  ),
                )
                .dyn_if(
                  || state.torrents.get().is_empty(),
                  || p().class("block").t("No results"),
                  move || {
                    component(|| {
                      TorrentCard(
                        cx,
                        TorrentCard_Props::builder()
                          .torrents(&state.torrents)
                          .build(),
                      )
                    })
                  },
                ),
            )
            .view(cx)
            .into(),
        )
        .build(),
    )
  })
}

#[component(inline_props)]
fn TorrentCard<'a, G: Html>(cx: Scope<'a>, torrents: &'a ReadSignal<Vec<Torrent>>) -> View<G> {
  div()
    .c(Keyed(
      cx,
      KeyedProps::builder()
        .iterable(torrents)
        .key(|x| x.infohash.clone())
        .view(|cx, torrent| {
          let magnet = magnet_link(&torrent);
          div()
            .class("box mb-2")
            .c(
              div()
                .c(
                  a()
                    .class("title is-6")
                    .dyn_attr("href", move || Some(magnet.clone()))
                    .dyn_t(move || torrent.name.clone()),
                )
                .c(
                  div()
                    .class("mt-1 is-flex is-justify-content-space-between")
                    .c(div().c(component(|| {
                      IconAndText(
                        cx,
                        IconAndText_Props::builder()
                          .icon("upload")
                          .text(pretty_num(torrent.seeders))
                          .text_class(Some("has-text-primary"))
                          .build(),
                      )
                    })))
                    .c(div().c(component(|| {
                      IconAndText(
                        cx,
                        IconAndText_Props::builder()
                          .icon("database")
                          .text(human_bytes(torrent.size_bytes as f64))
                          .text_class(None)
                          .build(),
                      )
                    })))
                    .c(div().c(component(|| {
                      IconAndText(
                        cx,
                        IconAndText_Props::builder()
                          .icon("calendar-o")
                          .text(pretty_date(torrent.created_unix))
                          .text_class(None)
                          .build(),
                      )
                    }))),
                ),
            )
            .view(cx)
        })
        .build(),
    ))
    .view(cx)
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
  component(|| {
    HeadCommon(
      cx,
      HeadCommon_Props::builder()
        .page_title("Search | Torrents-csv")
        .build(),
    )
  })
}

#[engine_only_fn]
async fn get_build_paths() -> BuildPaths {
  BuildPaths {
    paths: Vec::new(),
    extra: ().into(),
  }
}

#[engine_only_fn]
async fn get_build_state(
  info: StateGeneratorInfo<()>,
) -> Result<SearchPageState, BlamedError<reqwest::Error>> {
  let torrents = perseus::utils::cache_fallible_res(
    &info.path,
    || async {
      let endpoint = std::env::var("TORRENTS_CSV_ENDPOINT").unwrap_or(DEFAULT_ENDPOINT.to_string());
      let query = format!("{endpoint}/service/search?q={}", &info.path);
      let res = reqwest::get(query).await?.json().await?;
      Ok::<Vec<Torrent>, reqwest::Error>(res)
    },
    false,
  )
  .await?;

  Ok(SearchPageState { torrents })
}

pub fn get_template<G: Html>() -> Template<G> {
  Template::build("search")
    .head(head)
    .view_with_state(search_page)
    .build_state_fn(get_build_state)
    .build_paths_fn(get_build_paths)
    .incremental_generation()
    .build()
}
