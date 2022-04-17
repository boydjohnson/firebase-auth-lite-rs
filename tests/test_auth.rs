use firebase_auth_lite::{Auth, AuthOptions, OauthFlowOptions};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test(async)]
async fn test_auth_sign_out() {
    let auth = Auth::new(AuthOptions::new("not-an-api-key"));

    assert!(auth.sign_out().await.is_ok());
}

#[wasm_bindgen_test(async)]
async fn test_auth_sign_in_with_custom_token() {
    let auth = Auth::new(AuthOptions::new("not-an-api-key"));

    assert!(auth
        .sign_in_with_custom_token("not-a-custom-token".into())
        .await
        .is_err());
}

#[wasm_bindgen_test(async)]
async fn test_auth_sign_in_with_provider() {
    let auth = Auth::new(AuthOptions::new("not-an-api-key"));

    assert!(auth
        .sign_in_with_provider(OauthFlowOptions::new("".into(), "".into(), true))
        .await
        .is_err());
}

#[wasm_bindgen_test(async)]
async fn test_auth_handle_sign_in_redirect() {
    let auth = Auth::new(AuthOptions::new("not-an-api-key"));

    assert!(auth.handle_sign_in_redirect().await.is_ok());
}

#[wasm_bindgen_test(async)]
async fn test_auth_sign_up() {
    let auth = Auth::new(AuthOptions::new("not-an-api-key"));

    assert!(auth
        .signUp("example@example.com".into(), "example".into())
        .await
        .is_err());
}

#[wasm_bindgen_test(async)]
async fn test_auth_sign_in() {
    let auth = Auth::new(AuthOptions::new("not-an-api-key"));

    assert!(auth
        .sign_in("example@example.com".into(), "example".into())
        .await
        .is_err());
}

#[wasm_bindgen_test(async)]
async fn test_auth_send_oob_code() {
    let auth = Auth::new(AuthOptions::new("not-an-api-key"));

    assert!(auth
        .send_oob_code("EMAIL_SIGNIN".into(), "example@example.com".into())
        .await
        .is_ok());
}

#[wasm_bindgen_test(async)]
async fn test_auth_reset_password() {
    let auth = Auth::new(AuthOptions::new("not-an-api-key"));

    assert!(auth
        .reset_password("fake-oob-code".into(), "example".into())
        .await
        .is_err());
}

#[wasm_bindgen_test(async)]
async fn test_auth_fetch_providers_for_email() {
    let auth = Auth::new(AuthOptions::new("not-an-api-key"));

    assert!(auth
        .fetch_providers_for_email("example@example.com".into())
        .await
        .is_err());
}

#[wasm_bindgen_test(async)]
async fn test_auth_fetch_profile() {
    let auth = Auth::new(AuthOptions::new("not-an-api-key"));

    assert!(auth.fetch_profile().await.is_err());
}

#[wasm_bindgen_test(async)]
async fn test_auth_update_profile() {
    let auth = Auth::new(AuthOptions::new("not-an-api-key"));

    assert!(auth
        .update_profile(js_sys::Object::default())
        .await
        .is_err());
}

#[wasm_bindgen_test(async)]
async fn test_auth_delete_account() {
    let auth = Auth::new(AuthOptions::new("not-an-api-key"));

    assert!(auth.delete_account().await.is_err());
}
