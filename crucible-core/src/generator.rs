//! Code generator for Crucible architectures

use crate::error::{CrucibleError, Result};
use crate::types::{Module, Project, ExportType};
use std::fs;
use std::path::Path;

pub struct Generator {
    project: Project,
}

impl Generator {
    pub fn new(project: Project) -> Self {
        Self { project }
    }

    pub fn generate_typescript(&self, output_dir: &Path) -> Result<()> {
        fs::create_dir_all(output_dir).map_err(|e| {
            CrucibleError::FileRead {
                path: output_dir.display().to_string(),
                source: e,
            }
        })?;

        for module in &self.project.modules {
            let content = self.generate_typescript_module(module)?;
            let file_path = output_dir.join(format!("{}.ts", module.module));

            fs::write(&file_path, content).map_err(|e| {
                CrucibleError::FileRead {
                    path: file_path.display().to_string(),
                    source: e,
                }
            })?;
        }

        Ok(())
    }

    fn generate_typescript_module(&self, module: &Module) -> Result<String> {
        let mut output = String::new();

        // Header
        output.push_str(&format!(
            "// Generated from Crucible module: {}\n",
            module.module
        ));
        output.push_str(&format!("// Version: {}\n\n", module.version));

        // Generate exports
        for (name, export) in &module.exports {
            match export.export_type {
                ExportType::Interface => {
                    output.push_str(&format!("export interface {} {{\n", name));
                    if let Some(props) = &export.properties {
                        for (prop_name, prop) in props {
                            let optional = if prop.required { "" } else { "?" };
                            output.push_str(&format!(
                                "  {}{}: {};\n",
                                prop_name, optional, prop.prop_type
                            ));
                        }
                    }
                    output.push_str("}\n\n");
                }
                ExportType::Class => {
                    output.push_str(&format!("export class {} {{\n", name));
                    if let Some(methods) = &export.methods {
                        for (method_name, method) in methods {
                            output.push_str(&format!("  {}(", method_name));

                            // Parameters
                            let params: Vec<String> = method.inputs.iter()
                                .map(|p| format!("{}: {}", p.name, p.param_type))
                                .collect();
                            output.push_str(&params.join(", "));

                            // Return type
                            output.push_str(&format!("): {} {{\n", method.returns.return_type));
                            output.push_str("    throw new Error('Not implemented');\n");
                            output.push_str("  }\n\n");
                        }
                    }
                    output.push_str("}\n\n");
                }
                ExportType::Function => {
                    if let Some(methods) = &export.methods {
                        // For function exports, we expect a single "function" entry
                        if let Some((_, method)) = methods.iter().next() {
                            output.push_str(&format!("export function {}(", name));

                            // Parameters
                            let params: Vec<String> = method.inputs.iter()
                                .map(|p| format!("{}: {}", p.name, p.param_type))
                                .collect();
                            output.push_str(&params.join(", "));

                            // Return type
                            output.push_str(&format!("): {} {{\n", method.returns.return_type));
                            output.push_str("  throw new Error('Not implemented');\n");
                            output.push_str("}\n\n");
                        }
                    }
                }
                ExportType::Enum => {
                    output.push_str(&format!("export enum {} {{\n", name));
                    if let Some(values) = &export.values {
                        for value in values {
                            output.push_str(&format!("  {} = '{}',\n", value, value));
                        }
                    }
                    output.push_str("}\n\n");
                }
                _ => {
                    // TODO: Implement other export types
                }
            }
        }

        Ok(output)
    }
}
