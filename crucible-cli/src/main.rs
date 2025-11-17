use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use crucible_core::claude::{
    ContextGenerator, IntegrationConfig, IntegrationMode, SyncManager, ValidationHooks,
    ValidationLevel,
};
use crucible_core::{Generator, Parser as CrucibleParser, Validator};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new Crucible project
    Init {
        /// Project name (required unless --here is used)
        #[arg(long)]
        name: Option<String>,

        /// Initialize in current directory
        #[arg(long)]
        here: bool,

        /// Force overwrite if .crucible/ already exists
        #[arg(long)]
        force: bool,

        /// Programming language
        #[arg(long, default_value = "typescript")]
        language: String,

        /// Architecture pattern
        #[arg(long, default_value = "layered")]
        pattern: String,
    },

    /// Validate the architecture
    Validate {
        /// Path to .crucible directory
        #[arg(long, default_value = ".crucible")]
        path: PathBuf,

        /// Enable strict validation
        #[arg(long)]
        strict: bool,
    },

    /// Generate code from architecture
    Generate {
        /// Path to .crucible directory
        #[arg(long, default_value = ".crucible")]
        path: PathBuf,

        /// Target language
        #[arg(long)]
        lang: String,

        /// Output directory
        #[arg(long, default_value = "./generated")]
        output: PathBuf,
    },

    /// Show dependency graph
    Graph {
        /// Output format (text, dot, svg)
        #[arg(long, default_value = "text")]
        format: String,
    },

    /// Claude Code integration commands
    Claude {
        #[command(subcommand)]
        command: ClaudeCommands,
    },
}

#[derive(Subcommand)]
enum ClaudeCommands {
    /// Initialize Claude Code integration
    Init {
        /// Integration mode
        #[arg(long, default_value = "enhanced")]
        mode: String,

        /// Enable globally
        #[arg(long)]
        global: bool,

        /// Validation level
        #[arg(long, default_value = "warning")]
        validation: String,
    },

    /// Sync architecture with code
    Sync {
        /// Sync from code to architecture
        #[arg(long)]
        from_code: bool,

        /// Sync from architecture to code
        #[arg(long)]
        from_architecture: bool,

        /// Interactive mode with prompts
        #[arg(long, short)]
        interactive: bool,
    },

    /// Validate with Claude-friendly output
    Validate {
        /// Specific module to validate
        module: Option<String>,
    },

    /// Generate Claude context files
    Context {
        /// Output format
        #[arg(long, default_value = "json")]
        format: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init {
            name,
            here,
            force,
            language,
            pattern,
        } => {
            if !here && name.is_none() {
                eprintln!(
                    "{} Either --name or --here must be specified",
                    "Error:".red().bold()
                );
                eprintln!("  {}", "crucible init --name my-project".cyan());
                eprintln!("  {}", "crucible init --here".cyan());
                std::process::exit(1);
            }
            init_project(name.as_deref(), here, force, &language, &pattern)?;
        }
        Commands::Validate { path, strict } => {
            validate_project(&path, strict)?;
        }
        Commands::Generate { path, lang, output } => {
            generate_code(&path, &lang, &output)?;
        }
        Commands::Graph { format } => {
            println!("Graph generation not yet implemented");
            println!("Format: {format}");
        }
        Commands::Claude { command } => match command {
            ClaudeCommands::Init {
                mode,
                global,
                validation,
            } => {
                claude_init(&mode, &global, &validation)?;
            }
            ClaudeCommands::Sync {
                from_code,
                from_architecture,
                interactive,
            } => {
                claude_sync(&from_code, &from_architecture, &interactive)?;
            }
            ClaudeCommands::Validate { module } => {
                claude_validate(module.as_deref())?;
            }
            ClaudeCommands::Context { format } => {
                claude_context(&format)?;
            }
        },
    }

    Ok(())
}

