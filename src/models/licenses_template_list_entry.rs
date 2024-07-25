/* 
 * Gitea API
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.21.11
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// LicensesTemplateListEntry : LicensesListEntry is used for the API

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct LicensesTemplateListEntry {
  #[serde(rename = "key")]
  key: Option<String>,
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "url")]
  url: Option<String>
}

impl LicensesTemplateListEntry {
  /// LicensesListEntry is used for the API
  pub fn new() -> LicensesTemplateListEntry {
    LicensesTemplateListEntry {
      key: None,
      name: None,
      url: None
    }
  }

  pub fn set_key(&mut self, key: String) {
    self.key = Some(key);
  }

  pub fn with_key(mut self, key: String) -> LicensesTemplateListEntry {
    self.key = Some(key);
    self
  }

  pub fn key(&self) -> Option<&String> {
    self.key.as_ref()
  }

  pub fn reset_key(&mut self) {
    self.key = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> LicensesTemplateListEntry {
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

  pub fn with_url(mut self, url: String) -> LicensesTemplateListEntry {
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



