use perseus::{errors::ClientError, prelude::*};
use sycamore::builder::prelude::*;

pub fn get_error_views<G: Html>() -> ErrorViews<G> {
  ErrorViews::new(|cx, err, _err_info, _err_pos| {
    match err {
      ClientError::ServerError { status, message: _ } => match status {
        404 => (
          title().t("Page not found").view(cx),
          p().t("Sorry, that page doesn't seem to exist.").view(cx),
        ),
        // 4xx is a client error
        _ if (400..500).contains(&status) => (
          title().t("Error").view(cx),
          p()
            .t("There was something wrong with the last request, please try reloading the page.")
            .view(cx),
        ),
        // 5xx is a server error
        _ => (
          title().t("Error").view(cx),
          p()
            .t("Sorry, our server experienced an internal error. Please try reloading the page.")
            .view(cx),
        ),
      },
      ClientError::Panic(_) => (
            title().t("Critical error").view(cx),
            p().t("Sorry, but a critical internal error has occurred. This has been automatically reported to our team, who'll get on it as soon as possible. In the mean time, please try reloading the page.").view(cx)
      ),
      ClientError::FetchError(_) => (
          title().t("Error").view(cx),
          p().t("A network error occurred, do you have an internet connection? (If you do, try reloading the page.)").view(cx)
      ),
      _ => (
        title().t("Error").view(cx),
        p().dyn_t(move || format!("An internal error has occurred: '{err}'.")).view(cx)
      ),
    }
  })
}