fn confirm_overwrite(path_description: &str) -> Result<bool> {
    eprintln!(
        "{} Replacing existing .crucible/ directory{}",
        "Warning:".yellow().bold(),
        if !path_description.is_empty() {
            format!(" in '{path_description}'")
        } else {
            String::new()
        }
    );
    eprintln!(
        "  {} This will delete all existing architecture definitions!",
        "⚠️ ".yellow()
    );
    eprintln!();
    eprint!("  Type 'yes' to continue: ");
    io::stderr().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let confirmed = input.trim().eq_ignore_ascii_case("yes");
    if !confirmed {
        eprintln!();
        eprintln!("{} Operation cancelled", "Aborted:".red().bold());
    }
    eprintln!();

    Ok(confirmed)
}

fn init_project(
    name: Option<&str>,
    here: bool,
    force: bool,
    language: &str,
    pattern: &str,
) -> Result<()> {
    let (project_path, project_name) = if here {
        // Initialize in current directory
        let current_dir = std::env::current_dir()?;
        let dir_name = current_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("project")
            .to_string();

        // Check if .crucible/ already exists
        let crucible_path = current_dir.join(".crucible");
        if crucible_path.exists() {
            if !force {
                eprintln!(
                    "{} .crucible/ directory already exists in current directory",
                    "Error:".red().bold()
                );
                eprintln!();
                eprintln!("Options:");
                eprintln!("  1. Use {} to overwrite", "--force".cyan());
                eprintln!("  2. Remove existing .crucible/ directory first");
                eprintln!("  3. Initialize in a different directory");
                eprintln!();
                eprintln!("Example:");
                eprintln!("  {}", "crucible init --here --force".cyan());
                std::process::exit(1);
            } else {
                // Prompt for confirmation before removing existing .crucible/ directory
                if !confirm_overwrite("")? {
                    std::process::exit(1);
                }

                // Remove existing .crucible/ directory when --force is used
                std::fs::remove_dir_all(&crucible_path)?;
            }
        }

        println!(
            "{}  Crucible in current directory: {}",
            "Initializing".green().bold(),
            dir_name
        );

        (current_dir, dir_name)
    } else {
        // Create new project directory
        let name = name.unwrap(); // Safe because we validated earlier
        let project_dir = std::path::PathBuf::from(name);

        // Check if directory already exists
        if project_dir.exists() {
            let crucible_path = project_dir.join(".crucible");
            if crucible_path.exists() {
                if !force {
                    eprintln!(
                        "{} Directory '{}' already has .crucible/ directory",
                        "Error:".red().bold(),
                        name
                    );
                    eprintln!();
                    eprintln!("Options:");
                    eprintln!("  1. Use {} to overwrite", "--force".cyan());
                    eprintln!("  2. Choose a different project name");
                    eprintln!("  3. Remove existing .crucible/ directory first");
                    eprintln!();
                    eprintln!("Example:");
                    eprintln!(
                        "  {}",
                        format!("crucible init --name {name} --force").cyan()
                    );
                    std::process::exit(1);
                } else {
                    // Prompt for confirmation before removing existing .crucible/ directory
                    if !confirm_overwrite(name)? {
                        std::process::exit(1);
                    }

                    // Remove existing .crucible/ directory when --force is used
                    std::fs::remove_dir_all(&crucible_path)?;
                }
            } else if !force {
                eprintln!(
                    "{} Directory '{}' already exists",
                    "Warning:".yellow().bold(),
                    name
                );
                eprintln!("  Initializing Crucible in existing directory...");
                eprintln!();
            }
        }

        println!(
            "{}  Crucible project: {}",
            "Initializing".green().bold(),
            name
        );

        std::fs::create_dir_all(name)?;
        (project_dir, name.to_string())
    };

    // Create .crucible directory inside project
    std::fs::create_dir_all(project_path.join(".crucible/modules"))?;
    std::fs::create_dir_all(project_path.join(".crucible/types"))?;

    // Create manifest.json with example modules
    let manifest = format!(
        r#"{{
  "version": "0.1.0",
  "project": {{
    "name": "{project_name}",
    "language": "{language}",
    "architecture_pattern": "{pattern}"
  }},
  "modules": [
    "user",
    "user-service",
    "user-controller"
  ],
  "strict_validation": true
}}"#
    );

    std::fs::write(project_path.join(".crucible/manifest.json"), manifest)?;

    // Create rules.json
    let rules = r#"{
  "architecture": {
    "pattern": "layered",
    "layers": [
      {"name": "presentation", "can_depend_on": ["application"]},
      {"name": "application", "can_depend_on": ["domain"]},
      {"name": "domain", "can_depend_on": []}
    ]
  },
  "rules": [
    {
      "id": "no-circular-dependencies",
      "enabled": true,
      "severity": "error"
    }
  ]
}"#;

    std::fs::write(project_path.join(".crucible/rules.json"), rules)?;

    // Create example modules
    create_example_modules(&project_path)?;

    // Create README in modules directory
    create_modules_readme(&project_path)?;

    // Create Claude Code slash commands
    create_claude_commands(&project_path)?;

    let display_path = if here { "." } else { &project_name };

    println!("{} Created {}/.crucible/", "✓".green(), display_path);
    println!(
        "{} Created {}/.crucible/manifest.json",
        "✓".green(),
        display_path
    );
    println!(
        "{} Created {}/.crucible/rules.json",
        "✓".green(),
        display_path
    );
    println!(
        "{} Created {}/.crucible/modules/",
        "✓".green(),
        display_path
    );
    println!(
        "{} Created 3 example modules (user, user-service, user-controller)",
        "✓".green()
    );
    println!("{} Created {}/.claude/commands/", "✓".green(), display_path);
    println!("{} Created 8 Claude Code slash commands", "✓".green());
    println!();
    println!("{}", "Project initialized successfully!".green().bold());
    println!();
    println!("Next steps:");
    if !here {
        println!("  1. {}", format!("cd {project_name}").cyan());
        println!("  2. Review example modules in .crucible/modules/");
    } else {
        println!("  1. Review example modules in .crucible/modules/");
    }
    println!(
        "  {}. Read {} for guidance",
        if here { "2" } else { "3" },
        ".crucible/modules/README.md".cyan()
    );
    println!(
        "  {}. Run {} to validate",
        if here { "3" } else { "4" },
        "crucible validate".cyan()
    );
    println!(
        "  {}. Try Claude Code commands: type '/' and look for 'crucible:' commands",
        if here { "4" } else { "5" }
    );
    println!(
        "  {}. Customize modules or add your own",
        if here { "5" } else { "6" }
    );

    Ok(())
}

