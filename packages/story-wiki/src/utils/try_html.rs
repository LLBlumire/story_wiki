#[macro_export]
macro_rules! try_html {
    ($($t:tt)*) => {
        if let Some(__internal_bind) = { $($t)* } {
            __internal_bind
        } else {
            log::trace!("Try HTML None");
            return html! { }
        }
    }
}
