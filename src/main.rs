use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod app;
pub mod utils;

use app::App;

pub fn main() {
    _ = console_error_panic_hook::set_once();
    
    mount_to_body(|| {
        view! {
            <MetaTags/>
            <App/>
        }
    });
}