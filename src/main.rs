use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /hello -> returns a simple hello message
    let hello = warp::path("hello").map(|| warp::reply::html("<h1>Hello, World!</h1>"));

    // POST /submit -> receives form submission
    let submit = warp::path("submit").and(warp::body::form()).map(
        |form: std::collections::HashMap<String, String>| {
            let name = form.get("name").unwrap_or(&"".to_string()).clone();
            warp::reply::html(format!("Hello, {}!", name))
        },
    );

    let routes = warp::get().and(hello).or(warp::post().and(submit));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
