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
pub struct CredentialsList {
  /// An array of credential definitions that were created for this instance.
  #[serde(rename = "credentials")]
  credentials: Option<Vec<::models::Credentials>>
}

impl CredentialsList {
  pub fn new() -> CredentialsList {
    CredentialsList {
      credentials: None
    }
  }

  pub fn set_credentials(&mut self, credentials: Vec<::models::Credentials>) {
    self.credentials = Some(credentials);
  }

  pub fn with_credentials(mut self, credentials: Vec<::models::Credentials>) -> CredentialsList {
    self.credentials = Some(credentials);
    self
  }

  pub fn credentials(&self) -> Option<&Vec<::models::Credentials>> {
    self.credentials.as_ref()
  }

  pub fn reset_credentials(&mut self) {
    self.credentials = None;
  }

}



