//! Crucible Compliance CLI - Validate architecture against compliance frameworks

use anyhow::{Context, Result};
use clap::{Parser as ClapParser, ValueEnum};
use crucible_compliance::{
    ComplianceValidator, FrameworkLoader, OutputFormat, ReportConfig, Reporter,
};
use crucible_core::Parser as CrucibleParser;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(ClapParser)]
#[command(name = "crucible-comply")]
#[command(
    version,
    about = "Validate Crucible architecture against compliance frameworks"
)]
struct Cli {
    /// Path to the project root (defaults to current directory)
    #[arg(short, long, default_value = ".")]
    project: PathBuf,

    /// Compliance frameworks to validate against
    #[arg(short, long, value_delimiter = ',')]
    frameworks: Option<Vec<String>>,

    /// Path to custom framework files
    #[arg(long)]
    framework_path: Option<PathBuf>,

    /// Output format
    #[arg(short, long, default_value = "text", value_enum)]
    output: OutputFormatArg,

    /// Output file (defaults to stdout)
    #[arg(short = 'O', long)]
    output_file: Option<PathBuf>,

    /// Include fix suggestions in output
    #[arg(long, default_value = "true")]
    suggestions: bool,

    /// Include code examples in output
    #[arg(long)]
    examples: bool,

    /// Disable colored output
    #[arg(long)]
    no_color: bool,

    /// Exit with error on any violation (including warnings)
    #[arg(long)]
    strict: bool,

    /// List available frameworks and exit
    #[arg(long)]
    list_frameworks: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Clone, ValueEnum)]
enum OutputFormatArg {
    Text,
    Json,
    Sarif,
    Markdown,
    Html,
}

impl From<OutputFormatArg> for OutputFormat {
    fn from(arg: OutputFormatArg) -> Self {
        match arg {
            OutputFormatArg::Text => OutputFormat::Text,
            OutputFormatArg::Json => OutputFormat::Json,
            OutputFormatArg::Sarif => OutputFormat::Sarif,
            OutputFormatArg::Markdown => OutputFormat::Markdown,
            OutputFormatArg::Html => OutputFormat::Html,
        }
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse_from(std::env::args());

    match run(cli) {
        Ok(passed) => {
            if passed {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(1)
            }
        }
        Err(e) => {
            eprintln!("Error: {e:#}");
            ExitCode::from(2)
        }
    }
}

fn run(cli: Cli) -> Result<bool> {
    // Load frameworks
    let mut loader = FrameworkLoader::new();

    // Try to load from project's frameworks directory
    let project_frameworks = cli.project.join("frameworks");
    if project_frameworks.exists() {
        if cli.verbose {
            eprintln!("Loading frameworks from: {}", project_frameworks.display());
        }
        loader
            .load_directory(&project_frameworks)
            .context("Failed to load project frameworks")?;
    }

    // Load from custom framework path if specified
    if let Some(path) = &cli.framework_path {
        if cli.verbose {
            eprintln!("Loading frameworks from: {}", path.display());
        }
        if path.is_dir() {
            loader
                .load_directory(path)
                .context("Failed to load custom frameworks")?;
        } else {
            loader
                .load_file(path)
                .context("Failed to load framework file")?;
        }
    }

    // Load bundled frameworks from the crate's framework directory
    let crate_frameworks = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("frameworks");
    if crate_frameworks.exists() && loader.count() == 0 {
        if cli.verbose {
            eprintln!(
                "Loading bundled frameworks from: {}",
                crate_frameworks.display()
            );
        }
        loader
            .load_directory(&crate_frameworks)
            .context("Failed to load bundled frameworks")?;
    }

    // List frameworks mode
    if cli.list_frameworks {
        println!("Available compliance frameworks:");
        for name in loader.names() {
            if let Some(framework) = loader.get(name) {
                println!(
                    "  {} v{} - {} rules",
                    name,
                    framework.version(),
                    framework.rule_count()
                );
            }
        }
        return Ok(true);
    }

    // Ensure we have frameworks to validate against
    if loader.count() == 0 {
        anyhow::bail!("No compliance frameworks found. Use --framework-path to specify a framework file or directory.");
    }

    // Determine which frameworks to validate
    let frameworks_to_use: Vec<String> = if let Some(ref names) = cli.frameworks {
        // Validate requested frameworks exist
        for name in names {
            if !loader.has(name) {
                anyhow::bail!(
                    "Framework '{}' not found. Available: {}",
                    name,
                    loader.names().join(", ")
                );
            }
        }
        names.clone()
    } else {
        // Use all loaded frameworks
        loader.names().into_iter().map(|s| s.to_string()).collect()
    };

    if cli.verbose {
        eprintln!(
            "Validating against {} framework(s): {}",
            frameworks_to_use.len(),
            frameworks_to_use.join(", ")
        );
    }

    // Load the project
    let crucible_dir = cli.project.join(".crucible");
    let parser = CrucibleParser::new(&crucible_dir);
    let project = parser
        .parse_project()
        .context("Failed to load Crucible project")?;

    if cli.verbose {
        eprintln!("Loaded project with {} module(s)", project.modules.len());
    }

    // Setup reporter
    let config = ReportConfig {
        format: cli.output.into(),
        include_suggestions: cli.suggestions,
        include_examples: cli.examples,
        color: !cli.no_color && cli.output_file.is_none(),
    };
    let reporter = Reporter::new(config);

    // Validate against each framework
    let mut all_passed = true;

    for framework_name in &frameworks_to_use {
        let framework = loader
            .get(framework_name)
            .expect("Framework should exist after validation");

        let validator = ComplianceValidator::new(framework);
        let report = validator
            .validate(&project)
            .context(format!("Failed to validate against {framework_name}"))?;

        let passed = if cli.strict {
            report.violations.is_empty()
        } else {
            report.passed()
        };

        if !passed {
            all_passed = false;
        }

        // Output the report
        let output = reporter.format(&report);

        if let Some(ref path) = cli.output_file {
            let mut file = std::fs::File::create(path)
                .context(format!("Failed to create output file: {}", path.display()))?;
            std::io::Write::write_all(&mut file, output.as_bytes())
                .context("Failed to write report")?;
            if cli.verbose {
                eprintln!("Report written to: {}", path.display());
            }
        } else {
            print!("{output}");
        }
    }

    Ok(all_passed)
}
