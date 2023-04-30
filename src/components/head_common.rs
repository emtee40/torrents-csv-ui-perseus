use perseus::prelude::*;
use sycamore::{builder::prelude::*, prelude::*};

#[component(inline_props)]
pub fn HeadCommon<'a, G: Html>(cx: Scope<'a>, page_title: &'a str) -> View<G> {
  fragment([
    meta()
      .attr("name", "viewport")
      .attr("content", "width=device-width, initial-scale=1")
      .view(cx),
    title().t(page_title).view(cx),
    link()
      .attr("rel", "stylesheet")
      .attr("type", "text/css")
      .attr(
        "href",
        "https://cdn.jsdelivr.net/npm/bulma@0.9.4/css/bulma.min.css",
      )
      .view(cx),
    link()
      .attr("rel", "stylesheet")
      .attr("type", "text/css")
      .attr("href", "https://unpkg.com/bulma-prefers-dark")
      .view(cx),
  ])
}
