/* 
 * Gitea API
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.21.11
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// LicenseTemplateInfo : LicensesInfo contains information about a License

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct LicenseTemplateInfo {
  #[serde(rename = "body")]
  body: Option<String>,
  #[serde(rename = "implementation")]
  implementation: Option<String>,
  #[serde(rename = "key")]
  key: Option<String>,
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "url")]
  url: Option<String>
}

impl LicenseTemplateInfo {
  /// LicensesInfo contains information about a License
  pub fn new() -> LicenseTemplateInfo {
    LicenseTemplateInfo {
      body: None,
      implementation: None,
      key: None,
      name: None,
      url: None
    }
  }

  pub fn set_body(&mut self, body: String) {
    self.body = Some(body);
  }

  pub fn with_body(mut self, body: String) -> LicenseTemplateInfo {
    self.body = Some(body);
    self
  }

  pub fn body(&self) -> Option<&String> {
    self.body.as_ref()
  }

  pub fn reset_body(&mut self) {
    self.body = None;
  }

  pub fn set_implementation(&mut self, implementation: String) {
    self.implementation = Some(implementation);
  }

  pub fn with_implementation(mut self, implementation: String) -> LicenseTemplateInfo {
    self.implementation = Some(implementation);
    self
  }

  pub fn implementation(&self) -> Option<&String> {
    self.implementation.as_ref()
  }

  pub fn reset_implementation(&mut self) {
    self.implementation = None;
  }

  pub fn set_key(&mut self, key: String) {
    self.key = Some(key);
  }

  pub fn with_key(mut self, key: String) -> LicenseTemplateInfo {
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

  pub fn with_name(mut self, name: String) -> LicenseTemplateInfo {
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

  pub fn with_url(mut self, url: String) -> LicenseTemplateInfo {
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



