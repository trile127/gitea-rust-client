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
pub struct Activity {
  #[serde(rename = "act_user")]
  act_user: Option<::models::User>,
  #[serde(rename = "act_user_id")]
  act_user_id: Option<i64>,
  #[serde(rename = "comment")]
  comment: Option<::models::Comment>,
  #[serde(rename = "comment_id")]
  comment_id: Option<i64>,
  #[serde(rename = "content")]
  content: Option<String>,
  #[serde(rename = "created")]
  created: Option<String>,
  #[serde(rename = "id")]
  id: Option<i64>,
  #[serde(rename = "is_private")]
  is_private: Option<bool>,
  #[serde(rename = "op_type")]
  op_type: Option<String>,
  #[serde(rename = "ref_name")]
  ref_name: Option<String>,
  #[serde(rename = "repo")]
  repo: Option<::models::Repository>,
  #[serde(rename = "repo_id")]
  repo_id: Option<i64>,
  #[serde(rename = "user_id")]
  user_id: Option<i64>
}

impl Activity {
  pub fn new() -> Activity {
    Activity {
      act_user: None,
      act_user_id: None,
      comment: None,
      comment_id: None,
      content: None,
      created: None,
      id: None,
      is_private: None,
      op_type: None,
      ref_name: None,
      repo: None,
      repo_id: None,
      user_id: None
    }
  }

  pub fn set_act_user(&mut self, act_user: ::models::User) {
    self.act_user = Some(act_user);
  }

  pub fn with_act_user(mut self, act_user: ::models::User) -> Activity {
    self.act_user = Some(act_user);
    self
  }

  pub fn act_user(&self) -> Option<&::models::User> {
    self.act_user.as_ref()
  }

  pub fn reset_act_user(&mut self) {
    self.act_user = None;
  }

  pub fn set_act_user_id(&mut self, act_user_id: i64) {
    self.act_user_id = Some(act_user_id);
  }

  pub fn with_act_user_id(mut self, act_user_id: i64) -> Activity {
    self.act_user_id = Some(act_user_id);
    self
  }

  pub fn act_user_id(&self) -> Option<&i64> {
    self.act_user_id.as_ref()
  }

  pub fn reset_act_user_id(&mut self) {
    self.act_user_id = None;
  }

  pub fn set_comment(&mut self, comment: ::models::Comment) {
    self.comment = Some(comment);
  }

  pub fn with_comment(mut self, comment: ::models::Comment) -> Activity {
    self.comment = Some(comment);
    self
  }

  pub fn comment(&self) -> Option<&::models::Comment> {
    self.comment.as_ref()
  }

  pub fn reset_comment(&mut self) {
    self.comment = None;
  }

  pub fn set_comment_id(&mut self, comment_id: i64) {
    self.comment_id = Some(comment_id);
  }

  pub fn with_comment_id(mut self, comment_id: i64) -> Activity {
    self.comment_id = Some(comment_id);
    self
  }

  pub fn comment_id(&self) -> Option<&i64> {
    self.comment_id.as_ref()
  }

  pub fn reset_comment_id(&mut self) {
    self.comment_id = None;
  }

  pub fn set_content(&mut self, content: String) {
    self.content = Some(content);
  }

  pub fn with_content(mut self, content: String) -> Activity {
    self.content = Some(content);
    self
  }

  pub fn content(&self) -> Option<&String> {
    self.content.as_ref()
  }

  pub fn reset_content(&mut self) {
    self.content = None;
  }

  pub fn set_created(&mut self, created: String) {
    self.created = Some(created);
  }

  pub fn with_created(mut self, created: String) -> Activity {
    self.created = Some(created);
    self
  }

  pub fn created(&self) -> Option<&String> {
    self.created.as_ref()
  }

  pub fn reset_created(&mut self) {
    self.created = None;
  }

  pub fn set_id(&mut self, id: i64) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: i64) -> Activity {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&i64> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_is_private(&mut self, is_private: bool) {
    self.is_private = Some(is_private);
  }

  pub fn with_is_private(mut self, is_private: bool) -> Activity {
    self.is_private = Some(is_private);
    self
  }

  pub fn is_private(&self) -> Option<&bool> {
    self.is_private.as_ref()
  }

  pub fn reset_is_private(&mut self) {
    self.is_private = None;
  }

  pub fn set_op_type(&mut self, op_type: String) {
    self.op_type = Some(op_type);
  }

  pub fn with_op_type(mut self, op_type: String) -> Activity {
    self.op_type = Some(op_type);
    self
  }

  pub fn op_type(&self) -> Option<&String> {
    self.op_type.as_ref()
  }

  pub fn reset_op_type(&mut self) {
    self.op_type = None;
  }

  pub fn set_ref_name(&mut self, ref_name: String) {
    self.ref_name = Some(ref_name);
  }

  pub fn with_ref_name(mut self, ref_name: String) -> Activity {
    self.ref_name = Some(ref_name);
    self
  }

  pub fn ref_name(&self) -> Option<&String> {
    self.ref_name.as_ref()
  }

  pub fn reset_ref_name(&mut self) {
    self.ref_name = None;
  }

  pub fn set_repo(&mut self, repo: ::models::Repository) {
    self.repo = Some(repo);
  }

  pub fn with_repo(mut self, repo: ::models::Repository) -> Activity {
    self.repo = Some(repo);
    self
  }

  pub fn repo(&self) -> Option<&::models::Repository> {
    self.repo.as_ref()
  }

  pub fn reset_repo(&mut self) {
    self.repo = None;
  }

  pub fn set_repo_id(&mut self, repo_id: i64) {
    self.repo_id = Some(repo_id);
  }

  pub fn with_repo_id(mut self, repo_id: i64) -> Activity {
    self.repo_id = Some(repo_id);
    self
  }

  pub fn repo_id(&self) -> Option<&i64> {
    self.repo_id.as_ref()
  }

  pub fn reset_repo_id(&mut self) {
    self.repo_id = None;
  }

  pub fn set_user_id(&mut self, user_id: i64) {
    self.user_id = Some(user_id);
  }

  pub fn with_user_id(mut self, user_id: i64) -> Activity {
    self.user_id = Some(user_id);
    self
  }

  pub fn user_id(&self) -> Option<&i64> {
    self.user_id.as_ref()
  }

  pub fn reset_user_id(&mut self) {
    self.user_id = None;
  }

}


