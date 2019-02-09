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
pub struct Histogram {
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
  /// The field where the aggregation is located in the document.
  #[serde(rename = "field")]
  field: Option<String>,
  /// Interval of the aggregation. (For 'histogram' type).
  #[serde(rename = "interval")]
  interval: Option<i32>
}

impl Histogram {
  pub fn new() -> Histogram {
    Histogram {
      type_: None,
      results: None,
      matching_results: None,
      aggregations: None,
      field: None,
      interval: None
    }
  }

  pub fn set_type_(&mut self, type_: String) {
    self.type_ = Some(type_);
  }

  pub fn with_type_(mut self, type_: String) -> Histogram {
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

  pub fn with_results(mut self, results: Vec<::models::AggregationResult>) -> Histogram {
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

  pub fn with_matching_results(mut self, matching_results: i32) -> Histogram {
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

  pub fn with_aggregations(mut self, aggregations: Vec<::models::QueryAggregation>) -> Histogram {
    self.aggregations = Some(aggregations);
    self
  }

  pub fn aggregations(&self) -> Option<&Vec<::models::QueryAggregation>> {
    self.aggregations.as_ref()
  }

  pub fn reset_aggregations(&mut self) {
    self.aggregations = None;
  }

  pub fn set_field(&mut self, field: String) {
    self.field = Some(field);
  }

  pub fn with_field(mut self, field: String) -> Histogram {
    self.field = Some(field);
    self
  }

  pub fn field(&self) -> Option<&String> {
    self.field.as_ref()
  }

  pub fn reset_field(&mut self) {
    self.field = None;
  }

  pub fn set_interval(&mut self, interval: i32) {
    self.interval = Some(interval);
  }

  pub fn with_interval(mut self, interval: i32) -> Histogram {
    self.interval = Some(interval);
    self
  }

  pub fn interval(&self) -> Option<&i32> {
    self.interval.as_ref()
  }

  pub fn reset_interval(&mut self) {
    self.interval = None;
  }

}



