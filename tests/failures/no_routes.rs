use axum_folder_router::folder_router;

#[derive(Clone)]
struct AppState;

#[folder_router("../../../../tests/failures/no_routes", AppState)]
struct MyFolderRouter();

fn main() {}
