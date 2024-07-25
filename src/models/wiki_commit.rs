/* 
 * Gitea API
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.21.11
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// WikiCommit : WikiCommit page commit/revision

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct WikiCommit {
  #[serde(rename = "author")]
  author: Option<::models::CommitUser>,
  #[serde(rename = "commiter")]
  commiter: Option<::models::CommitUser>,
  #[serde(rename = "message")]
  message: Option<String>,
  #[serde(rename = "sha")]
  sha: Option<String>
}

impl WikiCommit {
  /// WikiCommit page commit/revision
  pub fn new() -> WikiCommit {
    WikiCommit {
      author: None,
      commiter: None,
      message: None,
      sha: None
    }
  }

  pub fn set_author(&mut self, author: ::models::CommitUser) {
    self.author = Some(author);
  }

  pub fn with_author(mut self, author: ::models::CommitUser) -> WikiCommit {
    self.author = Some(author);
    self
  }

  pub fn author(&self) -> Option<&::models::CommitUser> {
    self.author.as_ref()
  }

  pub fn reset_author(&mut self) {
    self.author = None;
  }

  pub fn set_commiter(&mut self, commiter: ::models::CommitUser) {
    self.commiter = Some(commiter);
  }

  pub fn with_commiter(mut self, commiter: ::models::CommitUser) -> WikiCommit {
    self.commiter = Some(commiter);
    self
  }

  pub fn commiter(&self) -> Option<&::models::CommitUser> {
    self.commiter.as_ref()
  }

  pub fn reset_commiter(&mut self) {
    self.commiter = None;
  }

  pub fn set_message(&mut self, message: String) {
    self.message = Some(message);
  }

  pub fn with_message(mut self, message: String) -> WikiCommit {
    self.message = Some(message);
    self
  }

  pub fn message(&self) -> Option<&String> {
    self.message.as_ref()
  }

  pub fn reset_message(&mut self) {
    self.message = None;
  }

  pub fn set_sha(&mut self, sha: String) {
    self.sha = Some(sha);
  }

  pub fn with_sha(mut self, sha: String) -> WikiCommit {
    self.sha = Some(sha);
    self
  }

  pub fn sha(&self) -> Option<&String> {
    self.sha.as_ref()
  }

  pub fn reset_sha(&mut self) {
    self.sha = None;
  }

}


