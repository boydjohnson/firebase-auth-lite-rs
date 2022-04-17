use crate::{Route, API_KEY};
use firebase_auth_lite::{Auth, AuthOptions};
use yew::prelude::*;
use yew_router::{history::History, scope_ext::RouterScopeExt};

pub struct Login {
    email: Option<String>,
    password: Option<String>,
    login_in_progress: bool,
    error: Option<String>,
}

pub enum LoginMsg {
    EmailInput(String),
    PasswordInput(String),
    ChangeToHome,
    Error(String),
    Login,
    Nil,
}

impl Component for Login {
    type Message = LoginMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Login {
            email: None,
            password: None,
            login_in_progress: false,
            error: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: LoginMsg) -> bool {
        match msg {
            LoginMsg::Error(error) => {
                self.login_in_progress = false;
                self.error = Some(error);
                true
            }
            LoginMsg::EmailInput(email) => {
                if let Some(e) = &mut self.email {
                    e.push_str(&email);
                } else {
                    self.email = Some(email);
                }
                false
            }
            LoginMsg::PasswordInput(password) => {
                if let Some(e) = &mut self.password {
                    e.push_str(&password);
                } else {
                    self.password = Some(password);
                }
                false
            }
            LoginMsg::Login => {
                let auth = Auth::new(AuthOptions::new(API_KEY));
                if let (Some(email), Some(password)) = (&self.email, &self.password) {
                    let email = email.clone();
                    let password = password.clone();
                    ctx.link().send_future(async move {
                        match auth.sign_in(email, password).await {
                            Ok(_) => LoginMsg::ChangeToHome,
                            Err(obj) => {
                                let e = js_sys::Error::from(obj);
                                match e.message().as_string() {
                                    Some(v) => {
                                        if &v == "EMAIL_NOT_FOUND" {
                                            LoginMsg::Error("User not found".into())
                                        } else if &v == "INVALID_PASSWORD" {
                                            LoginMsg::Error("Wrong Password".into())
                                        } else if &v == "USER_DISABLED" {
                                            LoginMsg::Error(
                                                "User has been disabled by an administrator".into(),
                                            )
                                        } else {
                                            LoginMsg::Error("There has been an error.".into())
                                        }
                                    }
                                    None => LoginMsg::Error("There has been an error.".into()),
                                }
                            }
                        }
                    });

                    self.login_in_progress = true;

                    true
                } else {
                    false
                }
            }
            LoginMsg::ChangeToHome => {
                if let Some(history) = ctx.link().history() {
                    history.push(Route::Home);
                }
                true
            }
            LoginMsg::Nil => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_click_signup = ctx.link().callback(|_| LoginMsg::Login);

        if self.login_in_progress {
            html! {
                <p> { "Working on login" }</p>
            }
        } else if let Some(error) = &self.error {
            html! {
                <>
                { self.display_email_password_input(ctx) }
                <p style="color:red;">{ error }</p>

                <button onclick = {on_click_signup}>{ "Log In" }</button>

                </>
            }
        } else {
            html! {
                <>
                { self.display_email_password_input(ctx) }
                <button onclick = {on_click_signup}>{ "Log In" }</button>
                </>
            }
        }
    }
}

impl Login {
    fn display_email_password_input(&self, ctx: &Context<Self>) -> Html {
        let on_input_email = ctx.link().callback(|evt: InputEvent| {
            if let Some(email) = evt.data() {
                LoginMsg::EmailInput(email)
            } else {
                LoginMsg::Nil
            }
        });

        let on_input_password = ctx.link().callback(|evt: InputEvent| {
            if let Some(password) = evt.data() {
                LoginMsg::PasswordInput(password)
            } else {
                LoginMsg::Nil
            }
        });

        html! {
            <>
            <div>
                <label>{ "Email" }
                    <input type = "email" oninput = {on_input_email} name="email" />
                </label>
            </div>
            <div>
                <label>{ "Password" }
                    <input type = "password" oninput = {on_input_password} name="password" />
                </label>
            </div>

            </>
        }
    }
}
