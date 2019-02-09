/* 
 * Discovery
 *
 * The IBM Watson&trade; Discovery Service is a cognitive search and content analytics engine that you can add to applications to identify patterns, trends and actionable insights to drive better decision-making. Securely unify structured and unstructured data with pre-enriched content, and use a simplified query language to eliminate the need for manual filtering of results.
 *
 * OpenAPI spec version: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// MetricTokenResponse : The response generated from a call to a **metrics** method that evaluates tokens.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MetricTokenResponse {
  #[serde(rename = "aggregations")]
  aggregations: Option<Vec<::models::MetricTokenAggregation>>
}

impl MetricTokenResponse {
  /// The response generated from a call to a **metrics** method that evaluates tokens.
  pub fn new() -> MetricTokenResponse {
    MetricTokenResponse {
      aggregations: None
    }
  }

  pub fn set_aggregations(&mut self, aggregations: Vec<::models::MetricTokenAggregation>) {
    self.aggregations = Some(aggregations);
  }

  pub fn with_aggregations(mut self, aggregations: Vec<::models::MetricTokenAggregation>) -> MetricTokenResponse {
    self.aggregations = Some(aggregations);
    self
  }

  pub fn aggregations(&self) -> Option<&Vec<::models::MetricTokenAggregation>> {
    self.aggregations.as_ref()
  }

  pub fn reset_aggregations(&mut self) {
    self.aggregations = None;
  }

}


