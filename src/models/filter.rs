/* 
 * Discovery
 *
 * The IBM Watson&trade; Discovery Service is a cognitive search and content analytics engine that you can add to applications to identify patterns, trends and actionable insights to drive better decision-making. Securely unify structured and unstructured data with pre-enriched content, and use a simplified query language to eliminate the need for manual filtering of results.
 *
 * OpenAPI spec version: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Filter {
  /// The type of aggregation command used. For example: term, filter, max, min, etc.
  #[serde(rename = "type")]
  type_: Option<String>,
  #[serde(rename = "results")]
  results: Option<Vec<::models::AggregationResult>>,
  /// Number of matching results.
  #[serde(rename = "matching_results")]
  matching_results: Option<i32>,
  /// Aggregations returned by the Discovery service.
  #[serde(rename = "aggregations")]
  aggregations: Option<Vec<::models::QueryAggregation>>,
  /// The match the aggregated results queried for.
  #[serde(rename = "match")]
  match_: Option<String>
}

impl Filter {
  pub fn new() -> Filter {
    Filter {
      type_: None,
      results: None,
      matching_results: None,
      aggregations: None,
      match_: None
    }
  }

  pub fn set_type_(&mut self, type_: String) {
    self.type_ = Some(type_);
  }

  pub fn with_type_(mut self, type_: String) -> Filter {
    self.type_ = Some(type_);
    self
  }

  pub fn type_(&self) -> Option<&String> {
    self.type_.as_ref()
  }

  pub fn reset_type_(&mut self) {
    self.type_ = None;
  }

  pub fn set_results(&mut self, results: Vec<::models::AggregationResult>) {
    self.results = Some(results);
  }

  pub fn with_results(mut self, results: Vec<::models::AggregationResult>) -> Filter {
    self.results = Some(results);
    self
  }

  pub fn results(&self) -> Option<&Vec<::models::AggregationResult>> {
    self.results.as_ref()
  }

  pub fn reset_results(&mut self) {
    self.results = None;
  }

  pub fn set_matching_results(&mut self, matching_results: i32) {
    self.matching_results = Some(matching_results);
  }

  pub fn with_matching_results(mut self, matching_results: i32) -> Filter {
    self.matching_results = Some(matching_results);
    self
  }

  pub fn matching_results(&self) -> Option<&i32> {
    self.matching_results.as_ref()
  }

  pub fn reset_matching_results(&mut self) {
    self.matching_results = None;
  }

  pub fn set_aggregations(&mut self, aggregations: Vec<::models::QueryAggregation>) {
    self.aggregations = Some(aggregations);
  }

  pub fn with_aggregations(mut self, aggregations: Vec<::models::QueryAggregation>) -> Filter {
    self.aggregations = Some(aggregations);
    self
  }

  pub fn aggregations(&self) -> Option<&Vec<::models::QueryAggregation>> {
    self.aggregations.as_ref()
  }

  pub fn reset_aggregations(&mut self) {
    self.aggregations = None;
  }

  pub fn set_match_(&mut self, match_: String) {
    self.match_ = Some(match_);
  }

  pub fn with_match_(mut self, match_: String) -> Filter {
    self.match_ = Some(match_);
    self
  }

  pub fn match_(&self) -> Option<&String> {
    self.match_.as_ref()
  }

  pub fn reset_match_(&mut self) {
    self.match_ = None;
  }

}



