use axum_folder_router::folder_router;

#[derive(Clone)]
struct AppState {
    _foo: String,
}

#[folder_router("examples/advanced/api", AppState)]
struct MyFolderRouter();
