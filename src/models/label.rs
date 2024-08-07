/* 
 * Gitea API
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.21.11
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// Label : Label a label to an issue or a pr

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Label {
  #[serde(rename = "color")]
  color: Option<String>,
  #[serde(rename = "description")]
  description: Option<String>,
  #[serde(rename = "exclusive")]
  exclusive: Option<bool>,
  #[serde(rename = "id")]
  id: Option<i64>,
  #[serde(rename = "is_archived")]
  is_archived: Option<bool>,
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "url")]
  url: Option<String>
}

impl Label {
  /// Label a label to an issue or a pr
  pub fn new() -> Label {
    Label {
      color: None,
      description: None,
      exclusive: None,
      id: None,
      is_archived: None,
      name: None,
      url: None
    }
  }

  pub fn set_color(&mut self, color: String) {
    self.color = Some(color);
  }

  pub fn with_color(mut self, color: String) -> Label {
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

  pub fn with_description(mut self, description: String) -> Label {
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

  pub fn with_exclusive(mut self, exclusive: bool) -> Label {
    self.exclusive = Some(exclusive);
    self
  }

  pub fn exclusive(&self) -> Option<&bool> {
    self.exclusive.as_ref()
  }

  pub fn reset_exclusive(&mut self) {
    self.exclusive = None;
  }

  pub fn set_id(&mut self, id: i64) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: i64) -> Label {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&i64> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_is_archived(&mut self, is_archived: bool) {
    self.is_archived = Some(is_archived);
  }

  pub fn with_is_archived(mut self, is_archived: bool) -> Label {
    self.is_archived = Some(is_archived);
    self
  }

  pub fn is_archived(&self) -> Option<&bool> {
    self.is_archived.as_ref()
  }

  pub fn reset_is_archived(&mut self) {
    self.is_archived = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> Label {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_url(&mut self, url: String) {
    self.url = Some(url);
  }

  pub fn with_url(mut self, url: String) -> Label {
    self.url = Some(url);
    self
  }

  pub fn url(&self) -> Option<&String> {
    self.url.as_ref()
  }

  pub fn reset_url(&mut self) {
    self.url = None;
  }

}



