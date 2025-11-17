//! Type system for enhanced TypeScript type validation
//!
//! Supports:
//! - Built-in types (string, number, Date, Buffer, etc.)
//! - Nullable types (Type | null)
//! - Array syntax (Type[] and array with items)
//! - Generic types (Partial<T>, Omit<T, K>, etc.)

use crate::types::Module;
use std::collections::HashSet;

/// Unified type reference structure
#[derive(Debug, Clone, PartialEq)]
pub struct TypeReference {
    /// Base type name (e.g., "Patient", "array", "Partial")
    pub base_type: String,

    /// Whether this type can be null
    pub nullable: bool,

    /// For array types - the item type
    pub items: Option<Box<TypeReference>>,

    /// For generic types - type arguments
    pub type_args: Vec<TypeReference>,
}

impl TypeReference {
    /// Create a simple type reference
    pub fn simple(base_type: impl Into<String>) -> Self {
        Self {
            base_type: base_type.into(),
            nullable: false,
            items: None,
            type_args: vec![],
        }
    }

    /// Create a nullable type reference
    pub fn nullable(base_type: impl Into<String>) -> Self {
        Self {
            base_type: base_type.into(),
            nullable: true,
            items: None,
            type_args: vec![],
        }
    }

    /// Create an array type reference
    pub fn array(items: TypeReference) -> Self {
        Self {
            base_type: "array".to_string(),
            nullable: false,
            items: Some(Box::new(items)),
            type_args: vec![],
        }
    }

    /// Create a generic type reference
    pub fn generic(base_type: impl Into<String>, type_args: Vec<TypeReference>) -> Self {
        Self {
            base_type: base_type.into(),
            nullable: false,
            items: None,
            type_args,
        }
    }
}

/// Built-in TypeScript type registry
pub struct BuiltInTypeRegistry {
    types: HashSet<&'static str>,
}

impl BuiltInTypeRegistry {
    /// Create a new built-in type registry with default types
    pub fn new() -> Self {
        let mut types = HashSet::new();

        // Primitives
        types.insert("string");
        types.insert("number");
        types.insert("boolean");
        types.insert("void");
        types.insert("null");
        types.insert("undefined");

        // Objects
        types.insert("Date");
        types.insert("Buffer");
        types.insert("Error");
        types.insert("RegExp");
        types.insert("Map");
        types.insert("Set");

        // Database/Connection types
        types.insert("Connection");
        types.insert("Transaction");
        types.insert("QueryResult");

        // Special
        types.insert("object");
        types.insert("any");
        types.insert("unknown");
        types.insert("never");

        // Common Node.js
        types.insert("Promise");
        types.insert("Array");

        Self { types }
    }

    /// Check if a type name is a built-in type
    pub fn is_builtin(&self, type_name: &str) -> bool {
        self.types.contains(type_name)
    }

    /// Get all built-in type names
    pub fn get_builtins(&self) -> Vec<&'static str> {
        self.types.iter().copied().collect()
    }
}

impl Default for BuiltInTypeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Generic utility type registry
pub struct GenericTypeRegistry {
    generics: HashSet<&'static str>,
}

impl GenericTypeRegistry {
    /// Create a new generic type registry with common TypeScript generics
    pub fn new() -> Self {
        let mut generics = HashSet::new();

        // TypeScript utility types
        generics.insert("Partial");
        generics.insert("Required");
        generics.insert("Readonly");
        generics.insert("Pick");
        generics.insert("Omit");
        generics.insert("Exclude");
        generics.insert("Extract");
        generics.insert("NonNullable");
        generics.insert("ReturnType");
        generics.insert("InstanceType");
        generics.insert("Parameters");
        generics.insert("ConstructorParameters");

        // Common generics
        generics.insert("Record");
        generics.insert("Promise");
        generics.insert("Array");
        generics.insert("Vec");  // Rust-style array
        generics.insert("Map");
        generics.insert("HashMap");
        generics.insert("Set");
        generics.insert("HashSet");
        generics.insert("Option");
        generics.insert("Result");

        Self { generics }
    }

    /// Check if a type name is a generic type
    pub fn is_generic(&self, type_name: &str) -> bool {
        self.generics.contains(type_name)
    }

    /// Validate a generic type reference
    pub fn validate_generic(
        &self,
        type_ref: &TypeReference,
        modules: &[Module],
        validator: &TypeValidator,
    ) -> Result<(), String> {
        if !self.is_generic(&type_ref.base_type) {
            return Err(format!("Unknown generic type: {}", type_ref.base_type));
        }

        // Validate all type arguments
        for arg in &type_ref.type_args {
            validator.validate_type_exists(arg, modules)?;
        }

        Ok(())
    }
}

impl Default for GenericTypeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Type parser - parses type strings into TypeReference
pub struct TypeParser {}

impl TypeParser {
    /// Create a new type parser
    pub fn new() -> Self {
        Self {}
    }

