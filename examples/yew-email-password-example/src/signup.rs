use crate::{Route, API_KEY};
use firebase_auth_lite::{Auth, AuthOptions};
use yew::{events::InputEvent, prelude::*};
use yew_router::{history::History, scope_ext::RouterScopeExt};

pub struct Signup {
    email: Option<String>,
    password: Option<String>,
    signup_in_progress: bool,
    error: Option<String>,
}

pub enum SignupMsg {
    EmailInput(String),
    PasswordInput(String),
    ChangeToHome,
    Signup,
    Error(String),
    Nil,
}

impl Component for Signup {
    type Message = SignupMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Signup {
            email: None,
            password: None,
            signup_in_progress: false,
            error: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: SignupMsg) -> bool {
        match msg {
            SignupMsg::Error(error) => {
                self.signup_in_progress = false;
                self.error = Some(error);
                true
            }
            SignupMsg::EmailInput(email) => {
                if let Some(e) = &mut self.email {
                    e.push_str(&email);
                } else {
                    self.email = Some(email);
                }
                false
            }
            SignupMsg::PasswordInput(password) => {
                if let Some(e) = &mut self.password {
                    e.push_str(&password);
                } else {
                    self.password = Some(password);
                }
                false
            }
            SignupMsg::Signup => {
                let auth = Auth::new(AuthOptions::new(API_KEY));
                if let (Some(email), Some(password)) = (&self.email, &self.password) {
                    let email = email.clone();
                    let password = password.clone();
                    ctx.link().send_future(async move {
                        match auth.signUp(email, password).await {
                            Ok(_) => SignupMsg::ChangeToHome,
                            Err(e) => {
                                let v = js_sys::Error::from(e);
                                match v.message().as_string() {
                                    Some(v) => {
                                        if &v == "EMAIL_EXISTS" {
                                            SignupMsg::Error("Email Already Exists".into())
                                        } else {
                                            SignupMsg::Error("An Error Occured".into())
                                        }
                                    }
                                    None => SignupMsg::Error("Something went wrong".into()),
                                }
                            }
                        }
                    });

                    self.signup_in_progress = true;

                    true
                } else {
                    false
                }
            }
            SignupMsg::ChangeToHome => {
                if let Some(history) = ctx.link().history() {
                    history.push(Route::Home);
                }
                true
            }
            SignupMsg::Nil => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_click_signup = ctx.link().callback(|_| SignupMsg::Signup);

        if self.signup_in_progress {
            html! {
                <p> { "Working on sign up" }</p>
            }
        } else if let Some(error) = &self.error {
            html! {
                <>
                { self.display_email_password_input(ctx) }
                <p style="color:red;"> { error }</p>
                <button onclick = {on_click_signup}>{ "Signup" }</button>
                </>
            }
        } else {
            html! {
                <>
                { self.display_email_password_input(ctx) }
                <button onclick = {on_click_signup}>{ "Signup" }</button>
                </>
            }
        }
    }
}

impl Signup {
    fn display_email_password_input(&self, ctx: &Context<Self>) -> Html {
        let on_input_email = ctx.link().callback(|evt: InputEvent| {
            if let Some(email) = evt.data() {
                SignupMsg::EmailInput(email)
            } else {
                SignupMsg::Nil
            }
        });

        let on_input_password = ctx.link().callback(|evt: InputEvent| {
            if let Some(password) = evt.data() {
                SignupMsg::PasswordInput(password)
            } else {
                SignupMsg::Nil
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
