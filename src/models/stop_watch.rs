/* 
 * Gitea API
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.21.11
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// StopWatch : StopWatch represent a running stopwatch

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StopWatch {
  #[serde(rename = "created")]
  created: Option<String>,
  #[serde(rename = "duration")]
  duration: Option<String>,
  #[serde(rename = "issue_index")]
  issue_index: Option<i64>,
  #[serde(rename = "issue_title")]
  issue_title: Option<String>,
  #[serde(rename = "repo_name")]
  repo_name: Option<String>,
  #[serde(rename = "repo_owner_name")]
  repo_owner_name: Option<String>,
  #[serde(rename = "seconds")]
  seconds: Option<i64>
}

impl StopWatch {
  /// StopWatch represent a running stopwatch
  pub fn new() -> StopWatch {
    StopWatch {
      created: None,
      duration: None,
      issue_index: None,
      issue_title: None,
      repo_name: None,
      repo_owner_name: None,
      seconds: None
    }
  }

  pub fn set_created(&mut self, created: String) {
    self.created = Some(created);
  }

  pub fn with_created(mut self, created: String) -> StopWatch {
    self.created = Some(created);
    self
  }

  pub fn created(&self) -> Option<&String> {
    self.created.as_ref()
  }

  pub fn reset_created(&mut self) {
    self.created = None;
  }

  pub fn set_duration(&mut self, duration: String) {
    self.duration = Some(duration);
  }

  pub fn with_duration(mut self, duration: String) -> StopWatch {
    self.duration = Some(duration);
    self
  }

  pub fn duration(&self) -> Option<&String> {
    self.duration.as_ref()
  }

  pub fn reset_duration(&mut self) {
    self.duration = None;
  }

  pub fn set_issue_index(&mut self, issue_index: i64) {
    self.issue_index = Some(issue_index);
  }

  pub fn with_issue_index(mut self, issue_index: i64) -> StopWatch {
    self.issue_index = Some(issue_index);
    self
  }

  pub fn issue_index(&self) -> Option<&i64> {
    self.issue_index.as_ref()
  }

  pub fn reset_issue_index(&mut self) {
    self.issue_index = None;
  }

  pub fn set_issue_title(&mut self, issue_title: String) {
    self.issue_title = Some(issue_title);
  }

  pub fn with_issue_title(mut self, issue_title: String) -> StopWatch {
    self.issue_title = Some(issue_title);
    self
  }

  pub fn issue_title(&self) -> Option<&String> {
    self.issue_title.as_ref()
  }

  pub fn reset_issue_title(&mut self) {
    self.issue_title = None;
  }

  pub fn set_repo_name(&mut self, repo_name: String) {
    self.repo_name = Some(repo_name);
  }

  pub fn with_repo_name(mut self, repo_name: String) -> StopWatch {
    self.repo_name = Some(repo_name);
    self
  }

  pub fn repo_name(&self) -> Option<&String> {
    self.repo_name.as_ref()
  }

  pub fn reset_repo_name(&mut self) {
    self.repo_name = None;
  }

  pub fn set_repo_owner_name(&mut self, repo_owner_name: String) {
    self.repo_owner_name = Some(repo_owner_name);
  }

  pub fn with_repo_owner_name(mut self, repo_owner_name: String) -> StopWatch {
    self.repo_owner_name = Some(repo_owner_name);
    self
  }

  pub fn repo_owner_name(&self) -> Option<&String> {
    self.repo_owner_name.as_ref()
  }

  pub fn reset_repo_owner_name(&mut self) {
    self.repo_owner_name = None;
  }

  pub fn set_seconds(&mut self, seconds: i64) {
    self.seconds = Some(seconds);
  }

  pub fn with_seconds(mut self, seconds: i64) -> StopWatch {
    self.seconds = Some(seconds);
    self
  }

  pub fn seconds(&self) -> Option<&i64> {
    self.seconds.as_ref()
  }

  pub fn reset_seconds(&mut self) {
    self.seconds = None;
  }

}