fn create_example_modules(project_path: &Path) -> Result<()> {
    // Domain layer: User entity
    let user_module = r#"{
  "module": "user",
  "version": "1.0.0",
  "layer": "domain",
  "description": "User domain entity - core business model",
  "exports": {
    "User": {
      "type": "type",
      "description": "User entity with business rules",
      "properties": {
        "id": {"type": "string", "required": true},
        "email": {"type": "string", "required": true},
        "name": {"type": "string", "required": true},
        "createdAt": {"type": "Date", "required": true},
        "isActive": {"type": "boolean", "required": true}
      }
    },
    "UserRole": {
      "type": "enum",
      "description": "User role enumeration",
      "values": ["admin", "user", "guest"]
    },
    "validateEmail": {
      "type": "function",
      "description": "Validates email format",
      "inputs": [
        {"name": "email", "type": "string", "optional": false}
      ],
      "returns": {"type": "boolean"},
      "throws": [],
      "calls": [],
      "effects": []
    }
  },
  "dependencies": {}
}"#;

    std::fs::write(
        project_path.join(".crucible/modules/user.json"),
        user_module,
    )?;

    // Application layer: User service
    let user_service_module = r#"{
  "module": "user-service",
  "version": "1.0.0",
  "layer": "application",
  "description": "User service - application logic and use cases",
  "exports": {
    "UserService": {
      "type": "class",
      "description": "Handles user-related business operations",
      "methods": {
        "createUser": {
          "inputs": [
            {"name": "email", "type": "string", "optional": false},
            {"name": "name", "type": "string", "optional": false}
          ],
          "returns": {"type": "Promise<user.User>"},
          "throws": ["InvalidEmailError", "UserAlreadyExistsError"],
          "calls": ["user.validateEmail"],
          "effects": ["creates user in database"]
        },
        "getUserById": {
          "inputs": [
            {"name": "id", "type": "string", "optional": false}
          ],
          "returns": {"type": "user.User | null"},
          "throws": [],
          "calls": [],
          "effects": []
        },
        "getAllUsers": {
          "inputs": [],
          "returns": {"type": "user.User[]"},
          "throws": [],
          "calls": [],
          "effects": []
        },
        "updateUser": {
          "inputs": [
            {"name": "id", "type": "string", "optional": false},
            {"name": "updates", "type": "Partial<user.User>", "optional": false}
          ],
          "returns": {"type": "Promise<user.User>"},
          "throws": ["UserNotFoundError"],
          "calls": [],
          "effects": ["updates user in database"]
        },
        "deleteUser": {
          "inputs": [
            {"name": "id", "type": "string", "optional": false}
          ],
          "returns": {"type": "Promise<void>"},
          "throws": ["UserNotFoundError"],
          "calls": [],
          "effects": ["deletes user from database"]
        }
      }
    },
    "InvalidEmailError": {
      "type": "class",
      "description": "Error thrown when email is invalid"
    },
    "UserAlreadyExistsError": {
      "type": "class",
      "description": "Error thrown when user already exists"
    },
    "UserNotFoundError": {
      "type": "class",
      "description": "Error thrown when user is not found"
    }
  },
  "dependencies": {
    "user": "1.0.0"
  }
}"#;

    std::fs::write(
        project_path.join(".crucible/modules/user-service.json"),
        user_service_module,
    )?;

    // Presentation layer: User controller
    let user_controller_module = r#"{
  "module": "user-controller",
  "version": "1.0.0",
  "layer": "presentation",
  "description": "User HTTP/API controller - handles HTTP requests",
  "exports": {
    "HttpRequest": {
      "type": "type",
      "description": "HTTP request object",
      "properties": {
        "body": {"type": "object", "required": true},
        "params": {"type": "object", "required": true}
      }
    },
    "HttpResponse": {
      "type": "type",
      "description": "HTTP response object",
      "properties": {
        "status": {"type": "number", "required": true},
        "body": {"type": "object", "required": true}
      }
    },
    "UserController": {
      "type": "class",
      "description": "RESTful API endpoints for user management",
      "methods": {
        "createUser": {
          "inputs": [
            {"name": "request", "type": "HttpRequest", "optional": false}
          ],
          "returns": {"type": "HttpResponse"},
          "throws": [],
          "calls": ["user-service.UserService.createUser"],
          "effects": ["sends HTTP response"]
        },
        "getUser": {
          "inputs": [
            {"name": "request", "type": "HttpRequest", "optional": false}
          ],
          "returns": {"type": "HttpResponse"},
          "throws": [],
          "calls": ["user-service.UserService.getUserById"],
          "effects": ["sends HTTP response"]
        },
        "updateUser": {
          "inputs": [
            {"name": "request", "type": "HttpRequest", "optional": false}
          ],
          "returns": {"type": "HttpResponse"},
          "throws": [],
          "calls": ["user-service.UserService.updateUser"],
          "effects": ["sends HTTP response"]
        },
        "deleteUser": {
          "inputs": [
            {"name": "request", "type": "HttpRequest", "optional": false}
          ],
          "returns": {"type": "HttpResponse"},
          "throws": [],
          "calls": ["user-service.UserService.deleteUser"],
          "effects": ["sends HTTP response"]
        }
      }
    }
  },
  "dependencies": {
    "user-service": "1.0.0"
  }
}"#;

    std::fs::write(
        project_path.join(".crucible/modules/user-controller.json"),
        user_controller_module,
    )?;

    Ok(())
}

