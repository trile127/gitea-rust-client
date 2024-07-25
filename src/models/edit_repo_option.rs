/* 
 * Gitea API
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.21.11
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// EditRepoOption : EditRepoOption options when editing a repository's properties

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EditRepoOption {
  /// either `true` to allow mark pr as merged manually, or `false` to prevent it.
  #[serde(rename = "allow_manual_merge")]
  allow_manual_merge: Option<bool>,
  /// either `true` to allow merging pull requests with a merge commit, or `false` to prevent merging pull requests with merge commits.
  #[serde(rename = "allow_merge_commits")]
  allow_merge_commits: Option<bool>,
  /// either `true` to allow rebase-merging pull requests, or `false` to prevent rebase-merging.
  #[serde(rename = "allow_rebase")]
  allow_rebase: Option<bool>,
  /// either `true` to allow rebase with explicit merge commits (--no-ff), or `false` to prevent rebase with explicit merge commits.
  #[serde(rename = "allow_rebase_explicit")]
  allow_rebase_explicit: Option<bool>,
  /// either `true` to allow updating pull request branch by rebase, or `false` to prevent it.
  #[serde(rename = "allow_rebase_update")]
  allow_rebase_update: Option<bool>,
  /// either `true` to allow squash-merging pull requests, or `false` to prevent squash-merging.
  #[serde(rename = "allow_squash_merge")]
  allow_squash_merge: Option<bool>,
  /// set to `true` to archive this repository.
  #[serde(rename = "archived")]
  archived: Option<bool>,
  /// either `true` to enable AutodetectManualMerge, or `false` to prevent it. Note: In some special cases, misjudgments can occur.
  #[serde(rename = "autodetect_manual_merge")]
  autodetect_manual_merge: Option<bool>,
  /// set to `true` to allow edits from maintainers by default
  #[serde(rename = "default_allow_maintainer_edit")]
  default_allow_maintainer_edit: Option<bool>,
  /// sets the default branch for this repository.
  #[serde(rename = "default_branch")]
  default_branch: Option<String>,
  /// set to `true` to delete pr branch after merge by default
  #[serde(rename = "default_delete_branch_after_merge")]
  default_delete_branch_after_merge: Option<bool>,
  /// set to a merge style to be used by this repository: \"merge\", \"rebase\", \"rebase-merge\", or \"squash\".
  #[serde(rename = "default_merge_style")]
  default_merge_style: Option<String>,
  /// a short description of the repository.
  #[serde(rename = "description")]
  description: Option<String>,
  /// enable prune - remove obsolete remote-tracking references
  #[serde(rename = "enable_prune")]
  enable_prune: Option<bool>,
  #[serde(rename = "external_tracker")]
  external_tracker: Option<::models::ExternalTracker>,
  #[serde(rename = "external_wiki")]
  external_wiki: Option<::models::ExternalWiki>,
  /// either `true` to enable actions unit, or `false` to disable them.
  #[serde(rename = "has_actions")]
  has_actions: Option<bool>,
  /// either `true` to enable issues for this repository or `false` to disable them.
  #[serde(rename = "has_issues")]
  has_issues: Option<bool>,
  /// either `true` to enable packages unit, or `false` to disable them.
  #[serde(rename = "has_packages")]
  has_packages: Option<bool>,
  /// either `true` to enable project unit, or `false` to disable them.
  #[serde(rename = "has_projects")]
  has_projects: Option<bool>,
  /// either `true` to allow pull requests, or `false` to prevent pull request.
  #[serde(rename = "has_pull_requests")]
  has_pull_requests: Option<bool>,
  /// either `true` to enable releases unit, or `false` to disable them.
  #[serde(rename = "has_releases")]
  has_releases: Option<bool>,
  /// either `true` to enable the wiki for this repository or `false` to disable it.
  #[serde(rename = "has_wiki")]
  has_wiki: Option<bool>,
  /// either `true` to ignore whitespace for conflicts, or `false` to not ignore whitespace.
  #[serde(rename = "ignore_whitespace_conflicts")]
  ignore_whitespace_conflicts: Option<bool>,
  #[serde(rename = "internal_tracker")]
  internal_tracker: Option<::models::InternalTracker>,
  /// set to a string like `8h30m0s` to set the mirror interval time
  #[serde(rename = "mirror_interval")]
  mirror_interval: Option<String>,
  /// name of the repository
  #[serde(rename = "name")]
  name: Option<String>,
  /// either `true` to make the repository private or `false` to make it public. Note: you will get a 422 error if the organization restricts changing repository visibility to organization owners and a non-owner tries to change the value of private.
  #[serde(rename = "private")]
  private: Option<bool>,
  /// either `true` to make this repository a template or `false` to make it a normal repository
  #[serde(rename = "template")]
  template: Option<bool>,
  /// a URL with more information about the repository.
  #[serde(rename = "website")]
  website: Option<String>
}

impl EditRepoOption {
  /// EditRepoOption options when editing a repository's properties
  pub fn new() -> EditRepoOption {
    EditRepoOption {
      allow_manual_merge: None,
      allow_merge_commits: None,
      allow_rebase: None,
      allow_rebase_explicit: None,
      allow_rebase_update: None,
      allow_squash_merge: None,
      archived: None,
      autodetect_manual_merge: None,
      default_allow_maintainer_edit: None,
      default_branch: None,
      default_delete_branch_after_merge: None,
      default_merge_style: None,
      description: None,
      enable_prune: None,
      external_tracker: None,
      external_wiki: None,
      has_actions: None,
      has_issues: None,
      has_packages: None,
      has_projects: None,
      has_pull_requests: None,
      has_releases: None,
      has_wiki: None,
      ignore_whitespace_conflicts: None,
      internal_tracker: None,
      mirror_interval: None,
      name: None,
      private: None,
      template: None,
      website: None
    }
  }

  pub fn set_allow_manual_merge(&mut self, allow_manual_merge: bool) {
    self.allow_manual_merge = Some(allow_manual_merge);
  }

  pub fn with_allow_manual_merge(mut self, allow_manual_merge: bool) -> EditRepoOption {
    self.allow_manual_merge = Some(allow_manual_merge);
    self
  }

  pub fn allow_manual_merge(&self) -> Option<&bool> {
    self.allow_manual_merge.as_ref()
  }

  pub fn reset_allow_manual_merge(&mut self) {
    self.allow_manual_merge = None;
  }

  pub fn set_allow_merge_commits(&mut self, allow_merge_commits: bool) {
    self.allow_merge_commits = Some(allow_merge_commits);
  }

  pub fn with_allow_merge_commits(mut self, allow_merge_commits: bool) -> EditRepoOption {
    self.allow_merge_commits = Some(allow_merge_commits);
    self
  }

  pub fn allow_merge_commits(&self) -> Option<&bool> {
    self.allow_merge_commits.as_ref()
  }

  pub fn reset_allow_merge_commits(&mut self) {
    self.allow_merge_commits = None;
  }

  pub fn set_allow_rebase(&mut self, allow_rebase: bool) {
    self.allow_rebase = Some(allow_rebase);
  }

  pub fn with_allow_rebase(mut self, allow_rebase: bool) -> EditRepoOption {
    self.allow_rebase = Some(allow_rebase);
    self
  }

  pub fn allow_rebase(&self) -> Option<&bool> {
    self.allow_rebase.as_ref()
  }

  pub fn reset_allow_rebase(&mut self) {
    self.allow_rebase = None;
  }

  pub fn set_allow_rebase_explicit(&mut self, allow_rebase_explicit: bool) {
    self.allow_rebase_explicit = Some(allow_rebase_explicit);
  }

  pub fn with_allow_rebase_explicit(mut self, allow_rebase_explicit: bool) -> EditRepoOption {
    self.allow_rebase_explicit = Some(allow_rebase_explicit);
    self
  }

  pub fn allow_rebase_explicit(&self) -> Option<&bool> {
    self.allow_rebase_explicit.as_ref()
  }

  pub fn reset_allow_rebase_explicit(&mut self) {
    self.allow_rebase_explicit = None;
  }

  pub fn set_allow_rebase_update(&mut self, allow_rebase_update: bool) {
    self.allow_rebase_update = Some(allow_rebase_update);
  }

  pub fn with_allow_rebase_update(mut self, allow_rebase_update: bool) -> EditRepoOption {
    self.allow_rebase_update = Some(allow_rebase_update);
    self
  }

  pub fn allow_rebase_update(&self) -> Option<&bool> {
    self.allow_rebase_update.as_ref()
  }

  pub fn reset_allow_rebase_update(&mut self) {
    self.allow_rebase_update = None;
  }

  pub fn set_allow_squash_merge(&mut self, allow_squash_merge: bool) {
    self.allow_squash_merge = Some(allow_squash_merge);
  }

  pub fn with_allow_squash_merge(mut self, allow_squash_merge: bool) -> EditRepoOption {
    self.allow_squash_merge = Some(allow_squash_merge);
    self
  }

  pub fn allow_squash_merge(&self) -> Option<&bool> {
    self.allow_squash_merge.as_ref()
  }

  pub fn reset_allow_squash_merge(&mut self) {
    self.allow_squash_merge = None;
  }

  pub fn set_archived(&mut self, archived: bool) {
    self.archived = Some(archived);
  }

  pub fn with_archived(mut self, archived: bool) -> EditRepoOption {
    self.archived = Some(archived);
    self
  }

  pub fn archived(&self) -> Option<&bool> {
    self.archived.as_ref()
  }

  pub fn reset_archived(&mut self) {
    self.archived = None;
  }

  pub fn set_autodetect_manual_merge(&mut self, autodetect_manual_merge: bool) {
    self.autodetect_manual_merge = Some(autodetect_manual_merge);
  }

  pub fn with_autodetect_manual_merge(mut self, autodetect_manual_merge: bool) -> EditRepoOption {
    self.autodetect_manual_merge = Some(autodetect_manual_merge);
    self
  }

  pub fn autodetect_manual_merge(&self) -> Option<&bool> {
    self.autodetect_manual_merge.as_ref()
  }

  pub fn reset_autodetect_manual_merge(&mut self) {
    self.autodetect_manual_merge = None;
  }

  pub fn set_default_allow_maintainer_edit(&mut self, default_allow_maintainer_edit: bool) {
    self.default_allow_maintainer_edit = Some(default_allow_maintainer_edit);
  }

  pub fn with_default_allow_maintainer_edit(mut self, default_allow_maintainer_edit: bool) -> EditRepoOption {
    self.default_allow_maintainer_edit = Some(default_allow_maintainer_edit);
    self
  }

  pub fn default_allow_maintainer_edit(&self) -> Option<&bool> {
    self.default_allow_maintainer_edit.as_ref()
  }

  pub fn reset_default_allow_maintainer_edit(&mut self) {
    self.default_allow_maintainer_edit = None;
  }

  pub fn set_default_branch(&mut self, default_branch: String) {
    self.default_branch = Some(default_branch);
  }

  pub fn with_default_branch(mut self, default_branch: String) -> EditRepoOption {
    self.default_branch = Some(default_branch);
    self
  }

  pub fn default_branch(&self) -> Option<&String> {
    self.default_branch.as_ref()
  }

  pub fn reset_default_branch(&mut self) {
    self.default_branch = None;
  }

  pub fn set_default_delete_branch_after_merge(&mut self, default_delete_branch_after_merge: bool) {
    self.default_delete_branch_after_merge = Some(default_delete_branch_after_merge);
  }

  pub fn with_default_delete_branch_after_merge(mut self, default_delete_branch_after_merge: bool) -> EditRepoOption {
    self.default_delete_branch_after_merge = Some(default_delete_branch_after_merge);
    self
  }

  pub fn default_delete_branch_after_merge(&self) -> Option<&bool> {
    self.default_delete_branch_after_merge.as_ref()
  }

  pub fn reset_default_delete_branch_after_merge(&mut self) {
    self.default_delete_branch_after_merge = None;
  }

  pub fn set_default_merge_style(&mut self, default_merge_style: String) {
    self.default_merge_style = Some(default_merge_style);
  }

  pub fn with_default_merge_style(mut self, default_merge_style: String) -> EditRepoOption {
    self.default_merge_style = Some(default_merge_style);
    self
  }

  pub fn default_merge_style(&self) -> Option<&String> {
    self.default_merge_style.as_ref()
  }

  pub fn reset_default_merge_style(&mut self) {
    self.default_merge_style = None;
  }

  pub fn set_description(&mut self, description: String) {
    self.description = Some(description);
  }

  pub fn with_description(mut self, description: String) -> EditRepoOption {
    self.description = Some(description);
    self
  }

  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }

  pub fn reset_description(&mut self) {
    self.description = None;
  }

  pub fn set_enable_prune(&mut self, enable_prune: bool) {
    self.enable_prune = Some(enable_prune);
  }

  pub fn with_enable_prune(mut self, enable_prune: bool) -> EditRepoOption {
    self.enable_prune = Some(enable_prune);
    self
  }

  pub fn enable_prune(&self) -> Option<&bool> {
    self.enable_prune.as_ref()
  }

  pub fn reset_enable_prune(&mut self) {
    self.enable_prune = None;
  }

  pub fn set_external_tracker(&mut self, external_tracker: ::models::ExternalTracker) {
    self.external_tracker = Some(external_tracker);
  }

  pub fn with_external_tracker(mut self, external_tracker: ::models::ExternalTracker) -> EditRepoOption {
    self.external_tracker = Some(external_tracker);
    self
  }

  pub fn external_tracker(&self) -> Option<&::models::ExternalTracker> {
    self.external_tracker.as_ref()
  }

  pub fn reset_external_tracker(&mut self) {
    self.external_tracker = None;
  }

  pub fn set_external_wiki(&mut self, external_wiki: ::models::ExternalWiki) {
    self.external_wiki = Some(external_wiki);
  }

  pub fn with_external_wiki(mut self, external_wiki: ::models::ExternalWiki) -> EditRepoOption {
    self.external_wiki = Some(external_wiki);
    self
  }

  pub fn external_wiki(&self) -> Option<&::models::ExternalWiki> {
    self.external_wiki.as_ref()
  }

  pub fn reset_external_wiki(&mut self) {
    self.external_wiki = None;
  }

  pub fn set_has_actions(&mut self, has_actions: bool) {
    self.has_actions = Some(has_actions);
  }

  pub fn with_has_actions(mut self, has_actions: bool) -> EditRepoOption {
    self.has_actions = Some(has_actions);
    self
  }

  pub fn has_actions(&self) -> Option<&bool> {
    self.has_actions.as_ref()
  }

  pub fn reset_has_actions(&mut self) {
    self.has_actions = None;
  }

  pub fn set_has_issues(&mut self, has_issues: bool) {
    self.has_issues = Some(has_issues);
  }

  pub fn with_has_issues(mut self, has_issues: bool) -> EditRepoOption {
    self.has_issues = Some(has_issues);
    self
  }

  pub fn has_issues(&self) -> Option<&bool> {
    self.has_issues.as_ref()
  }

  pub fn reset_has_issues(&mut self) {
    self.has_issues = None;
  }

  pub fn set_has_packages(&mut self, has_packages: bool) {
    self.has_packages = Some(has_packages);
  }

  pub fn with_has_packages(mut self, has_packages: bool) -> EditRepoOption {
    self.has_packages = Some(has_packages);
    self
  }

  pub fn has_packages(&self) -> Option<&bool> {
    self.has_packages.as_ref()
  }

  pub fn reset_has_packages(&mut self) {
    self.has_packages = None;
  }

  pub fn set_has_projects(&mut self, has_projects: bool) {
    self.has_projects = Some(has_projects);
  }

  pub fn with_has_projects(mut self, has_projects: bool) -> EditRepoOption {
    self.has_projects = Some(has_projects);
    self
  }

  pub fn has_projects(&self) -> Option<&bool> {
    self.has_projects.as_ref()
  }

  pub fn reset_has_projects(&mut self) {
    self.has_projects = None;
  }

  pub fn set_has_pull_requests(&mut self, has_pull_requests: bool) {
    self.has_pull_requests = Some(has_pull_requests);
  }

  pub fn with_has_pull_requests(mut self, has_pull_requests: bool) -> EditRepoOption {
    self.has_pull_requests = Some(has_pull_requests);
    self
  }

  pub fn has_pull_requests(&self) -> Option<&bool> {
    self.has_pull_requests.as_ref()
  }

  pub fn reset_has_pull_requests(&mut self) {
    self.has_pull_requests = None;
  }

  pub fn set_has_releases(&mut self, has_releases: bool) {
    self.has_releases = Some(has_releases);
  }

  pub fn with_has_releases(mut self, has_releases: bool) -> EditRepoOption {
    self.has_releases = Some(has_releases);
    self
  }

  pub fn has_releases(&self) -> Option<&bool> {
    self.has_releases.as_ref()
  }

  pub fn reset_has_releases(&mut self) {
    self.has_releases = None;
  }

  pub fn set_has_wiki(&mut self, has_wiki: bool) {
    self.has_wiki = Some(has_wiki);
  }

  pub fn with_has_wiki(mut self, has_wiki: bool) -> EditRepoOption {
    self.has_wiki = Some(has_wiki);
    self
  }

  pub fn has_wiki(&self) -> Option<&bool> {
    self.has_wiki.as_ref()
  }

  pub fn reset_has_wiki(&mut self) {
    self.has_wiki = None;
  }

  pub fn set_ignore_whitespace_conflicts(&mut self, ignore_whitespace_conflicts: bool) {
    self.ignore_whitespace_conflicts = Some(ignore_whitespace_conflicts);
  }

  pub fn with_ignore_whitespace_conflicts(mut self, ignore_whitespace_conflicts: bool) -> EditRepoOption {
    self.ignore_whitespace_conflicts = Some(ignore_whitespace_conflicts);
    self
  }

  pub fn ignore_whitespace_conflicts(&self) -> Option<&bool> {
    self.ignore_whitespace_conflicts.as_ref()
  }

  pub fn reset_ignore_whitespace_conflicts(&mut self) {
    self.ignore_whitespace_conflicts = None;
  }

  pub fn set_internal_tracker(&mut self, internal_tracker: ::models::InternalTracker) {
    self.internal_tracker = Some(internal_tracker);
  }

  pub fn with_internal_tracker(mut self, internal_tracker: ::models::InternalTracker) -> EditRepoOption {
    self.internal_tracker = Some(internal_tracker);
    self
  }

  pub fn internal_tracker(&self) -> Option<&::models::InternalTracker> {
    self.internal_tracker.as_ref()
  }

  pub fn reset_internal_tracker(&mut self) {
    self.internal_tracker = None;
  }

  pub fn set_mirror_interval(&mut self, mirror_interval: String) {
    self.mirror_interval = Some(mirror_interval);
  }

  pub fn with_mirror_interval(mut self, mirror_interval: String) -> EditRepoOption {
    self.mirror_interval = Some(mirror_interval);
    self
  }

  pub fn mirror_interval(&self) -> Option<&String> {
    self.mirror_interval.as_ref()
  }

  pub fn reset_mirror_interval(&mut self) {
    self.mirror_interval = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> EditRepoOption {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_private(&mut self, private: bool) {
    self.private = Some(private);
  }

  pub fn with_private(mut self, private: bool) -> EditRepoOption {
    self.private = Some(private);
    self
  }

  pub fn private(&self) -> Option<&bool> {
    self.private.as_ref()
  }

  pub fn reset_private(&mut self) {
    self.private = None;
  }

  pub fn set_template(&mut self, template: bool) {
    self.template = Some(template);
  }

  pub fn with_template(mut self, template: bool) -> EditRepoOption {
    self.template = Some(template);
    self
  }

  pub fn template(&self) -> Option<&bool> {
    self.template.as_ref()
  }

  pub fn reset_template(&mut self) {
    self.template = None;
  }

  pub fn set_website(&mut self, website: String) {
    self.website = Some(website);
  }

  pub fn with_website(mut self, website: String) -> EditRepoOption {
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



