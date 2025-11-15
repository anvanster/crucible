//! Template engine for code generation

use crate::error::{CrucibleError, Result};
use handlebars::Handlebars;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Template engine for generating files from Handlebars templates
pub struct TemplateEngine {
    handlebars: Handlebars<'static>,
    templates: HashMap<String, String>,
}

impl TemplateEngine {
    /// Create a new template engine
    pub fn new() -> Self {
        Self {
            handlebars: Handlebars::new(),
            templates: HashMap::new(),
        }
    }

    /// Register a template from a string
    pub fn register_template(&mut self, name: &str, template: &str) -> Result<()> {
        self.handlebars
            .register_template_string(name, template)
            .map_err(|e| CrucibleError::ParseError {
                file: format!("template:{}", name),
                message: e.to_string(),
            })?;

        self.templates
            .insert(name.to_string(), template.to_string());
        Ok(())
    }

    /// Register a template from a file
    pub fn register_template_file(&mut self, name: &str, path: &Path) -> Result<()> {
        let template = fs::read_to_string(path).map_err(|e| CrucibleError::FileRead {
            path: path.display().to_string(),
            source: e,
        })?;

        self.register_template(name, &template)
    }

    /// Render a template with the given data
    pub fn render(&self, template_name: &str, data: &Value) -> Result<String> {
        self.handlebars
            .render(template_name, data)
            .map_err(|e| CrucibleError::ParseError {
                file: format!("template:{}", template_name),
                message: e.to_string(),
            })
    }

    /// Register default templates
    pub fn register_defaults(&mut self) -> Result<()> {
        // Instructions template
        let instructions_template = include_str!("../templates/instructions.md.hbs");
        self.register_template("instructions", instructions_template)?;

        // Hooks template
        let hooks_template = include_str!("../templates/hooks.md.hbs");
        self.register_template("hooks", hooks_template)?;

        // Config template (though we use serde_json directly for this)
        let config_template = include_str!("../templates/config.json.hbs");
        self.register_template("config", config_template)?;

        Ok(())
    }
}

impl Default for TemplateEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_template_engine() {
        let mut engine = TemplateEngine::new();
        engine.register_template("test", "Hello {{name}}!").unwrap();

        let data = json!({ "name": "World" });
        let result = engine.render("test", &data).unwrap();

        assert_eq!(result, "Hello World!");
    }
}