fn create_modules_readme(project_path: &Path) -> Result<()> {
    let readme = r#"# Crucible Modules

This directory contains your architecture module definitions. Each `.json` file describes a module's interface, dependencies, and layer.

## Example Modules

Three example modules are provided to demonstrate the layered architecture pattern:

### 1. `user.json` - Domain Layer

The **domain layer** contains core business entities and rules with no dependencies.

**Key concepts:**
- Pure business logic
- Types and entities (User, UserRole)
- Business rules (validateEmail)
- No dependencies on other layers

**When to use:**
- Defining core business entities
- Business validation rules
- Domain-specific types and enums

### 2. `user-service.json` - Application Layer

The **application layer** implements use cases and coordinates domain entities.

**Key concepts:**
- Use case implementations (CRUD operations)
- Coordinates domain entities
- Can depend on domain layer only
- Contains application-specific errors

**When to use:**
- Implementing business use cases
- Orchestrating domain logic
- Managing transactions
- Application-level validation

### 3. `user-controller.json` - Presentation Layer

The **presentation layer** handles external interfaces (HTTP, CLI, etc.).

**Key concepts:**
- API endpoints / UI handlers
- HTTP request/response handling
- Can depend on application layer
- Adapts external input to application layer

**When to use:**
- REST API endpoints
- GraphQL resolvers
- CLI commands
- WebSocket handlers

## Module Structure

Each module JSON file contains:

```json
{
  "module": "module-name",
  "version": "1.0.0",
  "layer": "domain|application|presentation",
  "description": "What this module does",
  "exports": {
    "ExportName": {
      "type": "class|function|type|enum",
      "description": "What this export does",
      "methods": { /* for classes */ },
      "properties": { /* for types */ },
      "values": [ /* for enums */ ]
    }
  },
  "dependencies": {
    "other-module": "1.0.0"
  }
}
```

## Layer Rules

**Dependency flow (one direction only):**
```
presentation → application → domain
```

- ✅ Presentation can import from application
- ✅ Application can import from domain
- ❌ Domain cannot import from application or presentation
- ❌ Application cannot import from presentation

## Creating Your Own Modules

1. **Copy an example:**
   ```bash
   cp user.json my-module.json
   ```

2. **Edit the file:**
   - Change module name
   - Update layer (domain/application/presentation)
   - Define your exports
   - Declare dependencies

3. **Add to manifest.json:**
   ```json
   "modules": ["user", "user-service", "user-controller", "my-module"]
   ```

4. **Validate:**
   ```bash
   crucible validate
   ```

## Common Export Types

### Class Export
```json
"MyService": {
  "type": "class",
  "description": "Service description",
  "methods": {
    "methodName": {
      "inputs": [{"name": "param", "type": "string"}],
      "returns": {"type": "void"},
      "throws": ["ErrorType"],
      "calls": ["other-module.function"],
      "effects": ["description of side effect"]
    }
  }
}
```

### Function Export
```json
"myFunction": {
  "type": "function",
  "inputs": [{"name": "x", "type": "number"}],
  "returns": {"type": "number"},
  "throws": [],
  "calls": [],
  "effects": []
}
```

### Type Export
```json
"MyType": {
  "type": "type",
  "properties": {
    "id": {"type": "string", "required": true},
    "name": {"type": "string", "required": false}
  }
}
```

### Enum Export
```json
"MyEnum": {
  "type": "enum",
  "values": ["option1", "option2", "option3"]
}
```

## TypeScript Type System

Crucible supports modern TypeScript type patterns:

### Array Types
```json
"getAllUsers": {
  "returns": {"type": "user.User[]"}
}
```

### Nullable Types
```json
"findUser": {
  "returns": {"type": "user.User | null"}
}
```

### Generic Types
```json
"updateUser": {
  "inputs": [
    {"name": "updates", "type": "Partial<user.User>"}
  ],
  "returns": {"type": "Promise<user.User>"}
}
```

**Supported generics:**
- Utility types: `Partial<T>`, `Omit<T, K>`, `Pick<T, K>`, `Record<K, V>`
- Async: `Promise<T>`
- Collections: `Array<T>`, `Map<K,V>`, `Set<T>`

### Built-in Types
```json
"createdAt": {"type": "Date"},
"fileData": {"type": "Buffer"},
"metadata": {"type": "object"}
```

**Common built-ins:**
- Primitives: `string`, `number`, `boolean`, `void`
- Objects: `Date`, `Buffer`, `Error`, `RegExp`
- Database: `Connection`, `Transaction`
- Special: `any`, `unknown`, `object`

## Tips

1. **Start simple:** Begin with one module per layer
2. **Follow examples:** Copy and modify the provided examples
3. **Validate often:** Run `crucible validate` after each change
4. **Think dependencies:** Domain should have none, application depends on domain
5. **Document effects:** List side effects (database writes, API calls, etc.)

## Next Steps

1. Customize the example modules for your use case
2. Add more modules as needed
3. Run `crucible validate` to check architecture
4. Run `crucible claude init` to generate Claude Code integration
5. Start implementing your code following the architecture

For more examples, see:
https://github.com/anvanster/crucible/tree/main/spec/examples/calculator-app/modules
"#;

    std::fs::write(project_path.join(".crucible/modules/README.md"), readme)?;

    Ok(())
}

