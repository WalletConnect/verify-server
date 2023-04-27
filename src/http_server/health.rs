use {axum::response::Response, std::future};

pub fn get(
    provider: impl Fn() -> Response + Clone + Send + 'static,
) -> impl Fn() -> future::Ready<Response> + Clone + Send + 'static {
    move || future::ready(provider())
}
