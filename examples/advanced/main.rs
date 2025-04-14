use axum::Router;
use axum_folder_router::folder_router;

#[derive(Clone, Debug)]
struct AppState {
    _foo: String,
}

folder_router!("examples/advanced/api", AppState);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create app state
    let app_state = AppState {
        _foo: "".to_string(),
    };

    // Generate the router using the macro
    let folder_router: Router<AppState> = folder_router();

    // Build the router and provide the state
    let app: Router<()> = folder_router.with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Listening on http://{}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}