fn create_claude_commands(project_path: &Path) -> Result<()> {
    // Create .claude/commands directory
    std::fs::create_dir_all(project_path.join(".claude/commands"))?;

    // Include command files from embedded resources
    let crucible_validate = include_str!("../../.claude/commands/crucible-validate.md");
    let crucible_architecture = include_str!("../../.claude/commands/crucible-architecture.md");
    let crucible_init = include_str!("../../.claude/commands/crucible-init.md");
    let crucible_module = include_str!("../../.claude/commands/crucible-module.md");
    let crucible_review = include_str!("../../.claude/commands/crucible-review.md");
    let crucible_sync = include_str!("../../.claude/commands/crucible-sync.md");
    let crucible_analyze = include_str!("../../.claude/commands/crucible-analyze.md");
    let crucible_diff = include_str!("../../.claude/commands/crucible-diff.md");

    // Write command files
    std::fs::write(
        project_path.join(".claude/commands/crucible-validate.md"),
        crucible_validate,
    )?;
    std::fs::write(
        project_path.join(".claude/commands/crucible-architecture.md"),
        crucible_architecture,
    )?;
    std::fs::write(
        project_path.join(".claude/commands/crucible-init.md"),
        crucible_init,
    )?;
    std::fs::write(
        project_path.join(".claude/commands/crucible-module.md"),
        crucible_module,
    )?;
    std::fs::write(
        project_path.join(".claude/commands/crucible-review.md"),
        crucible_review,
    )?;
    std::fs::write(
        project_path.join(".claude/commands/crucible-sync.md"),
        crucible_sync,
    )?;
    std::fs::write(
        project_path.join(".claude/commands/crucible-analyze.md"),
        crucible_analyze,
    )?;
    std::fs::write(
        project_path.join(".claude/commands/crucible-diff.md"),
        crucible_diff,
    )?;

    Ok(())
}

