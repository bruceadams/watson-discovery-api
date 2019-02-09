/* 
 * Discovery
 *
 * The IBM Watson&trade; Discovery Service is a cognitive search and content analytics engine that you can add to applications to identify patterns, trends and actionable insights to drive better decision-making. Securely unify structured and unstructured data with pre-enriched content, and use a simplified query language to eliminate the need for manual filtering of results.
 *
 * OpenAPI spec version: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;

use hyper;
use serde_json;
use futures::Future;

use super::{Error, configuration};
use super::request as __internal_request;

pub struct QueryModificationsApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> QueryModificationsApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> QueryModificationsApiClient<C> {
        QueryModificationsApiClient {
            configuration: configuration,
        }
    }
}

pub trait QueryModificationsApi {
    fn create_expansions(&self, environment_id: &str, collection_id: &str, version: String, expansions: ::models::Expansions) -> Box<Future<Item = ::models::Expansions, Error = Error<serde_json::Value>>>;
    fn create_stopword_list(&self, environment_id: &str, collection_id: &str, version: String, stopword_file: ::models::File) -> Box<Future<Item = ::models::TokenDictStatusResponse, Error = Error<serde_json::Value>>>;
    fn create_tokenization_dictionary(&self, environment_id: &str, collection_id: &str, version: String, token_dict: ::models::TokenDict) -> Box<Future<Item = ::models::TokenDictStatusResponse, Error = Error<serde_json::Value>>>;
    fn delete_expansions(&self, environment_id: &str, collection_id: &str, version: String) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn delete_stopword_list(&self, environment_id: &str, collection_id: &str, version: String) -> Box<Future<Item = serde_json::Value, Error = Error<serde_json::Value>>>;
    fn delete_tokenization_dictionary(&self, environment_id: &str, collection_id: &str, version: String) -> Box<Future<Item = serde_json::Value, Error = Error<serde_json::Value>>>;
    fn get_stopword_list_status(&self, environment_id: &str, collection_id: &str, version: String) -> Box<Future<Item = ::models::TokenDictStatusResponse, Error = Error<serde_json::Value>>>;
    fn get_tokenization_dictionary_status(&self, environment_id: &str, collection_id: &str, version: String) -> Box<Future<Item = ::models::TokenDictStatusResponse, Error = Error<serde_json::Value>>>;
    fn list_expansions(&self, environment_id: &str, collection_id: &str, version: String) -> Box<Future<Item = ::models::Expansions, Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>QueryModificationsApi for QueryModificationsApiClient<C> {
    fn create_expansions(&self, environment_id: &str, collection_id: &str, version: String, expansions: ::models::Expansions) -> Box<Future<Item = ::models::Expansions, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Post, "/v1/environments/{environment_id}/collections/{collection_id}/expansions".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_auth(__internal_request::Auth::Basic)
            .with_query_param("version".to_string(), version.to_string())
            .with_path_param("environment_id".to_string(), environment_id.to_string())
            .with_path_param("collection_id".to_string(), collection_id.to_string())
            .with_body_param(expansions)
            .execute(self.configuration.borrow())
    }

    fn create_stopword_list(&self, environment_id: &str, collection_id: &str, version: String, stopword_file: ::models::File) -> Box<Future<Item = ::models::TokenDictStatusResponse, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Post, "/v1/environments/{environment_id}/collections/{collection_id}/word_lists/stopwords".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_auth(__internal_request::Auth::Basic)
            .with_query_param("version".to_string(), version.to_string())
            .with_path_param("environment_id".to_string(), environment_id.to_string())
            .with_path_param("collection_id".to_string(), collection_id.to_string())
            .with_form_param("stopword_file".to_string(), unimplemented!())
            .execute(self.configuration.borrow())
    }

    fn create_tokenization_dictionary(&self, environment_id: &str, collection_id: &str, version: String, token_dict: ::models::TokenDict) -> Box<Future<Item = ::models::TokenDictStatusResponse, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Post, "/v1/environments/{environment_id}/collections/{collection_id}/word_lists/tokenization_dictionary".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_auth(__internal_request::Auth::Basic)
            .with_query_param("version".to_string(), version.to_string())
            .with_path_param("environment_id".to_string(), environment_id.to_string())
            .with_path_param("collection_id".to_string(), collection_id.to_string())
            .with_body_param(token_dict)
            .execute(self.configuration.borrow())
    }

    fn delete_expansions(&self, environment_id: &str, collection_id: &str, version: String) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Delete, "/v1/environments/{environment_id}/collections/{collection_id}/expansions".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_auth(__internal_request::Auth::Basic)
            .with_query_param("version".to_string(), version.to_string())
            .with_path_param("environment_id".to_string(), environment_id.to_string())
            .with_path_param("collection_id".to_string(), collection_id.to_string())
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

    fn delete_stopword_list(&self, environment_id: &str, collection_id: &str, version: String) -> Box<Future<Item = serde_json::Value, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Delete, "/v1/environments/{environment_id}/collections/{collection_id}/word_lists/stopwords".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_auth(__internal_request::Auth::Basic)
            .with_query_param("version".to_string(), version.to_string())
            .with_path_param("environment_id".to_string(), environment_id.to_string())
            .with_path_param("collection_id".to_string(), collection_id.to_string())
            .execute(self.configuration.borrow())
    }

    fn delete_tokenization_dictionary(&self, environment_id: &str, collection_id: &str, version: String) -> Box<Future<Item = serde_json::Value, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Delete, "/v1/environments/{environment_id}/collections/{collection_id}/word_lists/tokenization_dictionary".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_auth(__internal_request::Auth::Basic)
            .with_query_param("version".to_string(), version.to_string())
            .with_path_param("environment_id".to_string(), environment_id.to_string())
            .with_path_param("collection_id".to_string(), collection_id.to_string())
            .execute(self.configuration.borrow())
    }

    fn get_stopword_list_status(&self, environment_id: &str, collection_id: &str, version: String) -> Box<Future<Item = ::models::TokenDictStatusResponse, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/v1/environments/{environment_id}/collections/{collection_id}/word_lists/stopwords".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_auth(__internal_request::Auth::Basic)
            .with_query_param("version".to_string(), version.to_string())
            .with_path_param("environment_id".to_string(), environment_id.to_string())
            .with_path_param("collection_id".to_string(), collection_id.to_string())
            .execute(self.configuration.borrow())
    }

    fn get_tokenization_dictionary_status(&self, environment_id: &str, collection_id: &str, version: String) -> Box<Future<Item = ::models::TokenDictStatusResponse, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/v1/environments/{environment_id}/collections/{collection_id}/word_lists/tokenization_dictionary".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_auth(__internal_request::Auth::Basic)
            .with_query_param("version".to_string(), version.to_string())
            .with_path_param("environment_id".to_string(), environment_id.to_string())
            .with_path_param("collection_id".to_string(), collection_id.to_string())
            .execute(self.configuration.borrow())
    }

    fn list_expansions(&self, environment_id: &str, collection_id: &str, version: String) -> Box<Future<Item = ::models::Expansions, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/v1/environments/{environment_id}/collections/{collection_id}/expansions".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_auth(__internal_request::Auth::Basic)
            .with_query_param("version".to_string(), version.to_string())
            .with_path_param("environment_id".to_string(), environment_id.to_string())
            .with_path_param("collection_id".to_string(), collection_id.to_string())
            .execute(self.configuration.borrow())
    }

}