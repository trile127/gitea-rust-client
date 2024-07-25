/* 
 * Gitea API
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.21.11
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// CreateForkOption : CreateForkOption options for creating a fork

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateForkOption {
  /// name of the forked repository
  #[serde(rename = "name")]
  name: Option<String>,
  /// organization name, if forking into an organization
  #[serde(rename = "organization")]
  organization: Option<String>
}

impl CreateForkOption {
  /// CreateForkOption options for creating a fork
  pub fn new() -> CreateForkOption {
    CreateForkOption {
      name: None,
      organization: None
    }
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> CreateForkOption {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_organization(&mut self, organization: String) {
    self.organization = Some(organization);
  }

  pub fn with_organization(mut self, organization: String) -> CreateForkOption {
    self.organization = Some(organization);
    self
  }

  pub fn organization(&self) -> Option<&String> {
    self.organization.as_ref()
  }

  pub fn reset_organization(&mut self) {
    self.organization = None;
  }

}



