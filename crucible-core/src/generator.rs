//! Code generator for Crucible architectures

use crate::error::{CrucibleError, Result};
use crate::types::{ExportType, Module, Project};
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
        fs::create_dir_all(output_dir).map_err(|e| CrucibleError::FileRead {
            path: output_dir.display().to_string(),
            source: e,
        })?;

        for module in &self.project.modules {
            let content = self.generate_typescript_module(module)?;
            let file_path = output_dir.join(format!("{}.ts", module.module));

            fs::write(&file_path, content).map_err(|e| CrucibleError::FileRead {
                path: file_path.display().to_string(),
                source: e,
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
                    output.push_str(&format!("export interface {name} {{\n"));
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
                    output.push_str(&format!("export class {name} {{\n"));
                    if let Some(methods) = &export.methods {
                        for (method_name, method) in methods {
                            output.push_str(&format!("  {method_name}("));

                            // Parameters
                            let params: Vec<String> = method
                                .inputs
                                .iter()
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
                            output.push_str(&format!("export function {name}("));

                            // Parameters
                            let params: Vec<String> = method
                                .inputs
                                .iter()
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
                    output.push_str(&format!("export enum {name} {{\n"));
                    if let Some(values) = &export.values {
                        for value in values {
                            output.push_str(&format!("  {value} = '{value}',\n"));
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;
    use std::collections::HashMap;
    use tempfile::tempdir;

    fn create_test_manifest() -> Manifest {
        Manifest {
            version: "0.1.0".to_string(),
            project: ProjectConfig {
                name: "test".to_string(),
                language: Language::TypeScript,
                architecture_pattern: Some(ArchitecturePattern::Layered),
            },
            modules: vec![],
            strict_validation: true,
            metadata: None,
        }
    }

    #[test]
    fn test_generate_interface() {
        let mut props = HashMap::new();
        props.insert(
            "id".to_string(),
            Property {
                prop_type: "string".to_string(),
                required: true,
                description: None,
            },
        );
        props.insert(
            "count".to_string(),
            Property {
                prop_type: "number".to_string(),
                required: false,
                description: None,
            },
        );

        let mut exports = HashMap::new();
        exports.insert(
            "User".to_string(),
            Export {
                export_type: ExportType::Interface,
                methods: None,
                properties: Some(props),
                values: None,
                dependencies: None,
            },
        );

        let module = Module {
            module: "user".to_string(),
            version: "1.0.0".to_string(),
            layer: None,
            description: None,
            exports,
            dependencies: HashMap::new(),
        };

        let project = Project {
            manifest: create_test_manifest(),
            modules: vec![module],
            rules: None,
        };

        let generator = Generator::new(project);
        let module_ref = &generator.project.modules[0];
        let output = generator.generate_typescript_module(module_ref).unwrap();

        assert!(output.contains("export interface User {"));
        assert!(output.contains("id: string;"));
        assert!(output.contains("count?: number;"));
    }

    #[test]
    fn test_generate_class() {
        let inputs = vec![Parameter {
            name: "name".to_string(),
            param_type: "string".to_string(),
            optional: false,
            description: None,
        }];

        let mut methods = HashMap::new();
        methods.insert(
            "greet".to_string(),
            Method {
                inputs,
                returns: ReturnType {
                    return_type: "void".to_string(),
                    inner: None,
                },
                throws: vec![],
                calls: vec![],
                effects: vec![],
            },
        );

        let mut exports = HashMap::new();
        exports.insert(
            "Greeter".to_string(),
            Export {
                export_type: ExportType::Class,
                methods: Some(methods),
                properties: None,
                values: None,
                dependencies: None,
            },
        );

        let module = Module {
            module: "greeter".to_string(),
            version: "1.0.0".to_string(),
            layer: None,
            description: None,
            exports,
            dependencies: HashMap::new(),
        };

        let project = Project {
            manifest: create_test_manifest(),
            modules: vec![module],
            rules: None,
        };

        let generator = Generator::new(project);
        let module_ref = &generator.project.modules[0];
        let output = generator.generate_typescript_module(module_ref).unwrap();

        assert!(output.contains("export class Greeter {"));
        assert!(output.contains("greet(name: string): void {"));
        assert!(output.contains("throw new Error('Not implemented');"));
    }

    #[test]
    fn test_generate_function() {
        let inputs = vec![
            Parameter {
                name: "a".to_string(),
                param_type: "number".to_string(),
                optional: false,
                description: None,
            },
            Parameter {
                name: "b".to_string(),
                param_type: "number".to_string(),
                optional: false,
                description: None,
            },
        ];

        let mut methods = HashMap::new();
        methods.insert(
            "add".to_string(),
            Method {
                inputs,
                returns: ReturnType {
                    return_type: "number".to_string(),
                    inner: None,
                },
                throws: vec![],
                calls: vec![],
                effects: vec![],
            },
        );

        let mut exports = HashMap::new();
        exports.insert(
            "add".to_string(),
            Export {
                export_type: ExportType::Function,
                methods: Some(methods),
                properties: None,
                values: None,
                dependencies: None,
            },
        );

        let module = Module {
            module: "math".to_string(),
            version: "1.0.0".to_string(),
            layer: None,
            description: None,
            exports,
            dependencies: HashMap::new(),
        };

        let project = Project {
            manifest: create_test_manifest(),
            modules: vec![module],
            rules: None,
        };

        let generator = Generator::new(project);
        let module_ref = &generator.project.modules[0];
        let output = generator.generate_typescript_module(module_ref).unwrap();

        assert!(output.contains("export function add(a: number, b: number): number {"));
        assert!(output.contains("throw new Error('Not implemented');"));
    }

    #[test]
    fn test_generate_enum() {
        let mut exports = HashMap::new();
        exports.insert(
            "Status".to_string(),
            Export {
                export_type: ExportType::Enum,
                methods: None,
                properties: None,
                values: Some(vec![
                    "Active".to_string(),
                    "Inactive".to_string(),
                    "Pending".to_string(),
                ]),
                dependencies: None,
            },
        );

        let module = Module {
            module: "status".to_string(),
            version: "1.0.0".to_string(),
            layer: None,
            description: None,
            exports,
            dependencies: HashMap::new(),
        };

        let project = Project {
            manifest: create_test_manifest(),
            modules: vec![module],
            rules: None,
        };

        let generator = Generator::new(project);
        let module_ref = &generator.project.modules[0];
        let output = generator.generate_typescript_module(module_ref).unwrap();

        assert!(output.contains("export enum Status {"));
        assert!(output.contains("Active = 'Active',"));
        assert!(output.contains("Inactive = 'Inactive',"));
        assert!(output.contains("Pending = 'Pending',"));
    }

    #[test]
    fn test_generate_module_header() {
        let module = Module {
            module: "test-module".to_string(),
            version: "2.1.5".to_string(),
            layer: None,
            description: None,
            exports: HashMap::new(),
            dependencies: HashMap::new(),
        };

        let project = Project {
            manifest: create_test_manifest(),
            modules: vec![module],
            rules: None,
        };

        let generator = Generator::new(project);
        let module_ref = &generator.project.modules[0];
        let output = generator.generate_typescript_module(module_ref).unwrap();

        assert!(output.contains("// Generated from Crucible module: test-module"));
        assert!(output.contains("// Version: 2.1.5"));
    }

    #[test]
    fn test_generate_typescript_to_file() {
        let dir = tempdir().unwrap();

        let module = Module {
            module: "test".to_string(),
            version: "1.0.0".to_string(),
            layer: None,
            description: None,
            exports: HashMap::new(),
            dependencies: HashMap::new(),
        };

        let project = Project {
            manifest: create_test_manifest(),
            modules: vec![module],
            rules: None,
        };

        let generator = Generator::new(project);
        generator.generate_typescript(dir.path()).unwrap();

        let file_path = dir.path().join("test.ts");
        assert!(file_path.exists());

        let content = std::fs::read_to_string(file_path).unwrap();
        assert!(content.contains("// Generated from Crucible module: test"));
    }

    #[test]
    fn test_generate_multiple_modules() {
        let dir = tempdir().unwrap();

        let module1 = Module {
            module: "module1".to_string(),
            version: "1.0.0".to_string(),
            layer: None,
            description: None,
            exports: HashMap::new(),
            dependencies: HashMap::new(),
        };

        let module2 = Module {
            module: "module2".to_string(),
            version: "1.0.0".to_string(),
            layer: None,
            description: None,
            exports: HashMap::new(),
            dependencies: HashMap::new(),
        };

        let project = Project {
            manifest: create_test_manifest(),
            modules: vec![module1, module2],
            rules: None,
        };

        let generator = Generator::new(project);
        generator.generate_typescript(dir.path()).unwrap();

        assert!(dir.path().join("module1.ts").exists());
        assert!(dir.path().join("module2.ts").exists());
    }

    #[test]
    fn test_generate_class_no_parameters() {
        let mut methods = HashMap::new();
        methods.insert(
            "execute".to_string(),
            Method {
                inputs: vec![],
                returns: ReturnType {
                    return_type: "void".to_string(),
                    inner: None,
                },
                throws: vec![],
                calls: vec![],
                effects: vec![],
            },
        );

        let mut exports = HashMap::new();
        exports.insert(
            "Command".to_string(),
            Export {
                export_type: ExportType::Class,
                methods: Some(methods),
                properties: None,
                values: None,
                dependencies: None,
            },
        );

        let module = Module {
            module: "command".to_string(),
            version: "1.0.0".to_string(),
            layer: None,
            description: None,
            exports,
            dependencies: HashMap::new(),
        };

        let project = Project {
            manifest: create_test_manifest(),
            modules: vec![module],
            rules: None,
        };

        let generator = Generator::new(project);
        let module_ref = &generator.project.modules[0];
        let output = generator.generate_typescript_module(module_ref).unwrap();

        assert!(output.contains("execute(): void {"));
    }
}
