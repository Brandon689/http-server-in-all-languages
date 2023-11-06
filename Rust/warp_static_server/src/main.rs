use warp::Filter;

#[tokio::main]
async fn main() {
    // Serve static files from the "static" directory.
    let static_files = warp::path("static").and(warp::fs::dir("./static"));

    // Custom filter to serve HTML files with a .html extension directly under root path.
    let html_files_no_prefix = warp::path::param::<String>()
        .and_then(|file_name: String| async move {
            let file_path = format!("./html/{}.html", file_name);
            Ok::<_, warp::reject::Rejection>(warp::reply::html(std::fs::read_to_string(file_path).unwrap()))
        });

    // Combine all the routes.
    let routes = static_files.or(html_files_no_prefix);

    // Start the warp server on the specified address and port.
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
