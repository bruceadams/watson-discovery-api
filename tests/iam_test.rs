extern crate tokio_core;
extern crate watson_discovery_api;

use tokio_core::reactor;
use watson_discovery_api::iam;

#[test]
/// Actually get tokens from the IAM service
fn iam_token_integration() {
    let mut core = reactor::Core::new().unwrap();

    let url = match std::env::var("WATSON_DISCOVERY_API_IDENTITY_URL") {
        Ok(url) => url,
        Err(_err) => "https://iam.bluemix.net/identity/token".to_string(),
    };
    let apikey = match std::env::var("WATSON_DISCOVERY_API_APIKEY") {
        Ok(apikey) => apikey,
        Err(_err) => panic!("Environment variable WATSON_DISCOVERY_API_APIKEY not found"),
    };
    let tokens = core.run(iam::get_tokens(&url, &apikey)).unwrap();
    assert_eq!(tokens.token_type, "Bearer");
}