fn validate_project(path: &Path, strict: bool) -> Result<()> {
    println!("{}  architecture...", "Validating".cyan().bold());

    let parser = CrucibleParser::new(path);
    let project = parser.parse_project()?;

    println!("  {} modules found", project.modules.len());

    let validator = Validator::new(project);
    let result = validator.validate();

    // Display results
    for error in &result.errors {
        println!("{} {}: {}", "✗".red(), error.rule.bold(), error.message);
        if let Some(location) = &error.location {
            println!("    at {}", location.dimmed());
        }
    }

    for warning in &result.warnings {
        if strict {
            println!(
                "{} {}: {}",
                "⚠".yellow(),
                warning.rule.bold(),
                warning.message
            );
            if let Some(location) = &warning.location {
                println!("    at {}", location.dimmed());
            }
        }
    }

    println!();
    if result.valid {
        println!("{}", "Architecture is valid!".green().bold());
    } else {
        println!("{}", "Architecture validation failed!".red().bold());
        std::process::exit(1);
    }

    Ok(())
}

fn generate_code(path: &Path, lang: &str, output: &Path) -> Result<()> {
    let parser = CrucibleParser::new(path);
    let project = parser.parse_project()?;

    match lang {
        "typescript" | "ts" => {
            let gen = Generator::new(project);
            gen.generate_typescript(output)?;
            println!("✓ Generated TypeScript interfaces in {}", output.display());
        }
        _ => {
            println!("Language '{lang}' not yet supported");
        }
    }

    Ok(())
}

