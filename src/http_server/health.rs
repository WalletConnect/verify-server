use {hyper::StatusCode, std::future};

pub fn get(
    resp: String,
) -> impl Fn() -> future::Ready<(StatusCode, String)> + Clone + Send + 'static {
    move || future::ready((StatusCode::OK, resp.clone()))
}
