/* 
 * Gitea API
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.21.11
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// EditUserOption : EditUserOption edit user options

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EditUserOption {
  #[serde(rename = "active")]
  active: Option<bool>,
  #[serde(rename = "admin")]
  admin: Option<bool>,
  #[serde(rename = "allow_create_organization")]
  allow_create_organization: Option<bool>,
  #[serde(rename = "allow_git_hook")]
  allow_git_hook: Option<bool>,
  #[serde(rename = "allow_import_local")]
  allow_import_local: Option<bool>,
  #[serde(rename = "description")]
  description: Option<String>,
  #[serde(rename = "email")]
  email: Option<String>,
  #[serde(rename = "full_name")]
  full_name: Option<String>,
  #[serde(rename = "location")]
  location: Option<String>,
  #[serde(rename = "login_name")]
  login_name: String,
  #[serde(rename = "max_repo_creation")]
  max_repo_creation: Option<i64>,
  #[serde(rename = "must_change_password")]
  must_change_password: Option<bool>,
  #[serde(rename = "password")]
  password: Option<String>,
  #[serde(rename = "prohibit_login")]
  prohibit_login: Option<bool>,
  #[serde(rename = "restricted")]
  restricted: Option<bool>,
  #[serde(rename = "source_id")]
  source_id: i64,
  #[serde(rename = "visibility")]
  visibility: Option<String>,
  #[serde(rename = "website")]
  website: Option<String>
}

impl EditUserOption {
  /// EditUserOption edit user options
  pub fn new(login_name: String, source_id: i64) -> EditUserOption {
    EditUserOption {
      active: None,
      admin: None,
      allow_create_organization: None,
      allow_git_hook: None,
      allow_import_local: None,
      description: None,
      email: None,
      full_name: None,
      location: None,
      login_name: login_name,
      max_repo_creation: None,
      must_change_password: None,
      password: None,
      prohibit_login: None,
      restricted: None,
      source_id: source_id,
      visibility: None,
      website: None
    }
  }

  pub fn set_active(&mut self, active: bool) {
    self.active = Some(active);
  }

  pub fn with_active(mut self, active: bool) -> EditUserOption {
    self.active = Some(active);
    self
  }

  pub fn active(&self) -> Option<&bool> {
    self.active.as_ref()
  }

  pub fn reset_active(&mut self) {
    self.active = None;
  }

  pub fn set_admin(&mut self, admin: bool) {
    self.admin = Some(admin);
  }

  pub fn with_admin(mut self, admin: bool) -> EditUserOption {
    self.admin = Some(admin);
    self
  }

  pub fn admin(&self) -> Option<&bool> {
    self.admin.as_ref()
  }

  pub fn reset_admin(&mut self) {
    self.admin = None;
  }

  pub fn set_allow_create_organization(&mut self, allow_create_organization: bool) {
    self.allow_create_organization = Some(allow_create_organization);
  }

  pub fn with_allow_create_organization(mut self, allow_create_organization: bool) -> EditUserOption {
    self.allow_create_organization = Some(allow_create_organization);
    self
  }

  pub fn allow_create_organization(&self) -> Option<&bool> {
    self.allow_create_organization.as_ref()
  }

  pub fn reset_allow_create_organization(&mut self) {
    self.allow_create_organization = None;
  }

  pub fn set_allow_git_hook(&mut self, allow_git_hook: bool) {
    self.allow_git_hook = Some(allow_git_hook);
  }

  pub fn with_allow_git_hook(mut self, allow_git_hook: bool) -> EditUserOption {
    self.allow_git_hook = Some(allow_git_hook);
    self
  }

  pub fn allow_git_hook(&self) -> Option<&bool> {
    self.allow_git_hook.as_ref()
  }

  pub fn reset_allow_git_hook(&mut self) {
    self.allow_git_hook = None;
  }

  pub fn set_allow_import_local(&mut self, allow_import_local: bool) {
    self.allow_import_local = Some(allow_import_local);
  }

  pub fn with_allow_import_local(mut self, allow_import_local: bool) -> EditUserOption {
    self.allow_import_local = Some(allow_import_local);
    self
  }

  pub fn allow_import_local(&self) -> Option<&bool> {
    self.allow_import_local.as_ref()
  }

  pub fn reset_allow_import_local(&mut self) {
    self.allow_import_local = None;
  }

  pub fn set_description(&mut self, description: String) {
    self.description = Some(description);
  }

  pub fn with_description(mut self, description: String) -> EditUserOption {
    self.description = Some(description);
    self
  }

  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }

  pub fn reset_description(&mut self) {
    self.description = None;
  }

  pub fn set_email(&mut self, email: String) {
    self.email = Some(email);
  }

  pub fn with_email(mut self, email: String) -> EditUserOption {
    self.email = Some(email);
    self
  }

  pub fn email(&self) -> Option<&String> {
    self.email.as_ref()
  }

  pub fn reset_email(&mut self) {
    self.email = None;
  }

  pub fn set_full_name(&mut self, full_name: String) {
    self.full_name = Some(full_name);
  }

  pub fn with_full_name(mut self, full_name: String) -> EditUserOption {
    self.full_name = Some(full_name);
    self
  }

  pub fn full_name(&self) -> Option<&String> {
    self.full_name.as_ref()
  }

  pub fn reset_full_name(&mut self) {
    self.full_name = None;
  }

  pub fn set_location(&mut self, location: String) {
    self.location = Some(location);
  }

  pub fn with_location(mut self, location: String) -> EditUserOption {
    self.location = Some(location);
    self
  }

  pub fn location(&self) -> Option<&String> {
    self.location.as_ref()
  }

  pub fn reset_location(&mut self) {
    self.location = None;
  }

  pub fn set_login_name(&mut self, login_name: String) {
    self.login_name = login_name;
  }

  pub fn with_login_name(mut self, login_name: String) -> EditUserOption {
    self.login_name = login_name;
    self
  }

  pub fn login_name(&self) -> &String {
    &self.login_name
  }


  pub fn set_max_repo_creation(&mut self, max_repo_creation: i64) {
    self.max_repo_creation = Some(max_repo_creation);
  }

  pub fn with_max_repo_creation(mut self, max_repo_creation: i64) -> EditUserOption {
    self.max_repo_creation = Some(max_repo_creation);
    self
  }

  pub fn max_repo_creation(&self) -> Option<&i64> {
    self.max_repo_creation.as_ref()
  }

  pub fn reset_max_repo_creation(&mut self) {
    self.max_repo_creation = None;
  }

  pub fn set_must_change_password(&mut self, must_change_password: bool) {
    self.must_change_password = Some(must_change_password);
  }

  pub fn with_must_change_password(mut self, must_change_password: bool) -> EditUserOption {
    self.must_change_password = Some(must_change_password);
    self
  }

  pub fn must_change_password(&self) -> Option<&bool> {
    self.must_change_password.as_ref()
  }

  pub fn reset_must_change_password(&mut self) {
    self.must_change_password = None;
  }

  pub fn set_password(&mut self, password: String) {
    self.password = Some(password);
  }

  pub fn with_password(mut self, password: String) -> EditUserOption {
    self.password = Some(password);
    self
  }

  pub fn password(&self) -> Option<&String> {
    self.password.as_ref()
  }

  pub fn reset_password(&mut self) {
    self.password = None;
  }

  pub fn set_prohibit_login(&mut self, prohibit_login: bool) {
    self.prohibit_login = Some(prohibit_login);
  }

  pub fn with_prohibit_login(mut self, prohibit_login: bool) -> EditUserOption {
    self.prohibit_login = Some(prohibit_login);
    self
  }

  pub fn prohibit_login(&self) -> Option<&bool> {
    self.prohibit_login.as_ref()
  }

  pub fn reset_prohibit_login(&mut self) {
    self.prohibit_login = None;
  }

  pub fn set_restricted(&mut self, restricted: bool) {
    self.restricted = Some(restricted);
  }

  pub fn with_restricted(mut self, restricted: bool) -> EditUserOption {
    self.restricted = Some(restricted);
    self
  }

  pub fn restricted(&self) -> Option<&bool> {
    self.restricted.as_ref()
  }

  pub fn reset_restricted(&mut self) {
    self.restricted = None;
  }

  pub fn set_source_id(&mut self, source_id: i64) {
    self.source_id = source_id;
  }

  pub fn with_source_id(mut self, source_id: i64) -> EditUserOption {
    self.source_id = source_id;
    self
  }

  pub fn source_id(&self) -> &i64 {
    &self.source_id
  }


  pub fn set_visibility(&mut self, visibility: String) {
    self.visibility = Some(visibility);
  }

  pub fn with_visibility(mut self, visibility: String) -> EditUserOption {
    self.visibility = Some(visibility);
    self
  }

  pub fn visibility(&self) -> Option<&String> {
    self.visibility.as_ref()
  }

  pub fn reset_visibility(&mut self) {
    self.visibility = None;
  }

  pub fn set_website(&mut self, website: String) {
    self.website = Some(website);
  }

  pub fn with_website(mut self, website: String) -> EditUserOption {
    self.website = Some(website);
    self
  }

  pub fn website(&self) -> Option<&String> {
    self.website.as_ref()
  }

  pub fn reset_website(&mut self) {
    self.website = None;
  }

}



