// use warp::fs::File;
use warp::Filter;

#[tokio::main]
async fn main() {
    // Serve the index.html file
    let index = warp::path::end().and(warp::fs::file("./static/index.html"));

    // POST /submit -> receives form submission
    let submit = warp::path("submit").and(warp::body::form()).map(
        |form: std::collections::HashMap<String, String>| {
            let name = form.get("name").unwrap_or(&"".to_string()).clone();
            warp::reply::html(format!("Hello, {}!", name))
        },
    );

    let routes = warp::get().and(index).or(warp::post().and(submit));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
