//! Core type definitions matching the Crucible specification

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub version: String,
    pub project: ProjectConfig,
    pub modules: Vec<String>,
    #[serde(default = "default_strict")]
    pub strict_validation: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub language: Language,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture_pattern: Option<ArchitecturePattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    TypeScript,
    Rust,
    Python,
    Go,
    Java,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ArchitecturePattern {
    Layered,
    Hexagonal,
    Microservices,
    Modular,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub module: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub exports: HashMap<String, Export>,
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Export {
    #[serde(rename = "type")]
    pub export_type: ExportType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub methods: Option<HashMap<String, Method>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, Property>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<Dependency>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExportType {
    Class,
    Function,
    Interface,
    Type,
    Enum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Method {
    pub inputs: Vec<Parameter>,
    pub returns: ReturnType,
    #[serde(default)]
    pub throws: Vec<String>,
    #[serde(default)]
    pub calls: Vec<String>,
    #[serde(default)]
    pub effects: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "type")]
    pub param_type: String,
    #[serde(default)]
    pub optional: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnType {
    #[serde(rename = "type")]
    pub return_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inner: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    #[serde(rename = "type")]
    pub prop_type: String,
    #[serde(default = "default_required")]
    pub required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub module: String,
    pub imports: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
}

/// A complete Crucible project
#[derive(Debug)]
pub struct Project {
    pub manifest: Manifest,
    pub modules: Vec<Module>,
    pub rules: Option<Rules>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rules {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<Architecture>,
    pub rules: Vec<Rule>,
    #[serde(default)]
    pub custom_rules: Vec<CustomRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Architecture {
    pub pattern: ArchitecturePattern,
    pub layers: Vec<Layer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    pub name: String,
    pub can_depend_on: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub id: String,
    pub enabled: bool,
    pub severity: Severity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomRule {
    pub id: String,
    #[serde(rename = "type")]
    pub rule_type: String,
    pub target: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    pub severity: Severity,
}

fn default_strict() -> bool {
    true
}

fn default_required() -> bool {
    true
}
