/* 
 * Gitea API
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.21.11
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ExternalWiki : ExternalWiki represents setting for external wiki

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalWiki {
  /// URL of external wiki.
  #[serde(rename = "external_wiki_url")]
  external_wiki_url: Option<String>
}

impl ExternalWiki {
  /// ExternalWiki represents setting for external wiki
  pub fn new() -> ExternalWiki {
    ExternalWiki {
      external_wiki_url: None
    }
  }

  pub fn set_external_wiki_url(&mut self, external_wiki_url: String) {
    self.external_wiki_url = Some(external_wiki_url);
  }

  pub fn with_external_wiki_url(mut self, external_wiki_url: String) -> ExternalWiki {
    self.external_wiki_url = Some(external_wiki_url);
    self
  }

  pub fn external_wiki_url(&self) -> Option<&String> {
    self.external_wiki_url.as_ref()
  }

  pub fn reset_external_wiki_url(&mut self) {
    self.external_wiki_url = None;
  }

}



