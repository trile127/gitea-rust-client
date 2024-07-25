/* 
 * Gitea API
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.21.11
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GpgKeyEmail : GPGKeyEmail an email attached to a GPGKey

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GpgKeyEmail {
  #[serde(rename = "email")]
  email: Option<String>,
  #[serde(rename = "verified")]
  verified: Option<bool>
}

impl GpgKeyEmail {
  /// GPGKeyEmail an email attached to a GPGKey
  pub fn new() -> GpgKeyEmail {
    GpgKeyEmail {
      email: None,
      verified: None
    }
  }

  pub fn set_email(&mut self, email: String) {
    self.email = Some(email);
  }

  pub fn with_email(mut self, email: String) -> GpgKeyEmail {
    self.email = Some(email);
    self
  }

  pub fn email(&self) -> Option<&String> {
    self.email.as_ref()
  }

  pub fn reset_email(&mut self) {
    self.email = None;
  }

  pub fn set_verified(&mut self, verified: bool) {
    self.verified = Some(verified);
  }

  pub fn with_verified(mut self, verified: bool) -> GpgKeyEmail {
    self.verified = Some(verified);
    self
  }

  pub fn verified(&self) -> Option<&bool> {
    self.verified.as_ref()
  }

  pub fn reset_verified(&mut self) {
    self.verified = None;
  }

}