fn claude_init(mode_str: &str, _global: &bool, validation_str: &str) -> Result<()> {
    println!(
        "{}  Claude Code integration...",
        "Initializing".cyan().bold()
    );

    // Parse mode
    let mode = match mode_str.to_lowercase().as_str() {
        "basic" => IntegrationMode::Basic,
        "enhanced" => IntegrationMode::Enhanced,
        "strict" => IntegrationMode::Strict,
        _ => {
            println!("{} Invalid mode: {}", "✗".red(), mode_str);
            return Ok(());
        }
    };

    // Parse validation level
    let _validation = match validation_str.to_lowercase().as_str() {
        "error" => ValidationLevel::Error,
        "warning" => ValidationLevel::Warning,
        "info" => ValidationLevel::Info,
        _ => {
            println!("{} Invalid validation level: {}", "✗".red(), validation_str);
            return Ok(());
        }
    };

    // Get current directory
    let project_root = std::env::current_dir()?;
    let crucible_path = project_root.join(".crucible");

    // Check if .crucible exists
    if !crucible_path.exists() {
        println!(
            "{} No .crucible directory found. Run {} first.",
            "✗".red(),
            "crucible init".cyan()
        );
        return Ok(());
    }

    // Parse existing project
    let parser = CrucibleParser::new(&crucible_path);
    let project = parser.parse_project()?;

    // Create .claude directory structure
    let claude_dir = project_root.join(".claude");
    let crucible_claude_dir = claude_dir.join("crucible");
    std::fs::create_dir_all(&crucible_claude_dir)?;

    // Create config with overrides
    let mut config = IntegrationConfig::load_with_overrides(None)?;
    config.mode = mode;

    // Generate and write files
    let context_gen = ContextGenerator::new(project, config.clone());

    // Write CRUCIBLE.md
    let instructions = context_gen.generate_instructions();
    std::fs::write(claude_dir.join("CRUCIBLE.md"), instructions)?;
    println!("{} Created .claude/CRUCIBLE.md", "✓".green());

    // Write context.json
    let context_json = context_gen.generate_context_json()?;
    std::fs::write(crucible_claude_dir.join("context.json"), context_json)?;
    println!("{} Created .claude/crucible/context.json", "✓".green());

    // Write hooks.md - need to re-parse project
    let parser2 = CrucibleParser::new(&crucible_path);
    let project_for_hooks = parser2.parse_project()?;
    let validation_hooks = ValidationHooks::new(project_for_hooks);
    let hooks = validation_hooks.generate_hooks();
    std::fs::write(crucible_claude_dir.join("hooks.md"), hooks)?;
    println!("{} Created .claude/crucible/hooks.md", "✓".green());

    // Write config.json
    let config_path = crucible_claude_dir.join("claude.json");
    config.to_file(&config_path)?;
    println!("{} Created .claude/crucible/claude.json", "✓".green());

    println!();
    println!(
        "{} Claude Code integration initialized!",
        "✓".green().bold()
    );
    println!();
    println!("Next steps:");
    println!("  1. Start Claude Code in this directory");
    println!(
        "  2. Claude will automatically read the architecture from {}",
        ".claude/CRUCIBLE.md".cyan()
    );
    println!("  3. Run {} to sync changes", "crucible claude sync".cyan());

    Ok(())
}

