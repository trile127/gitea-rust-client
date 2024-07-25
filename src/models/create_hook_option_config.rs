/* 
 * Gitea API
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.21.11
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// CreateHookOptionConfig : CreateHookOptionConfig has all config options in it required are \"content_type\" and \"url\" Required

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateHookOptionConfig {
}

impl CreateHookOptionConfig {
  /// CreateHookOptionConfig has all config options in it required are \"content_type\" and \"url\" Required
  pub fn new() -> CreateHookOptionConfig {
    CreateHookOptionConfig {
    }
  }

}


