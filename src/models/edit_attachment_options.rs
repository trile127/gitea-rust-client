/* 
 * Gitea API
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.21.11
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// EditAttachmentOptions : EditAttachmentOptions options for editing attachments

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EditAttachmentOptions {
  #[serde(rename = "name")]
  name: Option<String>
}

impl EditAttachmentOptions {
  /// EditAttachmentOptions options for editing attachments
  pub fn new() -> EditAttachmentOptions {
    EditAttachmentOptions {
      name: None
    }
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> EditAttachmentOptions {
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