fn claude_sync(from_code: &bool, from_architecture: &bool, interactive: &bool) -> Result<()> {
    if *from_architecture {
        println!(
            "{}",
            "Architecture → Code sync not yet implemented".yellow()
        );
        return Ok(());
    }

    if *from_code || (!from_code && !from_architecture) {
        println!("{}  code with architecture...", "Syncing".cyan().bold());

        // Parse current architecture
        let parser = CrucibleParser::new(PathBuf::from(".crucible"));
        let project = parser.parse_project()?;

        // Create sync manager
        let sync_manager = SyncManager::new(project);

        // Sync from code (assuming crucible-core/src for now)
        let source_dir = PathBuf::from("crucible-core/src");
        let (report, discovered) = sync_manager.sync_from_code(&source_dir)?;

        if !interactive {
            // Non-interactive mode - just show the report
            println!();
            println!("{}  Analysis Results", "Sync".green().bold());
            println!("  Modules discovered: {}", report.modules_discovered);
            println!();

            if !report.new_modules.is_empty() {
                println!("{}  New Modules Found:", "⚠".yellow());
                for module in &report.new_modules {
                    println!("  - {}", module.cyan());
                }
                println!();
            }

            if !report.new_exports.is_empty() {
                println!("{}  New Exports Found:", "⚠".yellow());
                for (module, exports) in &report.new_exports {
                    println!("  {} ({} new):", module.cyan(), exports.len());
                    for export in exports {
                        println!("    - {export}");
                    }
                }
                println!();
            }

            if !report.new_dependencies.is_empty() {
                println!("{}  New Dependencies Found:", "⚠".yellow());
                for (module, deps) in &report.new_dependencies {
                    println!("  {} depends on:", module.cyan());
                    for dep in deps {
                        println!("    - {dep}");
                    }
                }
                println!();
            }

            if report.new_modules.is_empty()
                && report.new_exports.is_empty()
                && report.new_dependencies.is_empty()
            {
                println!("{}", "✓ Architecture is in sync with code!".green().bold());
            } else {
                println!("{}", "⚠ Architecture needs updates".yellow().bold());
                println!();
                println!("Next steps:");
                println!("  1. Review the changes above");
                println!(
                    "  2. Run {} to auto-update",
                    "crucible claude sync --interactive".cyan()
                );
                println!("  3. Or manually update module definitions in .crucible/modules/");
                println!("  4. Run {} to verify", "crucible validate".cyan());
            }
        } else {
            // Interactive mode - prompt to apply changes
            let updates = sync_manager.apply_sync_updates(&report, &discovered, true)?;

            if updates == 0 {
                println!("\n{}", "✓ No updates needed!".green().bold());
            }
        }
    }

    Ok(())
}

fn claude_validate(_module: Option<&str>) -> Result<()> {
    println!(
        "{}  architecture with Claude output...",
        "Validating".cyan().bold()
    );

    let parser = CrucibleParser::new(PathBuf::from(".crucible"));
    let project = parser.parse_project()?;

    let validator = Validator::new(project);
    let result = validator.validate();

    // Format for Claude with enhanced suggestions - re-parse project
    let parser2 = CrucibleParser::new(PathBuf::from(".crucible"));
    let project2 = parser2.parse_project()?;
    let hooks = ValidationHooks::new(project2);
    let formatted = hooks.format_with_context(&result);

    println!();
    println!("{formatted}");

    Ok(())
}

fn claude_context(_format: &str) -> Result<()> {
    let parser = CrucibleParser::new(PathBuf::from(".crucible"));
    let project = parser.parse_project()?;

    let mut config = IntegrationConfig::load_with_overrides(None)?;
    config.mode = IntegrationMode::Enhanced;
    let context_gen = ContextGenerator::new(project, config);

    let context = context_gen.generate_context_json()?;
    println!("{context}");

    Ok(())
}
