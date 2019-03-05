/*
 * IoT Edge Management API
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: 2018-06-28
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use serde_derive::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModuleDetails {
    /// System generated unique identitier.
    #[serde(rename = "id")]
    id: String,
    /// The name of the module.
    #[serde(rename = "name")]
    name: String,
    /// The type of a module.
    #[serde(rename = "type")]
    type_: String,
    #[serde(rename = "config")]
    config: crate::models::Config,
    #[serde(rename = "status")]
    status: crate::models::Status,
}

impl ModuleDetails {
    pub fn new(
        id: String,
        name: String,
        type_: String,
        config: crate::models::Config,
        status: crate::models::Status,
    ) -> Self {
        ModuleDetails {
            id,
            name,
            type_,
            config,
            status,
        }
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn with_id(mut self, id: String) -> Self {
        self.id = id;
        self
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_type(&mut self, type_: String) {
        self.type_ = type_;
    }

    pub fn with_type(mut self, type_: String) -> Self {
        self.type_ = type_;
        self
    }

    pub fn type_(&self) -> &String {
        &self.type_
    }

    pub fn set_config(&mut self, config: crate::models::Config) {
        self.config = config;
    }

    pub fn with_config(mut self, config: crate::models::Config) -> Self {
        self.config = config;
        self
    }

    pub fn config(&self) -> &crate::models::Config {
        &self.config
    }

    pub fn set_status(&mut self, status: crate::models::Status) {
        self.status = status;
    }

    pub fn with_status(mut self, status: crate::models::Status) -> Self {
        self.status = status;
        self
    }

    pub fn status(&self) -> &crate::models::Status {
        &self.status
    }
}
