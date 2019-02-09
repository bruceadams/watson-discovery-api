/* 
 * Discovery
 *
 * The IBM Watson&trade; Discovery Service is a cognitive search and content analytics engine that you can add to applications to identify patterns, trends and actionable insights to drive better decision-making. Securely unify structured and unstructured data with pre-enriched content, and use a simplified query language to eliminate the need for manual filtering of results.
 *
 * OpenAPI spec version: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TokenDict : Tokenization dictionary describing how words are tokenized during ingestion and at query time.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenDict {
  /// An array of tokenization rules. Each rule contains, the original `text` string, component `tokens`, any alternate character set `readings`, and which `part_of_speech` the text is from.
  #[serde(rename = "tokenization_rules")]
  tokenization_rules: Option<Vec<::models::TokenDictRule>>
}

impl TokenDict {
  /// Tokenization dictionary describing how words are tokenized during ingestion and at query time.
  pub fn new() -> TokenDict {
    TokenDict {
      tokenization_rules: None
    }
  }

  pub fn set_tokenization_rules(&mut self, tokenization_rules: Vec<::models::TokenDictRule>) {
    self.tokenization_rules = Some(tokenization_rules);
  }

  pub fn with_tokenization_rules(mut self, tokenization_rules: Vec<::models::TokenDictRule>) -> TokenDict {
    self.tokenization_rules = Some(tokenization_rules);
    self
  }

  pub fn tokenization_rules(&self) -> Option<&Vec<::models::TokenDictRule>> {
    self.tokenization_rules.as_ref()
  }

  pub fn reset_tokenization_rules(&mut self) {
    self.tokenization_rules = None;
  }

}



