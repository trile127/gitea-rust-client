/* 
 * Gitea API
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.21.11
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DeployKey : DeployKey a deploy key

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeployKey {
  #[serde(rename = "created_at")]
  created_at: Option<String>,
  #[serde(rename = "fingerprint")]
  fingerprint: Option<String>,
  #[serde(rename = "id")]
  id: Option<i64>,
  #[serde(rename = "key")]
  key: Option<String>,
  #[serde(rename = "key_id")]
  key_id: Option<i64>,
  #[serde(rename = "read_only")]
  read_only: Option<bool>,
  #[serde(rename = "repository")]
  repository: Option<::models::Repository>,
  #[serde(rename = "title")]
  title: Option<String>,
  #[serde(rename = "url")]
  url: Option<String>
}

impl DeployKey {
  /// DeployKey a deploy key
  pub fn new() -> DeployKey {
    DeployKey {
      created_at: None,
      fingerprint: None,
      id: None,
      key: None,
      key_id: None,
      read_only: None,
      repository: None,
      title: None,
      url: None
    }
  }

  pub fn set_created_at(&mut self, created_at: String) {
    self.created_at = Some(created_at);
  }

  pub fn with_created_at(mut self, created_at: String) -> DeployKey {
    self.created_at = Some(created_at);
    self
  }

  pub fn created_at(&self) -> Option<&String> {
    self.created_at.as_ref()
  }

  pub fn reset_created_at(&mut self) {
    self.created_at = None;
  }

  pub fn set_fingerprint(&mut self, fingerprint: String) {
    self.fingerprint = Some(fingerprint);
  }

  pub fn with_fingerprint(mut self, fingerprint: String) -> DeployKey {
    self.fingerprint = Some(fingerprint);
    self
  }

  pub fn fingerprint(&self) -> Option<&String> {
    self.fingerprint.as_ref()
  }

  pub fn reset_fingerprint(&mut self) {
    self.fingerprint = None;
  }

  pub fn set_id(&mut self, id: i64) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: i64) -> DeployKey {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&i64> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_key(&mut self, key: String) {
    self.key = Some(key);
  }

  pub fn with_key(mut self, key: String) -> DeployKey {
    self.key = Some(key);
    self
  }

  pub fn key(&self) -> Option<&String> {
    self.key.as_ref()
  }

  pub fn reset_key(&mut self) {
    self.key = None;
  }

  pub fn set_key_id(&mut self, key_id: i64) {
    self.key_id = Some(key_id);
  }

  pub fn with_key_id(mut self, key_id: i64) -> DeployKey {
    self.key_id = Some(key_id);
    self
  }

  pub fn key_id(&self) -> Option<&i64> {
    self.key_id.as_ref()
  }

  pub fn reset_key_id(&mut self) {
    self.key_id = None;
  }

  pub fn set_read_only(&mut self, read_only: bool) {
    self.read_only = Some(read_only);
  }

  pub fn with_read_only(mut self, read_only: bool) -> DeployKey {
    self.read_only = Some(read_only);
    self
  }

  pub fn read_only(&self) -> Option<&bool> {
    self.read_only.as_ref()
  }

  pub fn reset_read_only(&mut self) {
    self.read_only = None;
  }

  pub fn set_repository(&mut self, repository: ::models::Repository) {
    self.repository = Some(repository);
  }

  pub fn with_repository(mut self, repository: ::models::Repository) -> DeployKey {
    self.repository = Some(repository);
    self
  }

  pub fn repository(&self) -> Option<&::models::Repository> {
    self.repository.as_ref()
  }

  pub fn reset_repository(&mut self) {
    self.repository = None;
  }

  pub fn set_title(&mut self, title: String) {
    self.title = Some(title);
  }

  pub fn with_title(mut self, title: String) -> DeployKey {
    self.title = Some(title);
    self
  }

  pub fn title(&self) -> Option<&String> {
    self.title.as_ref()
  }

  pub fn reset_title(&mut self) {
    self.title = None;
  }

  pub fn set_url(&mut self, url: String) {
    self.url = Some(url);
  }

  pub fn with_url(mut self, url: String) -> DeployKey {
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



