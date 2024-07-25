/* 
 * Gitea API
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.21.11
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// CreateBranchProtectionOption : CreateBranchProtectionOption options for creating a branch protection

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBranchProtectionOption {
  #[serde(rename = "approvals_whitelist_teams")]
  approvals_whitelist_teams: Option<Vec<String>>,
  #[serde(rename = "approvals_whitelist_username")]
  approvals_whitelist_username: Option<Vec<String>>,
  #[serde(rename = "block_on_official_review_requests")]
  block_on_official_review_requests: Option<bool>,
  #[serde(rename = "block_on_outdated_branch")]
  block_on_outdated_branch: Option<bool>,
  #[serde(rename = "block_on_rejected_reviews")]
  block_on_rejected_reviews: Option<bool>,
  /// Deprecated: true
  #[serde(rename = "branch_name")]
  branch_name: Option<String>,
  #[serde(rename = "dismiss_stale_approvals")]
  dismiss_stale_approvals: Option<bool>,
  #[serde(rename = "enable_approvals_whitelist")]
  enable_approvals_whitelist: Option<bool>,
  #[serde(rename = "enable_merge_whitelist")]
  enable_merge_whitelist: Option<bool>,
  #[serde(rename = "enable_push")]
  enable_push: Option<bool>,
  #[serde(rename = "enable_push_whitelist")]
  enable_push_whitelist: Option<bool>,
  #[serde(rename = "enable_status_check")]
  enable_status_check: Option<bool>,
  #[serde(rename = "merge_whitelist_teams")]
  merge_whitelist_teams: Option<Vec<String>>,
  #[serde(rename = "merge_whitelist_usernames")]
  merge_whitelist_usernames: Option<Vec<String>>,
  #[serde(rename = "protected_file_patterns")]
  protected_file_patterns: Option<String>,
  #[serde(rename = "push_whitelist_deploy_keys")]
  push_whitelist_deploy_keys: Option<bool>,
  #[serde(rename = "push_whitelist_teams")]
  push_whitelist_teams: Option<Vec<String>>,
  #[serde(rename = "push_whitelist_usernames")]
  push_whitelist_usernames: Option<Vec<String>>,
  #[serde(rename = "require_signed_commits")]
  require_signed_commits: Option<bool>,
  #[serde(rename = "required_approvals")]
  required_approvals: Option<i64>,
  #[serde(rename = "rule_name")]
  rule_name: Option<String>,
  #[serde(rename = "status_check_contexts")]
  status_check_contexts: Option<Vec<String>>,
  #[serde(rename = "unprotected_file_patterns")]
  unprotected_file_patterns: Option<String>
}

impl CreateBranchProtectionOption {
  /// CreateBranchProtectionOption options for creating a branch protection
  pub fn new() -> CreateBranchProtectionOption {
    CreateBranchProtectionOption {
      approvals_whitelist_teams: None,
      approvals_whitelist_username: None,
      block_on_official_review_requests: None,
      block_on_outdated_branch: None,
      block_on_rejected_reviews: None,
      branch_name: None,
      dismiss_stale_approvals: None,
      enable_approvals_whitelist: None,
      enable_merge_whitelist: None,
      enable_push: None,
      enable_push_whitelist: None,
      enable_status_check: None,
      merge_whitelist_teams: None,
      merge_whitelist_usernames: None,
      protected_file_patterns: None,
      push_whitelist_deploy_keys: None,
      push_whitelist_teams: None,
      push_whitelist_usernames: None,
      require_signed_commits: None,
      required_approvals: None,
      rule_name: None,
      status_check_contexts: None,
      unprotected_file_patterns: None
    }
  }

  pub fn set_approvals_whitelist_teams(&mut self, approvals_whitelist_teams: Vec<String>) {
    self.approvals_whitelist_teams = Some(approvals_whitelist_teams);
  }

  pub fn with_approvals_whitelist_teams(mut self, approvals_whitelist_teams: Vec<String>) -> CreateBranchProtectionOption {
    self.approvals_whitelist_teams = Some(approvals_whitelist_teams);
    self
  }

  pub fn approvals_whitelist_teams(&self) -> Option<&Vec<String>> {
    self.approvals_whitelist_teams.as_ref()
  }

  pub fn reset_approvals_whitelist_teams(&mut self) {
    self.approvals_whitelist_teams = None;
  }

  pub fn set_approvals_whitelist_username(&mut self, approvals_whitelist_username: Vec<String>) {
    self.approvals_whitelist_username = Some(approvals_whitelist_username);
  }

  pub fn with_approvals_whitelist_username(mut self, approvals_whitelist_username: Vec<String>) -> CreateBranchProtectionOption {
    self.approvals_whitelist_username = Some(approvals_whitelist_username);
    self
  }

  pub fn approvals_whitelist_username(&self) -> Option<&Vec<String>> {
    self.approvals_whitelist_username.as_ref()
  }

  pub fn reset_approvals_whitelist_username(&mut self) {
    self.approvals_whitelist_username = None;
  }

  pub fn set_block_on_official_review_requests(&mut self, block_on_official_review_requests: bool) {
    self.block_on_official_review_requests = Some(block_on_official_review_requests);
  }

  pub fn with_block_on_official_review_requests(mut self, block_on_official_review_requests: bool) -> CreateBranchProtectionOption {
    self.block_on_official_review_requests = Some(block_on_official_review_requests);
    self
  }

  pub fn block_on_official_review_requests(&self) -> Option<&bool> {
    self.block_on_official_review_requests.as_ref()
  }

  pub fn reset_block_on_official_review_requests(&mut self) {
    self.block_on_official_review_requests = None;
  }

  pub fn set_block_on_outdated_branch(&mut self, block_on_outdated_branch: bool) {
    self.block_on_outdated_branch = Some(block_on_outdated_branch);
  }

  pub fn with_block_on_outdated_branch(mut self, block_on_outdated_branch: bool) -> CreateBranchProtectionOption {
    self.block_on_outdated_branch = Some(block_on_outdated_branch);
    self
  }

  pub fn block_on_outdated_branch(&self) -> Option<&bool> {
    self.block_on_outdated_branch.as_ref()
  }

  pub fn reset_block_on_outdated_branch(&mut self) {
    self.block_on_outdated_branch = None;
  }

  pub fn set_block_on_rejected_reviews(&mut self, block_on_rejected_reviews: bool) {
    self.block_on_rejected_reviews = Some(block_on_rejected_reviews);
  }

  pub fn with_block_on_rejected_reviews(mut self, block_on_rejected_reviews: bool) -> CreateBranchProtectionOption {
    self.block_on_rejected_reviews = Some(block_on_rejected_reviews);
    self
  }

  pub fn block_on_rejected_reviews(&self) -> Option<&bool> {
    self.block_on_rejected_reviews.as_ref()
  }

  pub fn reset_block_on_rejected_reviews(&mut self) {
    self.block_on_rejected_reviews = None;
  }

  pub fn set_branch_name(&mut self, branch_name: String) {
    self.branch_name = Some(branch_name);
  }

  pub fn with_branch_name(mut self, branch_name: String) -> CreateBranchProtectionOption {
    self.branch_name = Some(branch_name);
    self
  }

  pub fn branch_name(&self) -> Option<&String> {
    self.branch_name.as_ref()
  }

  pub fn reset_branch_name(&mut self) {
    self.branch_name = None;
  }

  pub fn set_dismiss_stale_approvals(&mut self, dismiss_stale_approvals: bool) {
    self.dismiss_stale_approvals = Some(dismiss_stale_approvals);
  }

  pub fn with_dismiss_stale_approvals(mut self, dismiss_stale_approvals: bool) -> CreateBranchProtectionOption {
    self.dismiss_stale_approvals = Some(dismiss_stale_approvals);
    self
  }

  pub fn dismiss_stale_approvals(&self) -> Option<&bool> {
    self.dismiss_stale_approvals.as_ref()
  }

  pub fn reset_dismiss_stale_approvals(&mut self) {
    self.dismiss_stale_approvals = None;
  }

  pub fn set_enable_approvals_whitelist(&mut self, enable_approvals_whitelist: bool) {
    self.enable_approvals_whitelist = Some(enable_approvals_whitelist);
  }

  pub fn with_enable_approvals_whitelist(mut self, enable_approvals_whitelist: bool) -> CreateBranchProtectionOption {
    self.enable_approvals_whitelist = Some(enable_approvals_whitelist);
    self
  }

  pub fn enable_approvals_whitelist(&self) -> Option<&bool> {
    self.enable_approvals_whitelist.as_ref()
  }

  pub fn reset_enable_approvals_whitelist(&mut self) {
    self.enable_approvals_whitelist = None;
  }

  pub fn set_enable_merge_whitelist(&mut self, enable_merge_whitelist: bool) {
    self.enable_merge_whitelist = Some(enable_merge_whitelist);
  }

  pub fn with_enable_merge_whitelist(mut self, enable_merge_whitelist: bool) -> CreateBranchProtectionOption {
    self.enable_merge_whitelist = Some(enable_merge_whitelist);
    self
  }

  pub fn enable_merge_whitelist(&self) -> Option<&bool> {
    self.enable_merge_whitelist.as_ref()
  }

  pub fn reset_enable_merge_whitelist(&mut self) {
    self.enable_merge_whitelist = None;
  }

  pub fn set_enable_push(&mut self, enable_push: bool) {
    self.enable_push = Some(enable_push);
  }

  pub fn with_enable_push(mut self, enable_push: bool) -> CreateBranchProtectionOption {
    self.enable_push = Some(enable_push);
    self
  }

  pub fn enable_push(&self) -> Option<&bool> {
    self.enable_push.as_ref()
  }

  pub fn reset_enable_push(&mut self) {
    self.enable_push = None;
  }

  pub fn set_enable_push_whitelist(&mut self, enable_push_whitelist: bool) {
    self.enable_push_whitelist = Some(enable_push_whitelist);
  }

  pub fn with_enable_push_whitelist(mut self, enable_push_whitelist: bool) -> CreateBranchProtectionOption {
    self.enable_push_whitelist = Some(enable_push_whitelist);
    self
  }

  pub fn enable_push_whitelist(&self) -> Option<&bool> {
    self.enable_push_whitelist.as_ref()
  }

  pub fn reset_enable_push_whitelist(&mut self) {
    self.enable_push_whitelist = None;
  }

  pub fn set_enable_status_check(&mut self, enable_status_check: bool) {
    self.enable_status_check = Some(enable_status_check);
  }

  pub fn with_enable_status_check(mut self, enable_status_check: bool) -> CreateBranchProtectionOption {
    self.enable_status_check = Some(enable_status_check);
    self
  }

  pub fn enable_status_check(&self) -> Option<&bool> {
    self.enable_status_check.as_ref()
  }

  pub fn reset_enable_status_check(&mut self) {
    self.enable_status_check = None;
  }

  pub fn set_merge_whitelist_teams(&mut self, merge_whitelist_teams: Vec<String>) {
    self.merge_whitelist_teams = Some(merge_whitelist_teams);
  }

  pub fn with_merge_whitelist_teams(mut self, merge_whitelist_teams: Vec<String>) -> CreateBranchProtectionOption {
    self.merge_whitelist_teams = Some(merge_whitelist_teams);
    self
  }

  pub fn merge_whitelist_teams(&self) -> Option<&Vec<String>> {
    self.merge_whitelist_teams.as_ref()
  }

  pub fn reset_merge_whitelist_teams(&mut self) {
    self.merge_whitelist_teams = None;
  }

  pub fn set_merge_whitelist_usernames(&mut self, merge_whitelist_usernames: Vec<String>) {
    self.merge_whitelist_usernames = Some(merge_whitelist_usernames);
  }

  pub fn with_merge_whitelist_usernames(mut self, merge_whitelist_usernames: Vec<String>) -> CreateBranchProtectionOption {
    self.merge_whitelist_usernames = Some(merge_whitelist_usernames);
    self
  }

  pub fn merge_whitelist_usernames(&self) -> Option<&Vec<String>> {
    self.merge_whitelist_usernames.as_ref()
  }

  pub fn reset_merge_whitelist_usernames(&mut self) {
    self.merge_whitelist_usernames = None;
  }

  pub fn set_protected_file_patterns(&mut self, protected_file_patterns: String) {
    self.protected_file_patterns = Some(protected_file_patterns);
  }

  pub fn with_protected_file_patterns(mut self, protected_file_patterns: String) -> CreateBranchProtectionOption {
    self.protected_file_patterns = Some(protected_file_patterns);
    self
  }

  pub fn protected_file_patterns(&self) -> Option<&String> {
    self.protected_file_patterns.as_ref()
  }

  pub fn reset_protected_file_patterns(&mut self) {
    self.protected_file_patterns = None;
  }

  pub fn set_push_whitelist_deploy_keys(&mut self, push_whitelist_deploy_keys: bool) {
    self.push_whitelist_deploy_keys = Some(push_whitelist_deploy_keys);
  }

  pub fn with_push_whitelist_deploy_keys(mut self, push_whitelist_deploy_keys: bool) -> CreateBranchProtectionOption {
    self.push_whitelist_deploy_keys = Some(push_whitelist_deploy_keys);
    self
  }

  pub fn push_whitelist_deploy_keys(&self) -> Option<&bool> {
    self.push_whitelist_deploy_keys.as_ref()
  }

  pub fn reset_push_whitelist_deploy_keys(&mut self) {
    self.push_whitelist_deploy_keys = None;
  }

  pub fn set_push_whitelist_teams(&mut self, push_whitelist_teams: Vec<String>) {
    self.push_whitelist_teams = Some(push_whitelist_teams);
  }

  pub fn with_push_whitelist_teams(mut self, push_whitelist_teams: Vec<String>) -> CreateBranchProtectionOption {
    self.push_whitelist_teams = Some(push_whitelist_teams);
    self
  }

  pub fn push_whitelist_teams(&self) -> Option<&Vec<String>> {
    self.push_whitelist_teams.as_ref()
  }

  pub fn reset_push_whitelist_teams(&mut self) {
    self.push_whitelist_teams = None;
  }

  pub fn set_push_whitelist_usernames(&mut self, push_whitelist_usernames: Vec<String>) {
    self.push_whitelist_usernames = Some(push_whitelist_usernames);
  }

  pub fn with_push_whitelist_usernames(mut self, push_whitelist_usernames: Vec<String>) -> CreateBranchProtectionOption {
    self.push_whitelist_usernames = Some(push_whitelist_usernames);
    self
  }

  pub fn push_whitelist_usernames(&self) -> Option<&Vec<String>> {
    self.push_whitelist_usernames.as_ref()
  }

  pub fn reset_push_whitelist_usernames(&mut self) {
    self.push_whitelist_usernames = None;
  }

  pub fn set_require_signed_commits(&mut self, require_signed_commits: bool) {
    self.require_signed_commits = Some(require_signed_commits);
  }

  pub fn with_require_signed_commits(mut self, require_signed_commits: bool) -> CreateBranchProtectionOption {
    self.require_signed_commits = Some(require_signed_commits);
    self
  }

  pub fn require_signed_commits(&self) -> Option<&bool> {
    self.require_signed_commits.as_ref()
  }

  pub fn reset_require_signed_commits(&mut self) {
    self.require_signed_commits = None;
  }

  pub fn set_required_approvals(&mut self, required_approvals: i64) {
    self.required_approvals = Some(required_approvals);
  }

  pub fn with_required_approvals(mut self, required_approvals: i64) -> CreateBranchProtectionOption {
    self.required_approvals = Some(required_approvals);
    self
  }

  pub fn required_approvals(&self) -> Option<&i64> {
    self.required_approvals.as_ref()
  }

  pub fn reset_required_approvals(&mut self) {
    self.required_approvals = None;
  }

  pub fn set_rule_name(&mut self, rule_name: String) {
    self.rule_name = Some(rule_name);
  }

  pub fn with_rule_name(mut self, rule_name: String) -> CreateBranchProtectionOption {
    self.rule_name = Some(rule_name);
    self
  }

  pub fn rule_name(&self) -> Option<&String> {
    self.rule_name.as_ref()
  }

  pub fn reset_rule_name(&mut self) {
    self.rule_name = None;
  }

  pub fn set_status_check_contexts(&mut self, status_check_contexts: Vec<String>) {
    self.status_check_contexts = Some(status_check_contexts);
  }

  pub fn with_status_check_contexts(mut self, status_check_contexts: Vec<String>) -> CreateBranchProtectionOption {
    self.status_check_contexts = Some(status_check_contexts);
    self
  }

  pub fn status_check_contexts(&self) -> Option<&Vec<String>> {
    self.status_check_contexts.as_ref()
  }

  pub fn reset_status_check_contexts(&mut self) {
    self.status_check_contexts = None;
  }

  pub fn set_unprotected_file_patterns(&mut self, unprotected_file_patterns: String) {
    self.unprotected_file_patterns = Some(unprotected_file_patterns);
  }

  pub fn with_unprotected_file_patterns(mut self, unprotected_file_patterns: String) -> CreateBranchProtectionOption {
    self.unprotected_file_patterns = Some(unprotected_file_patterns);
    self
  }

  pub fn unprotected_file_patterns(&self) -> Option<&String> {
    self.unprotected_file_patterns.as_ref()
  }

  pub fn reset_unprotected_file_patterns(&mut self) {
    self.unprotected_file_patterns = None;
  }

}



