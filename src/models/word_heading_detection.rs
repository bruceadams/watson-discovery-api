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
pub struct WordHeadingDetection {
  #[serde(rename = "fonts")]
  fonts: Option<Vec<::models::FontSetting>>,
  #[serde(rename = "styles")]
  styles: Option<Vec<::models::WordStyle>>
}

impl WordHeadingDetection {
  pub fn new() -> WordHeadingDetection {
    WordHeadingDetection {
      fonts: None,
      styles: None
    }
  }

  pub fn set_fonts(&mut self, fonts: Vec<::models::FontSetting>) {
    self.fonts = Some(fonts);
  }

  pub fn with_fonts(mut self, fonts: Vec<::models::FontSetting>) -> WordHeadingDetection {
    self.fonts = Some(fonts);
    self
  }

  pub fn fonts(&self) -> Option<&Vec<::models::FontSetting>> {
    self.fonts.as_ref()
  }

  pub fn reset_fonts(&mut self) {
    self.fonts = None;
  }

  pub fn set_styles(&mut self, styles: Vec<::models::WordStyle>) {
    self.styles = Some(styles);
  }

  pub fn with_styles(mut self, styles: Vec<::models::WordStyle>) -> WordHeadingDetection {
    self.styles = Some(styles);
    self
  }

  pub fn styles(&self) -> Option<&Vec<::models::WordStyle>> {
    self.styles.as_ref()
  }

  pub fn reset_styles(&mut self) {
    self.styles = None;
  }

}


