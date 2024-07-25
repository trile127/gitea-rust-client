/* 
 * Gitea API
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.21.11
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GitBlobResponse : GitBlobResponse represents a git blob

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GitBlobResponse {
  #[serde(rename = "content")]
  content: Option<String>,
  #[serde(rename = "encoding")]
  encoding: Option<String>,
  #[serde(rename = "sha")]
  sha: Option<String>,
  #[serde(rename = "size")]
  size: Option<i64>,
  #[serde(rename = "url")]
  url: Option<String>
}

impl GitBlobResponse {
  /// GitBlobResponse represents a git blob
  pub fn new() -> GitBlobResponse {
    GitBlobResponse {
      content: None,
      encoding: None,
      sha: None,
      size: None,
      url: None
    }
  }

  pub fn set_content(&mut self, content: String) {
    self.content = Some(content);
  }

  pub fn with_content(mut self, content: String) -> GitBlobResponse {
    self.content = Some(content);
    self
  }

  pub fn content(&self) -> Option<&String> {
    self.content.as_ref()
  }

  pub fn reset_content(&mut self) {
    self.content = None;
  }

  pub fn set_encoding(&mut self, encoding: String) {
    self.encoding = Some(encoding);
  }

  pub fn with_encoding(mut self, encoding: String) -> GitBlobResponse {
    self.encoding = Some(encoding);
    self
  }

  pub fn encoding(&self) -> Option<&String> {
    self.encoding.as_ref()
  }

  pub fn reset_encoding(&mut self) {
    self.encoding = None;
  }

  pub fn set_sha(&mut self, sha: String) {
    self.sha = Some(sha);
  }

  pub fn with_sha(mut self, sha: String) -> GitBlobResponse {
    self.sha = Some(sha);
    self
  }

  pub fn sha(&self) -> Option<&String> {
    self.sha.as_ref()
  }

  pub fn reset_sha(&mut self) {
    self.sha = None;
  }

  pub fn set_size(&mut self, size: i64) {
    self.size = Some(size);
  }

  pub fn with_size(mut self, size: i64) -> GitBlobResponse {
    self.size = Some(size);
    self
  }

  pub fn size(&self) -> Option<&i64> {
    self.size.as_ref()
  }

  pub fn reset_size(&mut self) {
    self.size = None;
  }

  pub fn set_url(&mut self, url: String) {
    self.url = Some(url);
  }

  pub fn with_url(mut self, url: String) -> GitBlobResponse {
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



