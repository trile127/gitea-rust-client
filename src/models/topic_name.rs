/* 
 * Gitea API
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.21.11
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// TopicName : TopicName a list of repo topic names

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct TopicName {
  #[serde(rename = "topics")]
  topics: Option<Vec<String>>
}

impl TopicName {
  /// TopicName a list of repo topic names
  pub fn new() -> TopicName {
    TopicName {
      topics: None
    }
  }

  pub fn set_topics(&mut self, topics: Vec<String>) {
    self.topics = Some(topics);
  }

  pub fn with_topics(mut self, topics: Vec<String>) -> TopicName {
    self.topics = Some(topics);
    self
  }

  pub fn topics(&self) -> Option<&Vec<String>> {
    self.topics.as_ref()
  }

  pub fn reset_topics(&mut self) {
    self.topics = None;
  }

}



