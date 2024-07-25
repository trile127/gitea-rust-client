/* 
 * Gitea API
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.21.11
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PullRequest : PullRequest represents a pull request

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PullRequest {
  #[serde(rename = "allow_maintainer_edit")]
  allow_maintainer_edit: Option<bool>,
  #[serde(rename = "assignee")]
  assignee: Option<::models::User>,
  #[serde(rename = "assignees")]
  assignees: Option<Vec<::models::User>>,
  #[serde(rename = "base")]
  base: Option<::models::PrBranchInfo>,
  #[serde(rename = "body")]
  body: Option<String>,
  #[serde(rename = "closed_at")]
  closed_at: Option<String>,
  #[serde(rename = "comments")]
  comments: Option<i64>,
  #[serde(rename = "created_at")]
  created_at: Option<String>,
  #[serde(rename = "diff_url")]
  diff_url: Option<String>,
  #[serde(rename = "due_date")]
  due_date: Option<String>,
  #[serde(rename = "head")]
  head: Option<::models::PrBranchInfo>,
  #[serde(rename = "html_url")]
  html_url: Option<String>,
  #[serde(rename = "id")]
  id: Option<i64>,
  #[serde(rename = "is_locked")]
  is_locked: Option<bool>,
  #[serde(rename = "labels")]
  labels: Option<Vec<::models::Label>>,
  #[serde(rename = "merge_base")]
  merge_base: Option<String>,
  #[serde(rename = "merge_commit_sha")]
  merge_commit_sha: Option<String>,
  #[serde(rename = "mergeable")]
  mergeable: Option<bool>,
  #[serde(rename = "merged")]
  merged: Option<bool>,
  #[serde(rename = "merged_at")]
  merged_at: Option<String>,
  #[serde(rename = "merged_by")]
  merged_by: Option<::models::User>,
  #[serde(rename = "milestone")]
  milestone: Option<::models::Milestone>,
  #[serde(rename = "number")]
  number: Option<i64>,
  #[serde(rename = "patch_url")]
  patch_url: Option<String>,
  #[serde(rename = "pin_order")]
  pin_order: Option<i64>,
  #[serde(rename = "requested_reviewers")]
  requested_reviewers: Option<Vec<::models::User>>,
  #[serde(rename = "state")]
  state: Option<::models::StateType>,
  #[serde(rename = "title")]
  title: Option<String>,
  #[serde(rename = "updated_at")]
  updated_at: Option<String>,
  #[serde(rename = "url")]
  url: Option<String>,
  #[serde(rename = "user")]
  user: Option<::models::User>
}

impl PullRequest {
  /// PullRequest represents a pull request
  pub fn new() -> PullRequest {
    PullRequest {
      allow_maintainer_edit: None,
      assignee: None,
      assignees: None,
      base: None,
      body: None,
      closed_at: None,
      comments: None,
      created_at: None,
      diff_url: None,
      due_date: None,
      head: None,
      html_url: None,
      id: None,
      is_locked: None,
      labels: None,
      merge_base: None,
      merge_commit_sha: None,
      mergeable: None,
      merged: None,
      merged_at: None,
      merged_by: None,
      milestone: None,
      number: None,
      patch_url: None,
      pin_order: None,
      requested_reviewers: None,
      state: None,
      title: None,
      updated_at: None,
      url: None,
      user: None
    }
  }

  pub fn set_allow_maintainer_edit(&mut self, allow_maintainer_edit: bool) {
    self.allow_maintainer_edit = Some(allow_maintainer_edit);
  }

  pub fn with_allow_maintainer_edit(mut self, allow_maintainer_edit: bool) -> PullRequest {
    self.allow_maintainer_edit = Some(allow_maintainer_edit);
    self
  }

  pub fn allow_maintainer_edit(&self) -> Option<&bool> {
    self.allow_maintainer_edit.as_ref()
  }

  pub fn reset_allow_maintainer_edit(&mut self) {
    self.allow_maintainer_edit = None;
  }

  pub fn set_assignee(&mut self, assignee: ::models::User) {
    self.assignee = Some(assignee);
  }

  pub fn with_assignee(mut self, assignee: ::models::User) -> PullRequest {
    self.assignee = Some(assignee);
    self
  }

  pub fn assignee(&self) -> Option<&::models::User> {
    self.assignee.as_ref()
  }

  pub fn reset_assignee(&mut self) {
    self.assignee = None;
  }

  pub fn set_assignees(&mut self, assignees: Vec<::models::User>) {
    self.assignees = Some(assignees);
  }

  pub fn with_assignees(mut self, assignees: Vec<::models::User>) -> PullRequest {
    self.assignees = Some(assignees);
    self
  }

  pub fn assignees(&self) -> Option<&Vec<::models::User>> {
    self.assignees.as_ref()
  }

  pub fn reset_assignees(&mut self) {
    self.assignees = None;
  }

  pub fn set_base(&mut self, base: ::models::PrBranchInfo) {
    self.base = Some(base);
  }

  pub fn with_base(mut self, base: ::models::PrBranchInfo) -> PullRequest {
    self.base = Some(base);
    self
  }

  pub fn base(&self) -> Option<&::models::PrBranchInfo> {
    self.base.as_ref()
  }

  pub fn reset_base(&mut self) {
    self.base = None;
  }

  pub fn set_body(&mut self, body: String) {
    self.body = Some(body);
  }

  pub fn with_body(mut self, body: String) -> PullRequest {
    self.body = Some(body);
    self
  }

  pub fn body(&self) -> Option<&String> {
    self.body.as_ref()
  }

  pub fn reset_body(&mut self) {
    self.body = None;
  }

  pub fn set_closed_at(&mut self, closed_at: String) {
    self.closed_at = Some(closed_at);
  }

  pub fn with_closed_at(mut self, closed_at: String) -> PullRequest {
    self.closed_at = Some(closed_at);
    self
  }

  pub fn closed_at(&self) -> Option<&String> {
    self.closed_at.as_ref()
  }

  pub fn reset_closed_at(&mut self) {
    self.closed_at = None;
  }

  pub fn set_comments(&mut self, comments: i64) {
    self.comments = Some(comments);
  }

  pub fn with_comments(mut self, comments: i64) -> PullRequest {
    self.comments = Some(comments);
    self
  }

  pub fn comments(&self) -> Option<&i64> {
    self.comments.as_ref()
  }

  pub fn reset_comments(&mut self) {
    self.comments = None;
  }

  pub fn set_created_at(&mut self, created_at: String) {
    self.created_at = Some(created_at);
  }

  pub fn with_created_at(mut self, created_at: String) -> PullRequest {
    self.created_at = Some(created_at);
    self
  }

  pub fn created_at(&self) -> Option<&String> {
    self.created_at.as_ref()
  }

  pub fn reset_created_at(&mut self) {
    self.created_at = None;
  }

  pub fn set_diff_url(&mut self, diff_url: String) {
    self.diff_url = Some(diff_url);
  }

  pub fn with_diff_url(mut self, diff_url: String) -> PullRequest {
    self.diff_url = Some(diff_url);
    self
  }

  pub fn diff_url(&self) -> Option<&String> {
    self.diff_url.as_ref()
  }

  pub fn reset_diff_url(&mut self) {
    self.diff_url = None;
  }

  pub fn set_due_date(&mut self, due_date: String) {
    self.due_date = Some(due_date);
  }

  pub fn with_due_date(mut self, due_date: String) -> PullRequest {
    self.due_date = Some(due_date);
    self
  }

  pub fn due_date(&self) -> Option<&String> {
    self.due_date.as_ref()
  }

  pub fn reset_due_date(&mut self) {
    self.due_date = None;
  }

  pub fn set_head(&mut self, head: ::models::PrBranchInfo) {
    self.head = Some(head);
  }

  pub fn with_head(mut self, head: ::models::PrBranchInfo) -> PullRequest {
    self.head = Some(head);
    self
  }

  pub fn head(&self) -> Option<&::models::PrBranchInfo> {
    self.head.as_ref()
  }

  pub fn reset_head(&mut self) {
    self.head = None;
  }

  pub fn set_html_url(&mut self, html_url: String) {
    self.html_url = Some(html_url);
  }

  pub fn with_html_url(mut self, html_url: String) -> PullRequest {
    self.html_url = Some(html_url);
    self
  }

  pub fn html_url(&self) -> Option<&String> {
    self.html_url.as_ref()
  }

  pub fn reset_html_url(&mut self) {
    self.html_url = None;
  }

  pub fn set_id(&mut self, id: i64) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: i64) -> PullRequest {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&i64> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_is_locked(&mut self, is_locked: bool) {
    self.is_locked = Some(is_locked);
  }

  pub fn with_is_locked(mut self, is_locked: bool) -> PullRequest {
    self.is_locked = Some(is_locked);
    self
  }

  pub fn is_locked(&self) -> Option<&bool> {
    self.is_locked.as_ref()
  }

  pub fn reset_is_locked(&mut self) {
    self.is_locked = None;
  }

  pub fn set_labels(&mut self, labels: Vec<::models::Label>) {
    self.labels = Some(labels);
  }

  pub fn with_labels(mut self, labels: Vec<::models::Label>) -> PullRequest {
    self.labels = Some(labels);
    self
  }

  pub fn labels(&self) -> Option<&Vec<::models::Label>> {
    self.labels.as_ref()
  }

  pub fn reset_labels(&mut self) {
    self.labels = None;
  }

  pub fn set_merge_base(&mut self, merge_base: String) {
    self.merge_base = Some(merge_base);
  }

  pub fn with_merge_base(mut self, merge_base: String) -> PullRequest {
    self.merge_base = Some(merge_base);
    self
  }

  pub fn merge_base(&self) -> Option<&String> {
    self.merge_base.as_ref()
  }

  pub fn reset_merge_base(&mut self) {
    self.merge_base = None;
  }

  pub fn set_merge_commit_sha(&mut self, merge_commit_sha: String) {
    self.merge_commit_sha = Some(merge_commit_sha);
  }

  pub fn with_merge_commit_sha(mut self, merge_commit_sha: String) -> PullRequest {
    self.merge_commit_sha = Some(merge_commit_sha);
    self
  }

  pub fn merge_commit_sha(&self) -> Option<&String> {
    self.merge_commit_sha.as_ref()
  }

  pub fn reset_merge_commit_sha(&mut self) {
    self.merge_commit_sha = None;
  }

  pub fn set_mergeable(&mut self, mergeable: bool) {
    self.mergeable = Some(mergeable);
  }

  pub fn with_mergeable(mut self, mergeable: bool) -> PullRequest {
    self.mergeable = Some(mergeable);
    self
  }

  pub fn mergeable(&self) -> Option<&bool> {
    self.mergeable.as_ref()
  }

  pub fn reset_mergeable(&mut self) {
    self.mergeable = None;
  }

  pub fn set_merged(&mut self, merged: bool) {
    self.merged = Some(merged);
  }

  pub fn with_merged(mut self, merged: bool) -> PullRequest {
    self.merged = Some(merged);
    self
  }

  pub fn merged(&self) -> Option<&bool> {
    self.merged.as_ref()
  }

  pub fn reset_merged(&mut self) {
    self.merged = None;
  }

  pub fn set_merged_at(&mut self, merged_at: String) {
    self.merged_at = Some(merged_at);
  }

  pub fn with_merged_at(mut self, merged_at: String) -> PullRequest {
    self.merged_at = Some(merged_at);
    self
  }

  pub fn merged_at(&self) -> Option<&String> {
    self.merged_at.as_ref()
  }

  pub fn reset_merged_at(&mut self) {
    self.merged_at = None;
  }

  pub fn set_merged_by(&mut self, merged_by: ::models::User) {
    self.merged_by = Some(merged_by);
  }

  pub fn with_merged_by(mut self, merged_by: ::models::User) -> PullRequest {
    self.merged_by = Some(merged_by);
    self
  }

  pub fn merged_by(&self) -> Option<&::models::User> {
    self.merged_by.as_ref()
  }

  pub fn reset_merged_by(&mut self) {
    self.merged_by = None;
  }

  pub fn set_milestone(&mut self, milestone: ::models::Milestone) {
    self.milestone = Some(milestone);
  }

  pub fn with_milestone(mut self, milestone: ::models::Milestone) -> PullRequest {
    self.milestone = Some(milestone);
    self
  }

  pub fn milestone(&self) -> Option<&::models::Milestone> {
    self.milestone.as_ref()
  }

  pub fn reset_milestone(&mut self) {
    self.milestone = None;
  }

  pub fn set_number(&mut self, number: i64) {
    self.number = Some(number);
  }

  pub fn with_number(mut self, number: i64) -> PullRequest {
    self.number = Some(number);
    self
  }

  pub fn number(&self) -> Option<&i64> {
    self.number.as_ref()
  }

  pub fn reset_number(&mut self) {
    self.number = None;
  }

  pub fn set_patch_url(&mut self, patch_url: String) {
    self.patch_url = Some(patch_url);
  }

  pub fn with_patch_url(mut self, patch_url: String) -> PullRequest {
    self.patch_url = Some(patch_url);
    self
  }

  pub fn patch_url(&self) -> Option<&String> {
    self.patch_url.as_ref()
  }

  pub fn reset_patch_url(&mut self) {
    self.patch_url = None;
  }

  pub fn set_pin_order(&mut self, pin_order: i64) {
    self.pin_order = Some(pin_order);
  }

  pub fn with_pin_order(mut self, pin_order: i64) -> PullRequest {
    self.pin_order = Some(pin_order);
    self
  }

  pub fn pin_order(&self) -> Option<&i64> {
    self.pin_order.as_ref()
  }

  pub fn reset_pin_order(&mut self) {
    self.pin_order = None;
  }

  pub fn set_requested_reviewers(&mut self, requested_reviewers: Vec<::models::User>) {
    self.requested_reviewers = Some(requested_reviewers);
  }

  pub fn with_requested_reviewers(mut self, requested_reviewers: Vec<::models::User>) -> PullRequest {
    self.requested_reviewers = Some(requested_reviewers);
    self
  }

  pub fn requested_reviewers(&self) -> Option<&Vec<::models::User>> {
    self.requested_reviewers.as_ref()
  }

  pub fn reset_requested_reviewers(&mut self) {
    self.requested_reviewers = None;
  }

  pub fn set_state(&mut self, state: ::models::StateType) {
    self.state = Some(state);
  }

  pub fn with_state(mut self, state: ::models::StateType) -> PullRequest {
    self.state = Some(state);
    self
  }

  pub fn state(&self) -> Option<&::models::StateType> {
    self.state.as_ref()
  }

  pub fn reset_state(&mut self) {
    self.state = None;
  }

  pub fn set_title(&mut self, title: String) {
    self.title = Some(title);
  }

  pub fn with_title(mut self, title: String) -> PullRequest {
    self.title = Some(title);
    self
  }

  pub fn title(&self) -> Option<&String> {
    self.title.as_ref()
  }

  pub fn reset_title(&mut self) {
    self.title = None;
  }

  pub fn set_updated_at(&mut self, updated_at: String) {
    self.updated_at = Some(updated_at);
  }

  pub fn with_updated_at(mut self, updated_at: String) -> PullRequest {
    self.updated_at = Some(updated_at);
    self
  }

  pub fn updated_at(&self) -> Option<&String> {
    self.updated_at.as_ref()
  }

  pub fn reset_updated_at(&mut self) {
    self.updated_at = None;
  }

  pub fn set_url(&mut self, url: String) {
    self.url = Some(url);
  }

  pub fn with_url(mut self, url: String) -> PullRequest {
    self.url = Some(url);
    self
  }

  pub fn url(&self) -> Option<&String> {
    self.url.as_ref()
  }

  pub fn reset_url(&mut self) {
    self.url = None;
  }

  pub fn set_user(&mut self, user: ::models::User) {
    self.user = Some(user);
  }

  pub fn with_user(mut self, user: ::models::User) -> PullRequest {
    self.user = Some(user);
    self
  }

  pub fn user(&self) -> Option<&::models::User> {
    self.user.as_ref()
  }

  pub fn reset_user(&mut self) {
    self.user = None;
  }

}



