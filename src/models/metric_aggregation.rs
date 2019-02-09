/* 
 * Discovery
 *
 * The IBM Watson&trade; Discovery Service is a cognitive search and content analytics engine that you can add to applications to identify patterns, trends and actionable insights to drive better decision-making. Securely unify structured and unstructured data with pre-enriched content, and use a simplified query language to eliminate the need for manual filtering of results.
 *
 * OpenAPI spec version: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// MetricAggregation : An aggregation analyzing log information for queries and events.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MetricAggregation {
  /// The measurement interval for this metric. Metric intervals are always 1 day (`1d`).
  #[serde(rename = "interval")]
  interval: Option<String>,
  /// The event type associated with this metric result. This field, when present, will always be `click`.
  #[serde(rename = "event_type")]
  event_type: Option<String>,
  #[serde(rename = "results")]
  results: Option<Vec<::models::MetricAggregationResult>>
}

impl MetricAggregation {
  /// An aggregation analyzing log information for queries and events.
  pub fn new() -> MetricAggregation {
    MetricAggregation {
      interval: None,
      event_type: None,
      results: None
    }
  }

  pub fn set_interval(&mut self, interval: String) {
    self.interval = Some(interval);
  }

  pub fn with_interval(mut self, interval: String) -> MetricAggregation {
    self.interval = Some(interval);
    self
  }

  pub fn interval(&self) -> Option<&String> {
    self.interval.as_ref()
  }

  pub fn reset_interval(&mut self) {
    self.interval = None;
  }

  pub fn set_event_type(&mut self, event_type: String) {
    self.event_type = Some(event_type);
  }

  pub fn with_event_type(mut self, event_type: String) -> MetricAggregation {
    self.event_type = Some(event_type);
    self
  }

  pub fn event_type(&self) -> Option<&String> {
    self.event_type.as_ref()
  }

  pub fn reset_event_type(&mut self) {
    self.event_type = None;
  }

  pub fn set_results(&mut self, results: Vec<::models::MetricAggregationResult>) {
    self.results = Some(results);
  }

  pub fn with_results(mut self, results: Vec<::models::MetricAggregationResult>) -> MetricAggregation {
    self.results = Some(results);
    self
  }

  pub fn results(&self) -> Option<&Vec<::models::MetricAggregationResult>> {
    self.results.as_ref()
  }

  pub fn reset_results(&mut self) {
    self.results = None;
  }

}


