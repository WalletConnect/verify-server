use {hyper::StatusCode, std::future};

pub fn get(
    provider: impl Fn() -> String + Clone + Send + 'static,
) -> impl Fn() -> future::Ready<(StatusCode, String)> + Clone + Send + 'static {
    move || future::ready((StatusCode::OK, provider()))
}
