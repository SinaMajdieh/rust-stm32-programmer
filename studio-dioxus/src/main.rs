mod app;
mod components;
mod home;
mod new_project;
mod routes;

fn main() {
    dioxus::launch(app::App);
}
