/* 
 * Gitea API
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.21.11
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// IssueFormField : IssueFormField represents a form field

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IssueFormField {
  #[serde(rename = "attributes")]
  attributes: Option<::std::collections::HashMap<String, Value>>,
  #[serde(rename = "id")]
  id: Option<String>,
  #[serde(rename = "type")]
  _type: Option<::models::IssueFormFieldType>,
  #[serde(rename = "validations")]
  validations: Option<::std::collections::HashMap<String, Value>>
}

impl IssueFormField {
  /// IssueFormField represents a form field
  pub fn new() -> IssueFormField {
    IssueFormField {
      attributes: None,
      id: None,
      _type: None,
      validations: None
    }
  }

  pub fn set_attributes(&mut self, attributes: ::std::collections::HashMap<String, Value>) {
    self.attributes = Some(attributes);
  }

  pub fn with_attributes(mut self, attributes: ::std::collections::HashMap<String, Value>) -> IssueFormField {
    self.attributes = Some(attributes);
    self
  }

  pub fn attributes(&self) -> Option<&::std::collections::HashMap<String, Value>> {
    self.attributes.as_ref()
  }

  pub fn reset_attributes(&mut self) {
    self.attributes = None;
  }

  pub fn set_id(&mut self, id: String) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: String) -> IssueFormField {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&String> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set__type(&mut self, _type: ::models::IssueFormFieldType) {
    self._type = Some(_type);
  }

  pub fn with__type(mut self, _type: ::models::IssueFormFieldType) -> IssueFormField {
    self._type = Some(_type);
    self
  }

  pub fn _type(&self) -> Option<&::models::IssueFormFieldType> {
    self._type.as_ref()
  }

  pub fn reset__type(&mut self) {
    self._type = None;
  }

  pub fn set_validations(&mut self, validations: ::std::collections::HashMap<String, Value>) {
    self.validations = Some(validations);
  }

  pub fn with_validations(mut self, validations: ::std::collections::HashMap<String, Value>) -> IssueFormField {
    self.validations = Some(validations);
    self
  }

  pub fn validations(&self) -> Option<&::std::collections::HashMap<String, Value>> {
    self.validations.as_ref()
  }

  pub fn reset_validations(&mut self) {
    self.validations = None;
  }

}


