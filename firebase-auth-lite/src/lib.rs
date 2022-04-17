use serde::Deserialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/main.js")]
extern "C" {
    pub type Auth;

    #[wasm_bindgen(constructor)]
    pub fn new(opts: AuthOptions) -> Auth;

    #[wasm_bindgen(method, js_name = "signOut", catch)]
    pub async fn sign_out(this: &Auth) -> Result<(), JsValue>;

    #[wasm_bindgen(method, js_name = "signInWithCustomToken", catch)]
    pub async fn sign_in_with_custom_token(this: &Auth, token: String) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, js_name = "signInWithProvider", catch)]
    pub async fn sign_in_with_provider(
        this: &Auth,
        options: OauthFlowOptions,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(method, js_name = "handleSignInRedirect", catch)]
    pub async fn handle_sign_in_redirect(this: &Auth) -> Result<(), JsValue>;

    #[wasm_bindgen(method, js_name = "signUp", catch)]
    pub async fn signUp(this: &Auth, email: String, password: String) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, js_name = "signIn", catch)]
    pub async fn sign_in(this: &Auth, email: String, password: String) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, js_name = "sendOobCode", catch)]
    pub async fn send_oob_code(
        this: &Auth,
        request_type: String,
        email: String,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(method, js_name = "resetPassword", catch)]
    pub async fn reset_password(
        this: &Auth,
        oob_code: String,
        new_password: String,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, js_name = "fetchProvidersForEmail", catch)]
    pub async fn fetch_providers_for_email(this: &Auth, email: String) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, js_name = "fetchProfile", catch)]
    pub async fn fetch_profile(this: &Auth) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, js_name = "updateProfile", catch)]
    pub async fn update_profile(this: &Auth, new_data: js_sys::Object) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, js_name = "deleteAccount", catch)]
    pub async fn delete_account(this: &Auth) -> Result<(), JsValue>;

}

#[derive(Debug, Deserialize)]
pub struct UserProfile {
    #[serde(rename = "localId")]
    pub local_id: String,
    pub email: String,
    #[serde(rename = "passwordHash")]
    pub password_hash: String,
    #[serde(rename = "emailVerified")]
    pub email_verified: bool,
    #[serde(rename = "passwordUpdatedAt")]
    pub password_updated_at: u64,
    #[serde(rename = "validSince")]
    pub valid_since: String,
    #[serde(rename = "lastLoginAt")]
    pub last_login_at: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "lastRefreshAt")]
    pub last_refresh_at: String,
}

#[wasm_bindgen]
pub struct AuthOptions {
    api_key: String,
    redirect_uri: String,
}

#[wasm_bindgen]
impl AuthOptions {
    #[wasm_bindgen(getter, js_name = "apiKey")]
    pub fn api_key(&self) -> String {
        self.api_key.clone()
    }

    #[wasm_bindgen(getter, js_name = "redirectUri")]
    pub fn redirect_uri(&self) -> String {
        self.redirect_uri.clone()
    }
}

impl AuthOptions {
    pub fn new(api_key: &str, redirect_uri: String) -> Self {
        AuthOptions {
            api_key: api_key.into(),
            redirect_uri,
        }
    }
}

#[wasm_bindgen]
pub struct ProviderOptions {
    name: String,
    scope: Option<String>,
}

#[wasm_bindgen]
impl ProviderOptions {
    pub fn new(name: String) -> Self {
        ProviderOptions { name, scope: None }
    }

    pub fn with_scope(mut self, scope: String) -> Self {
        self.scope = Some(scope);
        self
    }

    pub fn set_scope(&mut self, scope: String) {
        self.scope = Some(scope);
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn scope(&self) -> Option<String> {
        self.scope.clone()
    }
}

#[wasm_bindgen]
pub struct OauthFlowOptions {
    provider: String,
    context: String,
    #[wasm_bindgen(js_name = "linkAccount")]
    pub link_account: bool,
}

#[wasm_bindgen]
impl OauthFlowOptions {
    pub fn new(provider: String, context: String, link_account: bool) -> Self {
        OauthFlowOptions {
            provider,
            context,
            link_account,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn provider(&self) -> String {
        self.provider.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn context(&self) -> String {
        self.context.clone()
    }
}
