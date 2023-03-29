pub mod share;
pub mod login;
pub mod fetch;

pub const BACKEND_URL: &str = match option_env!("BACKEND_URL") {
    Some(v) => v,
    None => {
        #[cfg(debug_assertions)]
        const DEFAULT: &str = "http://localhost:3000/api";

        #[cfg(not(debug_assertions))]
        const DEFAULT: &str = "https://api.play.yew.rs/api";

        DEFAULT
    }
};
