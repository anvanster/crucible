use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use crucible_core::claude::{
    ContextGenerator, IntegrationConfig, IntegrationMode, SyncManager, ValidationHooks,
    ValidationLevel,
};
use crucible_core::{Generator, Parser as CrucibleParser, Validator};
use std::path::PathBuf;

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
        /// Project name
        #[arg(long)]
        name: String,

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
            language,
            pattern,
        } => {
            init_project(&name, &language, &pattern)?;
        }
        Commands::Validate { path, strict } => {
            validate_project(&path, strict)?;
        }
        Commands::Generate { path, lang, output } => {
            generate_code(&path, &lang, &output)?;
        }
        Commands::Graph { format } => {
            println!("Graph generation not yet implemented");
            println!("Format: {}", format);
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

fn init_project(name: &str, language: &str, pattern: &str) -> Result<()> {
    println!(
        "{}  Crucible project: {}",
        "Initializing".green().bold(),
        name
    );

    // Create project directory
    std::fs::create_dir_all(name)?;

    // Create .crucible directory inside project
    let project_path = std::path::Path::new(name);
    std::fs::create_dir_all(project_path.join(".crucible/modules"))?;
    std::fs::create_dir_all(project_path.join(".crucible/types"))?;

    // Create manifest.json
    let manifest = format!(
        r#"{{
  "version": "0.1.0",
  "project": {{
    "name": "{}",
    "language": "{}",
    "architecture_pattern": "{}"
  }},
  "modules": [],
  "strict_validation": true
}}"#,
        name, language, pattern
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

    println!("{} Created {}/", "✓".green(), name);
    println!("{} Created {}/.crucible/manifest.json", "✓".green(), name);
    println!("{} Created {}/.crucible/rules.json", "✓".green(), name);
    println!("{} Created {}/.crucible/modules/", "✓".green(), name);
    println!("{} Created {}/.crucible/types/", "✓".green(), name);
    println!();
    println!("{}", "Project initialized successfully!".green().bold());
    println!();
    println!("Next steps:");
    println!("  1. {}", format!("cd {}", name).cyan());
    println!("  2. Create module definitions in .crucible/modules/");
    println!("  3. Run {} to validate", "crucible validate".cyan());

    Ok(())
}

fn validate_project(path: &PathBuf, strict: bool) -> Result<()> {
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

fn generate_code(path: &PathBuf, lang: &str, output: &PathBuf) -> Result<()> {
    let parser = CrucibleParser::new(path);
    let project = parser.parse_project()?;

    match lang {
        "typescript" | "ts" => {
            let gen = Generator::new(project);
            gen.generate_typescript(output)?;
            println!("✓ Generated TypeScript interfaces in {}", output.display());
        }
        _ => {
            println!("Language '{}' not yet supported", lang);
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
        let parser = CrucibleParser::new(&PathBuf::from(".crucible"));
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
                        println!("    - {}", export);
                    }
                }
                println!();
            }

            if !report.new_dependencies.is_empty() {
                println!("{}  New Dependencies Found:", "⚠".yellow());
                for (module, deps) in &report.new_dependencies {
                    println!("  {} depends on:", module.cyan());
                    for dep in deps {
                        println!("    - {}", dep);
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

    let parser = CrucibleParser::new(&PathBuf::from(".crucible"));
    let project = parser.parse_project()?;

    let validator = Validator::new(project);
    let result = validator.validate();

    // Format for Claude with enhanced suggestions - re-parse project
    let parser2 = CrucibleParser::new(&PathBuf::from(".crucible"));
    let project2 = parser2.parse_project()?;
    let hooks = ValidationHooks::new(project2);
    let formatted = hooks.format_with_context(&result);

    println!();
    println!("{}", formatted);

    Ok(())
}

fn claude_context(_format: &str) -> Result<()> {
    let parser = CrucibleParser::new(&PathBuf::from(".crucible"));
    let project = parser.parse_project()?;

    let mut config = IntegrationConfig::load_with_overrides(None)?;
    config.mode = IntegrationMode::Enhanced;
    let context_gen = ContextGenerator::new(project, config);

    let context = context_gen.generate_context_json()?;
    println!("{}", context);

    Ok(())
}
