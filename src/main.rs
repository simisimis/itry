use axum::{
    routing::get,
    Router,
};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // router
    let app = Router::new()
    .route("/hello", get(hello))
    .route("/hello/:name", get(get_name));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
async fn hello() -> String {
    format!("hello world")
}

async fn get_name(
        axum::extract::Path(name):
        axum::extract::Path<String>
) -> String {
    format!("hello {}", name)
}

