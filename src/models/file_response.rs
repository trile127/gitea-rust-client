/* 
 * Gitea API
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.21.11
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// FileResponse : FileResponse contains information about a repo's file

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileResponse {
  #[serde(rename = "commit")]
  commit: Option<::models::FileCommitResponse>,
  #[serde(rename = "content")]
  content: Option<::models::ContentsResponse>,
  #[serde(rename = "verification")]
  verification: Option<::models::PayloadCommitVerification>
}

impl FileResponse {
  /// FileResponse contains information about a repo's file
  pub fn new() -> FileResponse {
    FileResponse {
      commit: None,
      content: None,
      verification: None
    }
  }

  pub fn set_commit(&mut self, commit: ::models::FileCommitResponse) {
    self.commit = Some(commit);
  }

  pub fn with_commit(mut self, commit: ::models::FileCommitResponse) -> FileResponse {
    self.commit = Some(commit);
    self
  }

  pub fn commit(&self) -> Option<&::models::FileCommitResponse> {
    self.commit.as_ref()
  }

  pub fn reset_commit(&mut self) {
    self.commit = None;
  }

  pub fn set_content(&mut self, content: ::models::ContentsResponse) {
    self.content = Some(content);
  }

  pub fn with_content(mut self, content: ::models::ContentsResponse) -> FileResponse {
    self.content = Some(content);
    self
  }

  pub fn content(&self) -> Option<&::models::ContentsResponse> {
    self.content.as_ref()
  }

  pub fn reset_content(&mut self) {
    self.content = None;
  }

  pub fn set_verification(&mut self, verification: ::models::PayloadCommitVerification) {
    self.verification = Some(verification);
  }

  pub fn with_verification(mut self, verification: ::models::PayloadCommitVerification) -> FileResponse {
    self.verification = Some(verification);
    self
  }

  pub fn verification(&self) -> Option<&::models::PayloadCommitVerification> {
    self.verification.as_ref()
  }

  pub fn reset_verification(&mut self) {
    self.verification = None;
  }

}


