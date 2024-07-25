/* 
 * Gitea API
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.21.11
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PackageFile : PackageFile represents a package file

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageFile {
  #[serde(rename = "Size")]
  size: Option<i64>,
  #[serde(rename = "id")]
  id: Option<i64>,
  #[serde(rename = "md5")]
  md5: Option<String>,
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "sha1")]
  sha1: Option<String>,
  #[serde(rename = "sha256")]
  sha256: Option<String>,
  #[serde(rename = "sha512")]
  sha512: Option<String>
}

impl PackageFile {
  /// PackageFile represents a package file
  pub fn new() -> PackageFile {
    PackageFile {
      size: None,
      id: None,
      md5: None,
      name: None,
      sha1: None,
      sha256: None,
      sha512: None
    }
  }

  pub fn set_size(&mut self, size: i64) {
    self.size = Some(size);
  }

  pub fn with_size(mut self, size: i64) -> PackageFile {
    self.size = Some(size);
    self
  }

  pub fn size(&self) -> Option<&i64> {
    self.size.as_ref()
  }

  pub fn reset_size(&mut self) {
    self.size = None;
  }

  pub fn set_id(&mut self, id: i64) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: i64) -> PackageFile {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&i64> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_md5(&mut self, md5: String) {
    self.md5 = Some(md5);
  }

  pub fn with_md5(mut self, md5: String) -> PackageFile {
    self.md5 = Some(md5);
    self
  }

  pub fn md5(&self) -> Option<&String> {
    self.md5.as_ref()
  }

  pub fn reset_md5(&mut self) {
    self.md5 = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> PackageFile {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_sha1(&mut self, sha1: String) {
    self.sha1 = Some(sha1);
  }

  pub fn with_sha1(mut self, sha1: String) -> PackageFile {
    self.sha1 = Some(sha1);
    self
  }

  pub fn sha1(&self) -> Option<&String> {
    self.sha1.as_ref()
  }

  pub fn reset_sha1(&mut self) {
    self.sha1 = None;
  }

  pub fn set_sha256(&mut self, sha256: String) {
    self.sha256 = Some(sha256);
  }

  pub fn with_sha256(mut self, sha256: String) -> PackageFile {
    self.sha256 = Some(sha256);
    self
  }

  pub fn sha256(&self) -> Option<&String> {
    self.sha256.as_ref()
  }

  pub fn reset_sha256(&mut self) {
    self.sha256 = None;
  }

  pub fn set_sha512(&mut self, sha512: String) {
    self.sha512 = Some(sha512);
  }

  pub fn with_sha512(mut self, sha512: String) -> PackageFile {
    self.sha512 = Some(sha512);
    self
  }

  pub fn sha512(&self) -> Option<&String> {
    self.sha512.as_ref()
  }

  pub fn reset_sha512(&mut self) {
    self.sha512 = None;
  }

}



