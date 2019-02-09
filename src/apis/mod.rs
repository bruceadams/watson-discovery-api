use hyper;
use serde;
use serde_json;

#[derive(Debug)]
pub enum Error<T> {
    UriError(hyper::error::UriError),
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    ApiError(ApiError<T>),
}

#[derive(Debug)]
pub struct ApiError<T> {
    pub code: hyper::StatusCode,
    pub content: Option<T>,
}

impl<'de, T> From<(hyper::StatusCode, &'de [u8])> for Error<T> 
    where T: serde::Deserialize<'de> {
    fn from(e: (hyper::StatusCode, &'de [u8])) -> Self {
        if e.1.len() == 0 {
            return Error::ApiError(ApiError{
                code: e.0,
                content: None,
            });
        }
        match serde_json::from_slice::<T>(e.1) {
            Ok(t) => Error::ApiError(ApiError{
                code: e.0,
                content: Some(t),
            }),
            Err(e) => {
                Error::from(e)
            }
        }
    }
}

impl<T> From<hyper::Error> for Error<T> {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

use super::models::*;

mod request;

mod collections_api;
pub use self::collections_api::{ CollectionsApi, CollectionsApiClient };
mod configurations_api;
pub use self::configurations_api::{ ConfigurationsApi, ConfigurationsApiClient };
mod credentials_api;
pub use self::credentials_api::{ CredentialsApi, CredentialsApiClient };
mod documents_api;
pub use self::documents_api::{ DocumentsApi, DocumentsApiClient };
mod environments_api;
pub use self::environments_api::{ EnvironmentsApi, EnvironmentsApiClient };
mod events_and_feedback_api;
pub use self::events_and_feedback_api::{ EventsAndFeedbackApi, EventsAndFeedbackApiClient };
mod gateway_configuration_api;
pub use self::gateway_configuration_api::{ GatewayConfigurationApi, GatewayConfigurationApiClient };
mod queries_api;
pub use self::queries_api::{ QueriesApi, QueriesApiClient };
mod query_modifications_api;
pub use self::query_modifications_api::{ QueryModificationsApi, QueryModificationsApiClient };
mod test_your_configuration_on_a_document_api;
pub use self::test_your_configuration_on_a_document_api::{ TestYourConfigurationOnADocumentApi, TestYourConfigurationOnADocumentApiClient };
mod training_data_api;
pub use self::training_data_api::{ TrainingDataApi, TrainingDataApiClient };
mod user_data_api;
pub use self::user_data_api::{ UserDataApi, UserDataApiClient };

pub mod configuration;
pub mod client;
