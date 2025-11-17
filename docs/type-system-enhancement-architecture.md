# Type System Enhancement Architecture

## Overview
Enhance Crucible's type validation to support real-world TypeScript patterns while maintaining backward compatibility.

## Architecture Pattern
**Layered Architecture** with clear separation of concerns:

```
validator (application)
    ↓
type-system (domain)
    ↓
types (primitives)
```

## Core Modules

### 1. `type-system` (NEW - Domain Layer)
**Responsibility**: Type parsing, validation logic, and type system rules

**Exports**:
- `TypeReference` - Unified type reference structure
- `TypeParser` - Parses type strings into TypeReference
- `TypeValidator` - Validates types against modules
- `BuiltInTypeRegistry` - Registry of built-in types
- `GenericTypeRegistry` - Registry of generic types

**Design Principles**:
- **Single Responsibility**: Each component handles one aspect
- **Open/Closed**: Extensible for new type systems
- **Dependency Inversion**: Depends on abstractions (TypeReference)

### 2. `types` (ENHANCED - Primitives Layer)
**Changes**:
- Add `nullable` field to `ReturnType`
- Add `type_args` field to `ReturnType` for generics
- Add `items` field already exists for array support

### 3. `validator` (ENHANCED - Application Layer)
**Changes**:
- Use `TypeParser` to parse type strings
- Use `TypeValidator` to validate parsed types
- Delegate type system logic to `type-system` module

## Type Reference Structure

```rust
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
```

## Supported Type Patterns

### 1. Built-in Types (CRITICAL)
```json
{"type": "string"}    // Primitive
{"type": "Date"}      // Built-in object
{"type": "Buffer"}    // Node.js built-in
{"type": "object"}    // Generic object
```

**Built-in Registry**:
- Primitives: `string`, `number`, `boolean`, `void`
- Objects: `Date`, `Buffer`, `Error`
- Special: `object`, `any`, `unknown`, `null`, `undefined`

### 2. Nullable Types (CRITICAL)
```json
// NEW syntax
{"type": "patient.Patient", "nullable": true}

// Replaces union syntax (not supported yet)
// "patient.Patient | null"
```

### 3. Array Types (CRITICAL)
```json
// Shorthand (NEW)
{"type": "patient.Patient[]"}

// Long form (EXISTING)
{"type": "array", "items": "patient.Patient"}
```

**Parser Logic**:
```
if type.endsWith("[]") then
    base_type = "array"
    items = TypeReference::parse(type[0..-2])
```

### 4. Generic Types (HIGH PRIORITY)
```json
{
  "type": "Partial",
  "typeArgs": ["patient.Patient"]
}
```

**Generic Registry**:
- `Partial<T>` - Makes all properties optional
- `Omit<T, K>` - Omits keys from type
- `Pick<T, K>` - Picks keys from type
- `Record<K, V>` - Key-value mapping
- `Promise<T>` - Async promise

**Validation Strategy**:
- `passthrough`: Just validate type args exist
- `validate-inner`: Recursively validate type args

## Implementation Plan (TDD)

### Phase 1: Built-in Types
1. **Test**: Built-in type registry recognizes types
2. **Test**: Validator accepts built-in types
3. **Implement**: `BuiltInTypeRegistry`
4. **Implement**: Update `TypeValidator`

### Phase 2: Nullable Types
1. **Test**: Parse `{"type": "T", "nullable": true}`
2. **Test**: Validator accepts null for nullable types
3. **Implement**: Add `nullable` to `ReturnType`
4. **Implement**: Update `TypeParser` and `TypeValidator`

### Phase 3: Array Syntax
1. **Test**: Parse `"Type[]"` as array
2. **Test**: Validator resolves array item types
3. **Implement**: Array syntax parsing in `TypeParser`
4. **Implement**: Recursive type validation

### Phase 4: Generic Types
1. **Test**: Parse generic type with args
2. **Test**: Validator handles Partial, Omit, etc.
3. **Implement**: `GenericTypeRegistry`
4. **Implement**: Generic type validation

## Backward Compatibility

✅ **All changes are additive**:
- New optional fields (`nullable`, `typeArgs`)
- Existing syntax still works
- No breaking changes to existing modules

## Success Criteria

1. ✅ Healthcare project validates successfully
2. ✅ All existing tests pass
3. ✅ New tests cover all type patterns
4. ✅ Zero breaking changes
5. ✅ Performance: <10ms overhead per module

## Non-Goals (Future Work)

- ❌ Union types beyond nullable (e.g., `string | number`)
- ❌ Intersection types (`A & B`)
- ❌ Conditional types
- ❌ Mapped types
- ❌ Full TypeScript type system

## Module Dependencies

```
validator
  ↓ depends on
type-system
  ↓ depends on
types (ReturnType, Parameter, Property)
```

## Testing Strategy

### Unit Tests (`type_system.rs`)
- Type parsing: `"Type[]"` → `TypeReference`
- Built-in recognition: `is_builtin("Buffer")`
- Generic parsing: `Partial<T>` → `GenericType`

### Integration Tests (`type_validation_test.rs`)
- End-to-end type validation
- Healthcare module validation
- Backward compatibility

### Validation Tests
- Healthcare project passes validation
- Existing example projects still pass
