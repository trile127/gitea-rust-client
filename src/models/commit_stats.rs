/* 
 * Gitea API
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.21.11
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// CommitStats : CommitStats is statistics for a RepoCommit

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitStats {
  #[serde(rename = "additions")]
  additions: Option<i64>,
  #[serde(rename = "deletions")]
  deletions: Option<i64>,
  #[serde(rename = "total")]
  total: Option<i64>
}

impl CommitStats {
  /// CommitStats is statistics for a RepoCommit
  pub fn new() -> CommitStats {
    CommitStats {
      additions: None,
      deletions: None,
      total: None
    }
  }

  pub fn set_additions(&mut self, additions: i64) {
    self.additions = Some(additions);
  }

  pub fn with_additions(mut self, additions: i64) -> CommitStats {
    self.additions = Some(additions);
    self
  }

  pub fn additions(&self) -> Option<&i64> {
    self.additions.as_ref()
  }

  pub fn reset_additions(&mut self) {
    self.additions = None;
  }

  pub fn set_deletions(&mut self, deletions: i64) {
    self.deletions = Some(deletions);
  }

  pub fn with_deletions(mut self, deletions: i64) -> CommitStats {
    self.deletions = Some(deletions);
    self
  }

  pub fn deletions(&self) -> Option<&i64> {
    self.deletions.as_ref()
  }

  pub fn reset_deletions(&mut self) {
    self.deletions = None;
  }

  pub fn set_total(&mut self, total: i64) {
    self.total = Some(total);
  }

  pub fn with_total(mut self, total: i64) -> CommitStats {
    self.total = Some(total);
    self
  }

  pub fn total(&self) -> Option<&i64> {
    self.total.as_ref()
  }

  pub fn reset_total(&mut self) {
    self.total = None;
  }

}



