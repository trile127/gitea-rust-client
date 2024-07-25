/* 
 * Gitea API
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.21.11
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GeneralAttachmentSettings : GeneralAttachmentSettings contains global Attachment settings exposed by API

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneralAttachmentSettings {
  #[serde(rename = "allowed_types")]
  allowed_types: Option<String>,
  #[serde(rename = "enabled")]
  enabled: Option<bool>,
  #[serde(rename = "max_files")]
  max_files: Option<i64>,
  #[serde(rename = "max_size")]
  max_size: Option<i64>
}

impl GeneralAttachmentSettings {
  /// GeneralAttachmentSettings contains global Attachment settings exposed by API
  pub fn new() -> GeneralAttachmentSettings {
    GeneralAttachmentSettings {
      allowed_types: None,
      enabled: None,
      max_files: None,
      max_size: None
    }
  }

  pub fn set_allowed_types(&mut self, allowed_types: String) {
    self.allowed_types = Some(allowed_types);
  }

  pub fn with_allowed_types(mut self, allowed_types: String) -> GeneralAttachmentSettings {
    self.allowed_types = Some(allowed_types);
    self
  }

  pub fn allowed_types(&self) -> Option<&String> {
    self.allowed_types.as_ref()
  }

  pub fn reset_allowed_types(&mut self) {
    self.allowed_types = None;
  }

  pub fn set_enabled(&mut self, enabled: bool) {
    self.enabled = Some(enabled);
  }

  pub fn with_enabled(mut self, enabled: bool) -> GeneralAttachmentSettings {
    self.enabled = Some(enabled);
    self
  }

  pub fn enabled(&self) -> Option<&bool> {
    self.enabled.as_ref()
  }

  pub fn reset_enabled(&mut self) {
    self.enabled = None;
  }

  pub fn set_max_files(&mut self, max_files: i64) {
    self.max_files = Some(max_files);
  }

  pub fn with_max_files(mut self, max_files: i64) -> GeneralAttachmentSettings {
    self.max_files = Some(max_files);
    self
  }

  pub fn max_files(&self) -> Option<&i64> {
    self.max_files.as_ref()
  }

  pub fn reset_max_files(&mut self) {
    self.max_files = None;
  }

  pub fn set_max_size(&mut self, max_size: i64) {
    self.max_size = Some(max_size);
  }

  pub fn with_max_size(mut self, max_size: i64) -> GeneralAttachmentSettings {
    self.max_size = Some(max_size);
    self
  }

  pub fn max_size(&self) -> Option<&i64> {
    self.max_size.as_ref()
  }

  pub fn reset_max_size(&mut self) {
    self.max_size = None;
  }

}



