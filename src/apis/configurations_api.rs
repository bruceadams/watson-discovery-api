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

pub struct ConfigurationsApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> ConfigurationsApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> ConfigurationsApiClient<C> {
        ConfigurationsApiClient {
            configuration: configuration,
        }
    }
}

pub trait ConfigurationsApi {
    fn create_configuration(&self, environment_id: &str, version: String, configuration: ::models::Configuration) -> Box<Future<Item = ::models::Configuration, Error = Error<serde_json::Value>>>;
    fn delete_configuration(&self, environment_id: &str, configuration_id: &str, version: String) -> Box<Future<Item = ::models::DeleteConfigurationResponse, Error = Error<serde_json::Value>>>;
    fn get_configuration(&self, environment_id: &str, configuration_id: &str, version: String) -> Box<Future<Item = ::models::Configuration, Error = Error<serde_json::Value>>>;
    fn list_configurations(&self, environment_id: &str, version: String, name: &str) -> Box<Future<Item = ::models::ListConfigurationsResponse, Error = Error<serde_json::Value>>>;
    fn update_configuration(&self, environment_id: &str, configuration_id: &str, version: String, configuration: ::models::Configuration) -> Box<Future<Item = ::models::Configuration, Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>ConfigurationsApi for ConfigurationsApiClient<C> {
    fn create_configuration(&self, environment_id: &str, version: String, configuration: ::models::Configuration) -> Box<Future<Item = ::models::Configuration, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Post, "/v1/environments/{environment_id}/configurations".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_auth(__internal_request::Auth::Basic)
            .with_query_param("version".to_string(), version.to_string())
            .with_path_param("environment_id".to_string(), environment_id.to_string())
            .with_body_param(configuration)
            .execute(self.configuration.borrow())
    }

    fn delete_configuration(&self, environment_id: &str, configuration_id: &str, version: String) -> Box<Future<Item = ::models::DeleteConfigurationResponse, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Delete, "/v1/environments/{environment_id}/configurations/{configuration_id}".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_auth(__internal_request::Auth::Basic)
            .with_query_param("version".to_string(), version.to_string())
            .with_path_param("environment_id".to_string(), environment_id.to_string())
            .with_path_param("configuration_id".to_string(), configuration_id.to_string())
            .execute(self.configuration.borrow())
    }

    fn get_configuration(&self, environment_id: &str, configuration_id: &str, version: String) -> Box<Future<Item = ::models::Configuration, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/v1/environments/{environment_id}/configurations/{configuration_id}".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_auth(__internal_request::Auth::Basic)
            .with_query_param("version".to_string(), version.to_string())
            .with_path_param("environment_id".to_string(), environment_id.to_string())
            .with_path_param("configuration_id".to_string(), configuration_id.to_string())
            .execute(self.configuration.borrow())
    }

    fn list_configurations(&self, environment_id: &str, version: String, name: &str) -> Box<Future<Item = ::models::ListConfigurationsResponse, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/v1/environments/{environment_id}/configurations".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_auth(__internal_request::Auth::Basic)
            .with_query_param("name".to_string(), name.to_string())
            .with_query_param("version".to_string(), version.to_string())
            .with_path_param("environment_id".to_string(), environment_id.to_string())
            .execute(self.configuration.borrow())
    }

    fn update_configuration(&self, environment_id: &str, configuration_id: &str, version: String, configuration: ::models::Configuration) -> Box<Future<Item = ::models::Configuration, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Put, "/v1/environments/{environment_id}/configurations/{configuration_id}".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_auth(__internal_request::Auth::Basic)
            .with_query_param("version".to_string(), version.to_string())
            .with_path_param("environment_id".to_string(), environment_id.to_string())
            .with_path_param("configuration_id".to_string(), configuration_id.to_string())
            .with_body_param(configuration)
            .execute(self.configuration.borrow())
    }

}
