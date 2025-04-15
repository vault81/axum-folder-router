use axum_folder_router::folder_router;

#[derive(Clone)]
struct AppState;

#[folder_router("examples/simple/api", AppState)]
struct MyFolderRouter();