    /// Parse a type string into a TypeReference
    pub fn parse(&self, type_str: &str) -> Result<TypeReference, String> {
        // Handle array shorthand syntax (Type[])
        if type_str.ends_with("[]") {
            return self.parse_array_syntax(type_str);
        }

        // Handle angle bracket generic syntax (Promise<T>, Map<K,V>)
        if type_str.contains('<') && type_str.contains('>') {
            return self.parse_angle_bracket_generic(type_str);
        }

        // Handle union types (Type | null, Type | undefined)
        if type_str.contains(" | ") {
            return self.parse_union_type(type_str);
        }

        // Simple type
        Ok(TypeReference::simple(type_str))
    }

    /// Parse union type syntax (Type | null, Type | undefined)
    /// For now, only handle nullable unions (Type | null, Type | undefined)
    fn parse_union_type(&self, type_str: &str) -> Result<TypeReference, String> {
        let parts: Vec<&str> = type_str.split(" | ").map(|s| s.trim()).collect();

        // Check if this is a nullable union (Type | null or Type | undefined)
        if parts.len() == 2 {
            if parts[1] == "null" || parts[1] == "undefined" {
                // Parse the base type and mark as nullable
                let mut type_ref = self.parse(parts[0])?;
                type_ref.nullable = true;
                return Ok(type_ref);
            }
            if parts[0] == "null" || parts[0] == "undefined" {
                // null | Type (reversed order)
                let mut type_ref = self.parse(parts[1])?;
                type_ref.nullable = true;
                return Ok(type_ref);
            }
        }

        // Other union types not yet supported
        Err(format!(
            "Union type '{}' not supported yet (only Type | null is supported)",
            type_str
        ))
    }

    /// Parse angle bracket generic syntax (Promise<T>, Vec<string>, Map<K, V>)
    fn parse_angle_bracket_generic(&self, type_str: &str) -> Result<TypeReference, String> {
        // Find the base type and type arguments
        let open_bracket = type_str.find('<').ok_or("Missing opening bracket")?;
        let close_bracket = type_str.rfind('>').ok_or("Missing closing bracket")?;

        let base_type = &type_str[..open_bracket];
        let args_str = &type_str[open_bracket + 1..close_bracket];

        // Parse type arguments (split by comma, but handle nested generics)
        let mut type_args = Vec::new();
        let mut current_arg = String::new();
        let mut depth = 0;

        for ch in args_str.chars() {
            match ch {
                '<' => {
                    depth += 1;
                    current_arg.push(ch);
                }
                '>' => {
                    depth -= 1;
                    current_arg.push(ch);
                }
                ',' if depth == 0 => {
                    let arg = current_arg.trim();
                    if !arg.is_empty() {
                        type_args.push(self.parse(arg)?);
                    }
                    current_arg.clear();
                }
                _ => current_arg.push(ch),
            }
        }

        // Don't forget the last argument
        let arg = current_arg.trim();
        if !arg.is_empty() {
            type_args.push(self.parse(arg)?);
        }

        Ok(TypeReference {
            base_type: base_type.to_string(),
            nullable: false,
            items: None,
            type_args,
        })
    }

    /// Parse array shorthand syntax (Type[])
    pub fn parse_array_syntax(&self, type_str: &str) -> Result<TypeReference, String> {
        if !type_str.ends_with("[]") {
            return Err(format!("Expected array syntax, got: {}", type_str));
        }

        // Remove [] suffix
        let base = &type_str[..type_str.len() - 2];

        // Recursively parse the base type (handles nested arrays)
        let items = self.parse(base)?;

        Ok(TypeReference::array(items))
    }

    /// Parse a type reference from JSON-like structure
    pub fn parse_from_json(
        &self,
        base_type: &str,
        nullable: Option<bool>,
        items: Option<&str>,
        type_args: Option<&[String]>,
    ) -> Result<TypeReference, String> {
        let mut type_ref = TypeReference {
            base_type: base_type.to_string(),
            nullable: nullable.unwrap_or(false),
            items: None,
            type_args: vec![],
        };

        // Handle array items
        if let Some(items_str) = items {
            let items_ref = self.parse(items_str)?;
            type_ref.items = Some(Box::new(items_ref));
        }

        // Handle generic type arguments
        if let Some(args) = type_args {
            for arg in args {
                type_ref.type_args.push(self.parse(arg)?);
            }
        }

        Ok(type_ref)
    }
}

impl Default for TypeParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Type validator - validates types against available modules
pub struct TypeValidator {
    builtin_registry: BuiltInTypeRegistry,
    generic_registry: GenericTypeRegistry,
    parser: TypeParser,
}

impl TypeValidator {
    /// Create a new type validator
    pub fn new() -> Self {
        Self {
            builtin_registry: BuiltInTypeRegistry::new(),
            generic_registry: GenericTypeRegistry::new(),
            parser: TypeParser::new(),
        }
    }

