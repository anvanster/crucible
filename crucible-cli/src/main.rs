use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
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
    }

    Ok(())
}

fn init_project(name: &str, language: &str, pattern: &str) -> Result<()> {
    println!(
        "{}  Crucible project: {}",
        "Initializing".green().bold(),
        name
    );

    // Create .crucible directory
    std::fs::create_dir_all(".crucible/modules")?;
    std::fs::create_dir_all(".crucible/types")?;

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

    std::fs::write(".crucible/manifest.json", manifest)?;

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

    std::fs::write(".crucible/rules.json", rules)?;

    println!("{} Created .crucible/manifest.json", "✓".green());
    println!("{} Created .crucible/rules.json", "✓".green());
    println!("{} Created .crucible/modules/", "✓".green());
    println!("{} Created .crucible/types/", "✓".green());
    println!();
    println!("{}", "Project initialized successfully!".green().bold());
    println!();
    println!("Next steps:");
    println!("  1. Create module definitions in .crucible/modules/");
    println!("  2. Run {} to validate", "crucible validate".cyan());

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
