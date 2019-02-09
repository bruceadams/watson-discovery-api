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

pub struct GatewayConfigurationApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> GatewayConfigurationApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> GatewayConfigurationApiClient<C> {
        GatewayConfigurationApiClient {
            configuration: configuration,
        }
    }
}

pub trait GatewayConfigurationApi {
    fn create_gateway(&self, environment_id: &str, version: String, gateway_name: ::models::GatewayName) -> Box<Future<Item = ::models::Gateway, Error = Error<serde_json::Value>>>;
    fn delete_gateway(&self, environment_id: &str, version: String, gateway_id: &str) -> Box<Future<Item = ::models::GatewayDelete, Error = Error<serde_json::Value>>>;
    fn get_gateway(&self, environment_id: &str, version: String, gateway_id: &str) -> Box<Future<Item = ::models::Gateway, Error = Error<serde_json::Value>>>;
    fn list_gateways(&self, environment_id: &str, version: String) -> Box<Future<Item = ::models::GatewayList, Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>GatewayConfigurationApi for GatewayConfigurationApiClient<C> {
    fn create_gateway(&self, environment_id: &str, version: String, gateway_name: ::models::GatewayName) -> Box<Future<Item = ::models::Gateway, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Post, "/v1/environments/{environment_id}/gateways".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_auth(__internal_request::Auth::Basic)
            .with_query_param("version".to_string(), version.to_string())
            .with_path_param("environment_id".to_string(), environment_id.to_string())
            .with_body_param(gateway_name)
            .execute(self.configuration.borrow())
    }

    fn delete_gateway(&self, environment_id: &str, version: String, gateway_id: &str) -> Box<Future<Item = ::models::GatewayDelete, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Delete, "/v1/environments/{environment_id}/gateways/{gateway_id}".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_auth(__internal_request::Auth::Basic)
            .with_query_param("version".to_string(), version.to_string())
            .with_path_param("environment_id".to_string(), environment_id.to_string())
            .with_path_param("gateway_id".to_string(), gateway_id.to_string())
            .execute(self.configuration.borrow())
    }

    fn get_gateway(&self, environment_id: &str, version: String, gateway_id: &str) -> Box<Future<Item = ::models::Gateway, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/v1/environments/{environment_id}/gateways/{gateway_id}".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_auth(__internal_request::Auth::Basic)
            .with_query_param("version".to_string(), version.to_string())
            .with_path_param("environment_id".to_string(), environment_id.to_string())
            .with_path_param("gateway_id".to_string(), gateway_id.to_string())
            .execute(self.configuration.borrow())
    }

    fn list_gateways(&self, environment_id: &str, version: String) -> Box<Future<Item = ::models::GatewayList, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/v1/environments/{environment_id}/gateways".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_auth(__internal_request::Auth::Basic)
            .with_query_param("version".to_string(), version.to_string())
            .with_path_param("environment_id".to_string(), environment_id.to_string())
            .execute(self.configuration.borrow())
    }

}
