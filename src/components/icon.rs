use perseus::prelude::*;
use sycamore::{builder::prelude::*, component::Prop, prelude::*};

#[component(inline_props)]
pub fn IconAndText<'a, G: Html>(
  cx: Scope<'a>,
  text: String,
  icon: &'a str,
  text_class: Option<&'a str>,
) -> View<G> {
  span()
    .class("icon-text")
    .c(component(|| {
      Icon(cx, Icon_Props::builder().icon(icon).build())
    }))
    .c(
      span()
        .class(text_class.unwrap_or(""))
        .dyn_t(move || text.clone()),
    )
    .view(cx)
}

#[component(inline_props)]
pub fn Icon<'a, G: Html>(cx: Scope<'a>, icon: &'a str) -> View<G> {
  span()
    .class("icon")
    .c(
      i()
        .class(format!("fa fa-{icon}"))
        .bool_attr("aria-hidden", true),
    )
    .view(cx)
}
