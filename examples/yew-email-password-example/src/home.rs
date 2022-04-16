use crate::{Route, API_KEY};
use firebase_auth_lite::{Auth, AuthOptions, UserProfile};
use yew::prelude::*;
use yew_router::components::Link;

pub struct Home {
    user: Option<String>,
}

pub enum HomeMsg {
    NotLoggedIn,
    LoggedIn(String),
}

impl Component for Home {
    type Message = HomeMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let auth = Auth::new(AuthOptions::new(API_KEY));

        ctx.link().send_future(async move {
            if let Ok(prof) = auth.fetch_profile().await {
                if let Ok(prof) = prof.into_serde() {
                    let user_profile: UserProfile = prof;
                    return HomeMsg::LoggedIn(user_profile.email);
                }
            }
            HomeMsg::NotLoggedIn
        });

        Home { user: None }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            HomeMsg::NotLoggedIn => {
                self.user = None;
            }
            HomeMsg::LoggedIn(user) => {
                self.user = Some(user);
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        if let Some(user) = &self.user {
            html! {
                <h1> { format!("Hello, {}", user) }</h1>
            }
        } else {
            html! {
                <>
                <h1>{ "Please Sign up or Log in" }</h1>
                <span><Link<Route> to = { Route::Signup }>{ "Sign Up" }</Link<Route>></span>
                <span><Link<Route> to = { Route::Login }>{ "Log In" }</Link<Route>></span>
                </>
            }
        }
    }
}