    /// Validate that a type exists in the available modules or is built-in
    pub fn validate_type_exists(
        &self,
        type_ref: &TypeReference,
        modules: &[Module],
    ) -> Result<(), String> {
        // Check if it's a built-in type
        if self.builtin_registry.is_builtin(&type_ref.base_type) {
            return Ok(());
        }

        // Check if it's a generic type
        if self.generic_registry.is_generic(&type_ref.base_type) {
            return self
                .generic_registry
                .validate_generic(type_ref, modules, self);
        }

        // Handle array types
        if type_ref.base_type == "array" {
            if let Some(items) = &type_ref.items {
                return self.validate_type_exists(items, modules);
            }
            return Err("Array type must specify items".to_string());
        }

        // Check module exports
        self.validate_module_type(&type_ref.base_type, modules)
    }

    /// Validate a type string (helper method)
    pub fn validate_type_string(
        &self,
        type_str: &str,
        nullable: Option<bool>,
        modules: &[Module],
    ) -> Result<(), String> {
        let mut type_ref = self.parser.parse(type_str)?;
        if let Some(n) = nullable {
            type_ref.nullable = n;
        }
        self.validate_type_exists(&type_ref, modules)
    }

    /// Validate that a type exists in module exports
    fn validate_module_type(&self, type_name: &str, modules: &[Module]) -> Result<(), String> {
        // Handle module-qualified types (module.Type)
        if type_name.contains('.') {
            let parts: Vec<&str> = type_name.split('.').collect();
            if parts.len() != 2 {
                return Err(format!("Invalid type reference: {}", type_name));
            }

            let module_name = parts[0];
            let export_name = parts[1];

            // Find the module
            let module = modules
                .iter()
                .find(|m| m.module == module_name)
                .ok_or_else(|| format!("Module '{}' not found", module_name))?;

            // Check if export exists
            if module.exports.contains_key(export_name) {
                return Ok(());
            }

            return Err(format!(
                "Export '{}' not found in module '{}'",
                export_name, module_name
            ));
        }

        // Unqualified type - search all modules for this export
        // This allows for simpler type references when using dependencies
        for module in modules {
            if module.exports.contains_key(type_name) {
                return Ok(());
            }
        }

        Err(format!(
            "Type '{}' not found in any module (consider using module.Type syntax)",
            type_name
        ))
    }

    /// Get the location of a type (module name)
    pub fn get_type_location(&self, type_name: &str, _modules: &[Module]) -> Option<String> {
        if self.builtin_registry.is_builtin(type_name) {
            return Some("built-in".to_string());
        }

        if self.generic_registry.is_generic(type_name) {
            return Some("generic".to_string());
        }

        if type_name.contains('.') {
            let parts: Vec<&str> = type_name.split('.').collect();
            if parts.len() == 2 {
                return Some(parts[0].to_string());
            }
        }

        None
    }
}

impl Default for TypeValidator {
    fn default() -> Self {
        Self::new()
    }
}

// Convenience functions for testing

pub fn is_builtin_type(type_name: &str) -> bool {
    BuiltInTypeRegistry::new().is_builtin(type_name)
}

pub fn is_generic_type(type_name: &str) -> bool {
    GenericTypeRegistry::new().is_generic(type_name)
}

pub fn parse_type_string(type_str: &str) -> Result<TypeReference, String> {
    TypeParser::new().parse(type_str)
}

pub fn validate_type_string(
    type_str: &str,
    nullable: Option<bool>,
    modules: &[Module],
) -> Result<(), String> {
    TypeValidator::new().validate_type_string(type_str, nullable, modules)
}

pub fn validate_type_reference(
    type_ref: &TypeReference,
    modules: &[Module],
) -> Result<(), String> {
    TypeValidator::new().validate_type_exists(type_ref, modules)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builtin_registry() {
        let registry = BuiltInTypeRegistry::new();
        assert!(registry.is_builtin("string"));
        assert!(registry.is_builtin("Date"));
        assert!(registry.is_builtin("Buffer"));
        assert!(!registry.is_builtin("CustomType"));
    }

    #[test]
    fn test_generic_registry() {
        let registry = GenericTypeRegistry::new();
        assert!(registry.is_generic("Partial"));
        assert!(registry.is_generic("Promise"));
        assert!(!registry.is_generic("CustomType"));
    }

    #[test]
    fn test_parse_simple_type() {
        let parser = TypeParser::new();
        let result = parser.parse("Patient").unwrap();
        assert_eq!(result.base_type, "Patient");
        assert!(!result.nullable);
        assert!(result.items.is_none());
    }

    #[test]
    fn test_parse_array_syntax() {
        let parser = TypeParser::new();
        let result = parser.parse("Patient[]").unwrap();
        assert_eq!(result.base_type, "array");
        assert!(result.items.is_some());
        let items = result.items.unwrap();
        assert_eq!(items.base_type, "Patient");
    }

    #[test]
    fn test_parse_nested_array() {
        let parser = TypeParser::new();
        let result = parser.parse("Patient[][]").unwrap();
        assert_eq!(result.base_type, "array");
        let items = result.items.unwrap();
        assert_eq!(items.base_type, "array");
        let nested = items.items.unwrap();
        assert_eq!(nested.base_type, "Patient");
    }
}
