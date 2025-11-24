use axum::{Router};
use tower_http::services::ServeDir;

// Este servicio solo se encarga de devolver el formulario, tiene que ser muy ligero, las funciones para permitir el paso seran en nh-auth
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let static_files = ServeDir::new("public");

    let router = Router::new()
        .fallback_service(static_files);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Servidor en http://0.0.0.0:3000");
    axum::serve(listener, router).await.unwrap();
}
