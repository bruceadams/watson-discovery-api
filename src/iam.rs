use failure::Error;
use futures;
use hyper;
use hyper::rt::{Future, Stream};

use hyper::{header, Body, Client, Request, Uri};
use hyper_rustls::HttpsConnector;
use serde_json;
use serde_urlencoded;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tokens {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub expiration: u64,
    pub scope: String,
}

const GRANT_TYPE: &str = "urn:ibm:params:oauth:grant-type:apikey";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct IamRequest {
    apikey: String,
    grant_type: String,
}

pub fn get_tokens(identity_url: &str, apikey: &str) -> Box<Future<Item = Tokens, Error = Error>> {
    let https = HttpsConnector::new(4);
    let client = Client::builder().build::<_, hyper::Body>(https);

    let uri: Uri = match identity_url.parse() {
        Ok(uri) => uri,
        Err(err) => return Box::new(futures::done(Err(Error::from(err)))),
    };

    let body = IamRequest {
        apikey: apikey.to_string(),
        grant_type: GRANT_TYPE.to_string(),
    };

    let body = serde_urlencoded::to_string(body).unwrap();
    println!("body {:#?}", body);

    let request = Request::post(uri)
        .header(header::ACCEPT, "application/json")
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(Body::from(body));

    let request = match request {
        Ok(req) => req,
        Err(err) => return Box::new(futures::done(Err(err.into()))),
    };
    println!("request {:#?}", request);

    let s1 = client.request(request);
    let s2 = s1.map_err(|err| Error::from(err)).and_then(|res| {
        println!("POST: {}", res.status());

        res.into_body().concat2().map_err(|err| err.into())
    });

    let s3 = s2.and_then(|body| serde_json::from_slice(&body).map_err(|err| err.into()));

    Box::new(s3)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_core::reactor;

    #[test]
    /// Serializing a struct to urlencoded isn't documented. Make sure it works!
    fn serialize_iam_request() {
        let iam = IamRequest {
            apikey: "m?m".to_string(),
            grant_type: "1<2".to_string(),
        };
        assert_eq!(
            serde_urlencoded::to_string(iam),
            Ok("apikey=m%3Fm&grant_type=1%3C2".to_string())
        );
    }
}
