/* 
 * Gitea API
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.21.11
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// NodeInfoUsageUsers : NodeInfoUsageUsers contains statistics about the users of this server

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeInfoUsageUsers {
  #[serde(rename = "activeHalfyear")]
  active_halfyear: Option<i64>,
  #[serde(rename = "activeMonth")]
  active_month: Option<i64>,
  #[serde(rename = "total")]
  total: Option<i64>
}

impl NodeInfoUsageUsers {
  /// NodeInfoUsageUsers contains statistics about the users of this server
  pub fn new() -> NodeInfoUsageUsers {
    NodeInfoUsageUsers {
      active_halfyear: None,
      active_month: None,
      total: None
    }
  }

  pub fn set_active_halfyear(&mut self, active_halfyear: i64) {
    self.active_halfyear = Some(active_halfyear);
  }

  pub fn with_active_halfyear(mut self, active_halfyear: i64) -> NodeInfoUsageUsers {
    self.active_halfyear = Some(active_halfyear);
    self
  }

  pub fn active_halfyear(&self) -> Option<&i64> {
    self.active_halfyear.as_ref()
  }

  pub fn reset_active_halfyear(&mut self) {
    self.active_halfyear = None;
  }

  pub fn set_active_month(&mut self, active_month: i64) {
    self.active_month = Some(active_month);
  }

  pub fn with_active_month(mut self, active_month: i64) -> NodeInfoUsageUsers {
    self.active_month = Some(active_month);
    self
  }

  pub fn active_month(&self) -> Option<&i64> {
    self.active_month.as_ref()
  }

  pub fn reset_active_month(&mut self) {
    self.active_month = None;
  }

  pub fn set_total(&mut self, total: i64) {
    self.total = Some(total);
  }

  pub fn with_total(mut self, total: i64) -> NodeInfoUsageUsers {
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


