/* 
 * Gitea API
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.21.11
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// EditReleaseOption : EditReleaseOption options when editing a release

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EditReleaseOption {
  #[serde(rename = "body")]
  body: Option<String>,
  #[serde(rename = "draft")]
  draft: Option<bool>,
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "prerelease")]
  prerelease: Option<bool>,
  #[serde(rename = "tag_name")]
  tag_name: Option<String>,
  #[serde(rename = "target_commitish")]
  target_commitish: Option<String>
}

impl EditReleaseOption {
  /// EditReleaseOption options when editing a release
  pub fn new() -> EditReleaseOption {
    EditReleaseOption {
      body: None,
      draft: None,
      name: None,
      prerelease: None,
      tag_name: None,
      target_commitish: None
    }
  }

  pub fn set_body(&mut self, body: String) {
    self.body = Some(body);
  }

  pub fn with_body(mut self, body: String) -> EditReleaseOption {
    self.body = Some(body);
    self
  }

  pub fn body(&self) -> Option<&String> {
    self.body.as_ref()
  }

  pub fn reset_body(&mut self) {
    self.body = None;
  }

  pub fn set_draft(&mut self, draft: bool) {
    self.draft = Some(draft);
  }

  pub fn with_draft(mut self, draft: bool) -> EditReleaseOption {
    self.draft = Some(draft);
    self
  }

  pub fn draft(&self) -> Option<&bool> {
    self.draft.as_ref()
  }

  pub fn reset_draft(&mut self) {
    self.draft = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> EditReleaseOption {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_prerelease(&mut self, prerelease: bool) {
    self.prerelease = Some(prerelease);
  }

  pub fn with_prerelease(mut self, prerelease: bool) -> EditReleaseOption {
    self.prerelease = Some(prerelease);
    self
  }

  pub fn prerelease(&self) -> Option<&bool> {
    self.prerelease.as_ref()
  }

  pub fn reset_prerelease(&mut self) {
    self.prerelease = None;
  }

  pub fn set_tag_name(&mut self, tag_name: String) {
    self.tag_name = Some(tag_name);
  }

  pub fn with_tag_name(mut self, tag_name: String) -> EditReleaseOption {
    self.tag_name = Some(tag_name);
    self
  }

  pub fn tag_name(&self) -> Option<&String> {
    self.tag_name.as_ref()
  }

  pub fn reset_tag_name(&mut self) {
    self.tag_name = None;
  }

  pub fn set_target_commitish(&mut self, target_commitish: String) {
    self.target_commitish = Some(target_commitish);
  }

  pub fn with_target_commitish(mut self, target_commitish: String) -> EditReleaseOption {
    self.target_commitish = Some(target_commitish);
    self
  }

  pub fn target_commitish(&self) -> Option<&String> {
    self.target_commitish.as_ref()
  }

  pub fn reset_target_commitish(&mut self) {
    self.target_commitish = None;
  }

}



