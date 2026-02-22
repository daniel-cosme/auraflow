use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod routes;
pub mod components;

use crate::create_app_state;
use routes::Routes;

#[component]
pub fn App() -> impl IntoView {
    let app_state = create_app_state();
    
    provide_context(app_state);
    
    view! {
        <Stylesheet id="tailwind" href="/pkg/odontologia-spa.css"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1"/>
        
        <Router>
            <Routes/>
        </Router>
    }
}