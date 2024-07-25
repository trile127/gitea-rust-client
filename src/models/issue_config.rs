/* 
 * Gitea API
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.21.11
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IssueConfig {
  #[serde(rename = "blank_issues_enabled")]
  blank_issues_enabled: Option<bool>,
  #[serde(rename = "contact_links")]
  contact_links: Option<Vec<::models::IssueConfigContactLink>>
}

impl IssueConfig {
  pub fn new() -> IssueConfig {
    IssueConfig {
      blank_issues_enabled: None,
      contact_links: None
    }
  }

  pub fn set_blank_issues_enabled(&mut self, blank_issues_enabled: bool) {
    self.blank_issues_enabled = Some(blank_issues_enabled);
  }

  pub fn with_blank_issues_enabled(mut self, blank_issues_enabled: bool) -> IssueConfig {
    self.blank_issues_enabled = Some(blank_issues_enabled);
    self
  }

  pub fn blank_issues_enabled(&self) -> Option<&bool> {
    self.blank_issues_enabled.as_ref()
  }

  pub fn reset_blank_issues_enabled(&mut self) {
    self.blank_issues_enabled = None;
  }

  pub fn set_contact_links(&mut self, contact_links: Vec<::models::IssueConfigContactLink>) {
    self.contact_links = Some(contact_links);
  }

  pub fn with_contact_links(mut self, contact_links: Vec<::models::IssueConfigContactLink>) -> IssueConfig {
    self.contact_links = Some(contact_links);
    self
  }

  pub fn contact_links(&self) -> Option<&Vec<::models::IssueConfigContactLink>> {
    self.contact_links.as_ref()
  }

  pub fn reset_contact_links(&mut self) {
    self.contact_links = None;
  }

}


