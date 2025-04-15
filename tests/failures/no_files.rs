use axum_folder_router::folder_router;

#[derive(Clone)]
struct AppState;

#[folder_router("some/non/existing/directory", AppState)]
struct MyFolderRouter();

fn main() {}
