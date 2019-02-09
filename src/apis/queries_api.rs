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

pub struct QueriesApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> QueriesApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> QueriesApiClient<C> {
        QueriesApiClient {
            configuration: configuration,
        }
    }
}

pub trait QueriesApi {
    fn federated_query(&self, environment_id: &str, version: String, x_watson_logging_opt_out: bool, query_large: ::models::QueryLarge) -> Box<Future<Item = ::models::QueryResponse, Error = Error<serde_json::Value>>>;
    fn federated_query_notices(&self, environment_id: &str, collection_ids: Vec<String>, version: String, filter: &str, query: &str, natural_language_query: &str, aggregation: &str, count: i32, return_: Vec<String>, offset: i32, sort: Vec<String>, highlight: bool, deduplicate_field: &str, similar: bool, similar_document_ids: Vec<String>, similar_fields: Vec<String>) -> Box<Future<Item = ::models::QueryNoticesResponse, Error = Error<serde_json::Value>>>;
    fn federated_query_using_get(&self, environment_id: &str, collection_ids: Vec<String>, version: String, filter: &str, query: &str, natural_language_query: &str, aggregation: &str, count: i32, return_: Vec<String>, offset: i32, sort: Vec<String>, highlight: bool, deduplicate: bool, deduplicate_field: &str, similar: bool, similar_document_ids: Vec<String>, similar_fields: Vec<String>, passages: bool, passages_fields: Vec<String>, passages_count: i32, passages_characters: i32, bias: &str) -> Box<Future<Item = ::models::QueryResponse, Error = Error<serde_json::Value>>>;
    fn query(&self, environment_id: &str, collection_id: &str, version: String, x_watson_logging_opt_out: bool, query_large: ::models::QueryLarge) -> Box<Future<Item = ::models::QueryResponse, Error = Error<serde_json::Value>>>;
    fn query_entities(&self, environment_id: &str, collection_id: &str, version: String, query_entities: ::models::QueryEntities) -> Box<Future<Item = ::models::QueryEntitiesResponse, Error = Error<serde_json::Value>>>;
    fn query_notices(&self, environment_id: &str, collection_id: &str, version: String, filter: &str, query: &str, natural_language_query: &str, passages: bool, aggregation: &str, count: i32, return_: Vec<String>, offset: i32, sort: Vec<String>, highlight: bool, passages_fields: Vec<String>, passages_count: i32, passages_characters: i32, deduplicate_field: &str, similar: bool, similar_document_ids: Vec<String>, similar_fields: Vec<String>) -> Box<Future<Item = ::models::QueryNoticesResponse, Error = Error<serde_json::Value>>>;
    fn query_relations(&self, environment_id: &str, collection_id: &str, version: String, query_relations: ::models::QueryRelations) -> Box<Future<Item = ::models::QueryRelationsResponse, Error = Error<serde_json::Value>>>;
    fn query_using_get(&self, environment_id: &str, collection_id: &str, version: String, filter: &str, query: &str, natural_language_query: &str, passages: bool, aggregation: &str, count: i32, return_: Vec<String>, offset: i32, sort: Vec<String>, highlight: bool, passages_fields: Vec<String>, passages_count: i32, passages_characters: i32, deduplicate: bool, deduplicate_field: &str, similar: bool, similar_document_ids: Vec<String>, similar_fields: Vec<String>, x_watson_logging_opt_out: bool, bias: &str) -> Box<Future<Item = ::models::QueryResponse, Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>QueriesApi for QueriesApiClient<C> {
    fn federated_query(&self, environment_id: &str, version: String, x_watson_logging_opt_out: bool, query_large: ::models::QueryLarge) -> Box<Future<Item = ::models::QueryResponse, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Post, "/v1/environments/{environment_id}/query".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_auth(__internal_request::Auth::Basic)
            .with_query_param("version".to_string(), version.to_string())
            .with_path_param("environment_id".to_string(), environment_id.to_string())
            .with_header_param("X-Watson-Logging-Opt-Out".to_string(), x_watson_logging_opt_out.to_string())
            .with_body_param(query_large)
            .execute(self.configuration.borrow())
    }

