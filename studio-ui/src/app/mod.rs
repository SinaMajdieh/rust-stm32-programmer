mod application;
mod navigation;
mod projects;

pub use application::Application;

#[macro_export]
macro_rules! todo_log {
    ($($arg:tt)*) => {
        println!("[TODO] {}", format_args!($($arg)*))
    };
}
