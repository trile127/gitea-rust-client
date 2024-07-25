/* 
 * Gitea API
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.21.11
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// LabelTemplate : LabelTemplate info of a Label template

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct LabelTemplate {
  #[serde(rename = "color")]
  color: Option<String>,
  #[serde(rename = "description")]
  description: Option<String>,
  #[serde(rename = "exclusive")]
  exclusive: Option<bool>,
  #[serde(rename = "name")]
  name: Option<String>
}

impl LabelTemplate {
  /// LabelTemplate info of a Label template
  pub fn new() -> LabelTemplate {
    LabelTemplate {
      color: None,
      description: None,
      exclusive: None,
      name: None
    }
  }

  pub fn set_color(&mut self, color: String) {
    self.color = Some(color);
  }

  pub fn with_color(mut self, color: String) -> LabelTemplate {
    self.color = Some(color);
    self
  }

  pub fn color(&self) -> Option<&String> {
    self.color.as_ref()
  }

  pub fn reset_color(&mut self) {
    self.color = None;
  }

  pub fn set_description(&mut self, description: String) {
    self.description = Some(description);
  }

  pub fn with_description(mut self, description: String) -> LabelTemplate {
    self.description = Some(description);
    self
  }

  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }

  pub fn reset_description(&mut self) {
    self.description = None;
  }

  pub fn set_exclusive(&mut self, exclusive: bool) {
    self.exclusive = Some(exclusive);
  }

  pub fn with_exclusive(mut self, exclusive: bool) -> LabelTemplate {
    self.exclusive = Some(exclusive);
    self
  }

  pub fn exclusive(&self) -> Option<&bool> {
    self.exclusive.as_ref()
  }

  pub fn reset_exclusive(&mut self) {
    self.exclusive = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> LabelTemplate {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

}



