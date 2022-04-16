use yew::prelude::*;
use yew_router::prelude::*;

pub mod home;
pub mod signup;

pub const API_KEY: &str = include_str!("../API_KEY.txt");

#[derive(Debug, Clone, Copy, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login/")]
    Login,
    #[at("/signup/")]
    Signup,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <home::Home /> },
        Route::Login => todo!(),
        Route::Signup => html! { <signup::Signup /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}