    fn federated_query_notices(&self, environment_id: &str, collection_ids: Vec<String>, version: String, filter: &str, query: &str, natural_language_query: &str, aggregation: &str, count: i32, return_: Vec<String>, offset: i32, sort: Vec<String>, highlight: bool, deduplicate_field: &str, similar: bool, similar_document_ids: Vec<String>, similar_fields: Vec<String>) -> Box<Future<Item = ::models::QueryNoticesResponse, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/v1/environments/{environment_id}/notices".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_auth(__internal_request::Auth::Basic)
            .with_query_param("collection_ids".to_string(), collection_ids.join(",").to_string())
            .with_query_param("filter".to_string(), filter.to_string())
            .with_query_param("query".to_string(), query.to_string())
            .with_query_param("natural_language_query".to_string(), natural_language_query.to_string())
            .with_query_param("aggregation".to_string(), aggregation.to_string())
            .with_query_param("count".to_string(), count.to_string())
            .with_query_param("return".to_string(), return_.join(",").to_string())
            .with_query_param("offset".to_string(), offset.to_string())
            .with_query_param("sort".to_string(), sort.join(",").to_string())
            .with_query_param("highlight".to_string(), highlight.to_string())
            .with_query_param("deduplicate.field".to_string(), deduplicate_field.to_string())
            .with_query_param("similar".to_string(), similar.to_string())
            .with_query_param("similar.document_ids".to_string(), similar_document_ids.join(",").to_string())
            .with_query_param("similar.fields".to_string(), similar_fields.join(",").to_string())
            .with_query_param("version".to_string(), version.to_string())
            .with_path_param("environment_id".to_string(), environment_id.to_string())
            .execute(self.configuration.borrow())
    }

    fn federated_query_using_get(&self, environment_id: &str, collection_ids: Vec<String>, version: String, filter: &str, query: &str, natural_language_query: &str, aggregation: &str, count: i32, return_: Vec<String>, offset: i32, sort: Vec<String>, highlight: bool, deduplicate: bool, deduplicate_field: &str, similar: bool, similar_document_ids: Vec<String>, similar_fields: Vec<String>, passages: bool, passages_fields: Vec<String>, passages_count: i32, passages_characters: i32, bias: &str) -> Box<Future<Item = ::models::QueryResponse, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/v1/environments/{environment_id}/query".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_auth(__internal_request::Auth::Basic)
            .with_query_param("collection_ids".to_string(), collection_ids.join(",").to_string())
            .with_query_param("filter".to_string(), filter.to_string())
            .with_query_param("query".to_string(), query.to_string())
            .with_query_param("natural_language_query".to_string(), natural_language_query.to_string())
            .with_query_param("aggregation".to_string(), aggregation.to_string())
            .with_query_param("count".to_string(), count.to_string())
            .with_query_param("return".to_string(), return_.join(",").to_string())
            .with_query_param("offset".to_string(), offset.to_string())
            .with_query_param("sort".to_string(), sort.join(",").to_string())
            .with_query_param("highlight".to_string(), highlight.to_string())
            .with_query_param("deduplicate".to_string(), deduplicate.to_string())
            .with_query_param("deduplicate.field".to_string(), deduplicate_field.to_string())
            .with_query_param("similar".to_string(), similar.to_string())
            .with_query_param("similar.document_ids".to_string(), similar_document_ids.join(",").to_string())
            .with_query_param("similar.fields".to_string(), similar_fields.join(",").to_string())
            .with_query_param("version".to_string(), version.to_string())
            .with_query_param("passages".to_string(), passages.to_string())
            .with_query_param("passages.fields".to_string(), passages_fields.join(",").to_string())
            .with_query_param("passages.count".to_string(), passages_count.to_string())
            .with_query_param("passages.characters".to_string(), passages_characters.to_string())
            .with_query_param("bias".to_string(), bias.to_string())
            .with_path_param("environment_id".to_string(), environment_id.to_string())
            .execute(self.configuration.borrow())
    }

    fn query(&self, environment_id: &str, collection_id: &str, version: String, x_watson_logging_opt_out: bool, query_large: ::models::QueryLarge) -> Box<Future<Item = ::models::QueryResponse, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Post, "/v1/environments/{environment_id}/collections/{collection_id}/query".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_auth(__internal_request::Auth::Basic)
            .with_query_param("version".to_string(), version.to_string())
            .with_path_param("environment_id".to_string(), environment_id.to_string())
            .with_path_param("collection_id".to_string(), collection_id.to_string())
            .with_header_param("X-Watson-Logging-Opt-Out".to_string(), x_watson_logging_opt_out.to_string())
            .with_body_param(query_large)
            .execute(self.configuration.borrow())
    }

    fn query_entities(&self, environment_id: &str, collection_id: &str, version: String, query_entities: ::models::QueryEntities) -> Box<Future<Item = ::models::QueryEntitiesResponse, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Post, "/v1/environments/{environment_id}/collections/{collection_id}/query_entities".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_auth(__internal_request::Auth::Basic)
            .with_query_param("version".to_string(), version.to_string())
            .with_path_param("environment_id".to_string(), environment_id.to_string())
            .with_path_param("collection_id".to_string(), collection_id.to_string())
            .with_body_param(query_entities)
            .execute(self.configuration.borrow())
    }

    fn query_notices(&self, environment_id: &str, collection_id: &str, version: String, filter: &str, query: &str, natural_language_query: &str, passages: bool, aggregation: &str, count: i32, return_: Vec<String>, offset: i32, sort: Vec<String>, highlight: bool, passages_fields: Vec<String>, passages_count: i32, passages_characters: i32, deduplicate_field: &str, similar: bool, similar_document_ids: Vec<String>, similar_fields: Vec<String>) -> Box<Future<Item = ::models::QueryNoticesResponse, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/v1/environments/{environment_id}/collections/{collection_id}/notices".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_auth(__internal_request::Auth::Basic)
            .with_query_param("filter".to_string(), filter.to_string())
            .with_query_param("query".to_string(), query.to_string())
            .with_query_param("natural_language_query".to_string(), natural_language_query.to_string())
            .with_query_param("passages".to_string(), passages.to_string())
            .with_query_param("aggregation".to_string(), aggregation.to_string())
            .with_query_param("count".to_string(), count.to_string())
            .with_query_param("return".to_string(), return_.join(",").to_string())
            .with_query_param("offset".to_string(), offset.to_string())
            .with_query_param("sort".to_string(), sort.join(",").to_string())
            .with_query_param("highlight".to_string(), highlight.to_string())
            .with_query_param("passages.fields".to_string(), passages_fields.join(",").to_string())
            .with_query_param("passages.count".to_string(), passages_count.to_string())
            .with_query_param("passages.characters".to_string(), passages_characters.to_string())
            .with_query_param("deduplicate.field".to_string(), deduplicate_field.to_string())
            .with_query_param("similar".to_string(), similar.to_string())
            .with_query_param("similar.document_ids".to_string(), similar_document_ids.join(",").to_string())
            .with_query_param("similar.fields".to_string(), similar_fields.join(",").to_string())
            .with_query_param("version".to_string(), version.to_string())
            .with_path_param("environment_id".to_string(), environment_id.to_string())
            .with_path_param("collection_id".to_string(), collection_id.to_string())
            .execute(self.configuration.borrow())
    }

    fn query_relations(&self, environment_id: &str, collection_id: &str, version: String, query_relations: ::models::QueryRelations) -> Box<Future<Item = ::models::QueryRelationsResponse, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Post, "/v1/environments/{environment_id}/collections/{collection_id}/query_relations".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_auth(__internal_request::Auth::Basic)
            .with_query_param("version".to_string(), version.to_string())
            .with_path_param("environment_id".to_string(), environment_id.to_string())
            .with_path_param("collection_id".to_string(), collection_id.to_string())
            .with_body_param(query_relations)
            .execute(self.configuration.borrow())
    }

    fn query_using_get(&self, environment_id: &str, collection_id: &str, version: String, filter: &str, query: &str, natural_language_query: &str, passages: bool, aggregation: &str, count: i32, return_: Vec<String>, offset: i32, sort: Vec<String>, highlight: bool, passages_fields: Vec<String>, passages_count: i32, passages_characters: i32, deduplicate: bool, deduplicate_field: &str, similar: bool, similar_document_ids: Vec<String>, similar_fields: Vec<String>, x_watson_logging_opt_out: bool, bias: &str) -> Box<Future<Item = ::models::QueryResponse, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/v1/environments/{environment_id}/collections/{collection_id}/query".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_auth(__internal_request::Auth::Basic)
            .with_query_param("filter".to_string(), filter.to_string())
            .with_query_param("query".to_string(), query.to_string())
            .with_query_param("natural_language_query".to_string(), natural_language_query.to_string())
            .with_query_param("passages".to_string(), passages.to_string())
            .with_query_param("aggregation".to_string(), aggregation.to_string())
            .with_query_param("count".to_string(), count.to_string())
            .with_query_param("return".to_string(), return_.join(",").to_string())
            .with_query_param("offset".to_string(), offset.to_string())
            .with_query_param("sort".to_string(), sort.join(",").to_string())
            .with_query_param("highlight".to_string(), highlight.to_string())
            .with_query_param("passages.fields".to_string(), passages_fields.join(",").to_string())
            .with_query_param("passages.count".to_string(), passages_count.to_string())
            .with_query_param("passages.characters".to_string(), passages_characters.to_string())
            .with_query_param("deduplicate".to_string(), deduplicate.to_string())
            .with_query_param("deduplicate.field".to_string(), deduplicate_field.to_string())
            .with_query_param("similar".to_string(), similar.to_string())
            .with_query_param("similar.document_ids".to_string(), similar_document_ids.join(",").to_string())
            .with_query_param("similar.fields".to_string(), similar_fields.join(",").to_string())
            .with_query_param("version".to_string(), version.to_string())
            .with_query_param("bias".to_string(), bias.to_string())
            .with_path_param("environment_id".to_string(), environment_id.to_string())
            .with_path_param("collection_id".to_string(), collection_id.to_string())
            .with_header_param("X-Watson-Logging-Opt-Out".to_string(), x_watson_logging_opt_out.to_string())
            .execute(self.configuration.borrow())
    }

}