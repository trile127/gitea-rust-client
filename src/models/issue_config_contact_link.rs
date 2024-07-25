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
pub struct IssueConfigContactLink {
  #[serde(rename = "about")]
  about: Option<String>,
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "url")]
  url: Option<String>
}

impl IssueConfigContactLink {
  pub fn new() -> IssueConfigContactLink {
    IssueConfigContactLink {
      about: None,
      name: None,
      url: None
    }
  }

  pub fn set_about(&mut self, about: String) {
    self.about = Some(about);
  }

  pub fn with_about(mut self, about: String) -> IssueConfigContactLink {
    self.about = Some(about);
    self
  }

  pub fn about(&self) -> Option<&String> {
    self.about.as_ref()
  }

  pub fn reset_about(&mut self) {
    self.about = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> IssueConfigContactLink {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_url(&mut self, url: String) {
    self.url = Some(url);
  }

  pub fn with_url(mut self, url: String) -> IssueConfigContactLink {
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



