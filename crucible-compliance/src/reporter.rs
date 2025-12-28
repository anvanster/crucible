//! Report formatting and output for compliance validation results

use crate::validator::{ValidationReport, Violation};
use crucible_core::types::Severity;
use serde::Serialize;
use std::io::{self, Write};

/// Available output formats for reports
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutputFormat {
    /// Human-readable text format
    #[default]
    Text,
    /// JSON format for programmatic consumption
    Json,
    /// SARIF format for IDE integration
    Sarif,
    /// Markdown format for documentation
    Markdown,
    /// HTML format for audit-ready reports
    Html,
}

/// Configuration for report generation
#[derive(Debug, Clone)]
pub struct ReportConfig {
    /// Output format
    pub format: OutputFormat,
    /// Include fix suggestions in output
    pub include_suggestions: bool,
    /// Include code examples in output
    pub include_examples: bool,
    /// Use colored output for terminal
    pub color: bool,
}

impl Default for ReportConfig {
    fn default() -> Self {
        Self {
            format: OutputFormat::Text,
            include_suggestions: true,
            include_examples: false,
            color: true,
        }
    }
}

impl ReportConfig {
    /// Create a new configuration with the specified format
    pub fn new(format: OutputFormat) -> Self {
        Self {
            format,
            ..Default::default()
        }
    }

    /// Create a JSON configuration
    pub fn json() -> Self {
        Self::new(OutputFormat::Json)
    }

    /// Create a SARIF configuration
    pub fn sarif() -> Self {
        Self::new(OutputFormat::Sarif)
    }

    /// Create a Markdown configuration
    pub fn markdown() -> Self {
        Self::new(OutputFormat::Markdown)
    }

    /// Create an HTML configuration
    pub fn html() -> Self {
        Self::new(OutputFormat::Html)
    }

    /// Set whether to include suggestions
    pub fn with_suggestions(mut self, include: bool) -> Self {
        self.include_suggestions = include;
        self
    }

    /// Set whether to include examples
    pub fn with_examples(mut self, include: bool) -> Self {
        self.include_examples = include;
        self
    }

    /// Set whether to use colored output
    pub fn with_color(mut self, color: bool) -> Self {
        self.color = color;
        self
    }
}

/// Formats and outputs validation reports
#[derive(Debug)]
pub struct Reporter {
    config: ReportConfig,
}

impl Reporter {
    /// Create a new reporter with the given configuration
    pub fn new(config: ReportConfig) -> Self {
        Self { config }
    }

    /// Create a reporter with default text format
    pub fn text() -> Self {
        Self::new(ReportConfig::default())
    }

    /// Create a reporter with JSON format
    pub fn json() -> Self {
        Self::new(ReportConfig::json())
    }

    /// Create a reporter with SARIF format
    pub fn sarif() -> Self {
        Self::new(ReportConfig::sarif())
    }

    /// Create a reporter with Markdown format
    pub fn markdown() -> Self {
        Self::new(ReportConfig::markdown())
    }

    /// Create a reporter with HTML format
    pub fn html() -> Self {
        Self::new(ReportConfig::html())
    }

    /// Format report as string
    pub fn format(&self, report: &ValidationReport) -> String {
        match self.config.format {
            OutputFormat::Text => self.format_text(report),
            OutputFormat::Json => self.format_json(report),
            OutputFormat::Sarif => self.format_sarif(report),
            OutputFormat::Markdown => self.format_markdown(report),
            OutputFormat::Html => self.format_html(report),
        }
    }

    /// Write report to output
    pub fn write<W: Write>(&self, report: &ValidationReport, writer: &mut W) -> io::Result<()> {
        let formatted = self.format(report);
        writer.write_all(formatted.as_bytes())
    }

    /// Print report to stdout
    pub fn print(&self, report: &ValidationReport) {
        print!("{}", self.format(report));
    }

    /// Format as human-readable text
    fn format_text(&self, report: &ValidationReport) -> String {
        let mut output = String::new();

        // Header
        output.push_str(&self.text_header(report));
        output.push('\n');

        // Summary
        output.push_str(&self.text_summary(report));
        output.push('\n');

        // Violations
        if !report.violations.is_empty() {
            output.push('\n');
            output.push_str(&self.text_violations(report));
        }

        // Footer
        output.push_str(&self.text_footer(report));

        output
    }

    fn text_header(&self, report: &ValidationReport) -> String {
        let header = format!(
            "Compliance Validation Report\n{}\nFramework: {} v{}\n",
            "=".repeat(50),
            report.framework_name,
            report.framework_version
        );

        if self.config.color {
            format!("\x1b[1;34m{header}\x1b[0m")
        } else {
            header
        }
    }

    fn text_summary(&self, report: &ValidationReport) -> String {
        let status = if report.passed() {
            if self.config.color {
                "\x1b[1;32mPASSED\x1b[0m".to_string()
            } else {
                "PASSED".to_string()
            }
        } else if self.config.color {
            "\x1b[1;31mFAILED\x1b[0m".to_string()
        } else {
            "FAILED".to_string()
        };

        format!(
            "Status: {}\nModules validated: {}\nRules checked: {}\nErrors: {}\nWarnings: {}\n",
            status,
            report.modules_validated,
            report.rules_checked,
            report.error_count(),
            report.warning_count()
        )
    }

    fn text_violations(&self, report: &ValidationReport) -> String {
        let mut output = String::new();
        output.push_str("Violations:\n");
        output.push_str(&"-".repeat(50));
        output.push('\n');

        for (i, violation) in report.violations.iter().enumerate() {
            output.push_str(&self.format_text_violation(i + 1, violation));
            output.push('\n');
        }

        output
    }

    fn format_text_violation(&self, index: usize, violation: &Violation) -> String {
        let severity_str = match violation.severity {
            Severity::Error => {
                if self.config.color {
                    "\x1b[1;31mERROR\x1b[0m"
                } else {
                    "ERROR"
                }
            }
            Severity::Warning => {
                if self.config.color {
                    "\x1b[1;33mWARNING\x1b[0m"
                } else {
                    "WARNING"
                }
            }
            Severity::Info => {
                if self.config.color {
                    "\x1b[1;36mINFO\x1b[0m"
                } else {
                    "INFO"
                }
            }
        };

        let mut output = format!(
            "{}. [{}] {} ({})\n   Location: {}\n   Issue: {}\n",
            index,
            severity_str,
            violation.rule_id,
            violation.description,
            violation.location,
            violation.issue
        );

        if self.config.include_suggestions {
            if let Some(suggestion) = &violation.suggestion {
                output.push_str(&format!("   Suggestion: {suggestion}\n"));
            }
        }

        output
    }

    fn text_footer(&self, _report: &ValidationReport) -> String {
        let footer = format!("\n{}\n", "=".repeat(50));
        if self.config.color {
            format!("\x1b[1;34m{footer}\x1b[0m")
        } else {
            footer
        }
    }

    /// Format as JSON
    fn format_json(&self, report: &ValidationReport) -> String {
        let json_report = JsonReport::from_report(report, &self.config);
        serde_json::to_string_pretty(&json_report).unwrap_or_else(|_| "{}".to_string())
    }

    /// Format as SARIF (Static Analysis Results Interchange Format)
    fn format_sarif(&self, report: &ValidationReport) -> String {
        let sarif = SarifReport::from_report(report, &self.config);
        serde_json::to_string_pretty(&sarif).unwrap_or_else(|_| "{}".to_string())
    }

    /// Format as Markdown
    fn format_markdown(&self, report: &ValidationReport) -> String {
        let mut output = String::new();

        // Header
        output.push_str(&format!(
            "# Compliance Validation Report\n\n**Framework:** {} v{}\n\n",
            report.framework_name, report.framework_version
        ));

        // Status badge
        let status = if report.passed() {
            "![Status](https://img.shields.io/badge/status-passed-green)"
        } else {
            "![Status](https://img.shields.io/badge/status-failed-red)"
        };
        output.push_str(&format!("{status}\n\n"));

        // Summary table
        output.push_str("## Summary\n\n");
        output.push_str("| Metric | Value |\n");
        output.push_str("|--------|-------|\n");
        output.push_str(&format!(
            "| Modules Validated | {} |\n",
            report.modules_validated
        ));
        output.push_str(&format!("| Rules Checked | {} |\n", report.rules_checked));
        output.push_str(&format!("| Errors | {} |\n", report.error_count()));
        output.push_str(&format!("| Warnings | {} |\n\n", report.warning_count()));

        // Violations
        if !report.violations.is_empty() {
            output.push_str("## Violations\n\n");

            // Group by severity
            let errors: Vec<_> = report
                .violations
                .iter()
                .filter(|v| v.severity == Severity::Error)
                .collect();
            let warnings: Vec<_> = report
                .violations
                .iter()
                .filter(|v| v.severity == Severity::Warning)
                .collect();
            let infos: Vec<_> = report
                .violations
                .iter()
                .filter(|v| v.severity == Severity::Info)
                .collect();

            if !errors.is_empty() {
                output.push_str("### Errors\n\n");
                for violation in errors {
                    output.push_str(&self.format_markdown_violation(violation));
                }
            }

            if !warnings.is_empty() {
                output.push_str("### Warnings\n\n");
                for violation in warnings {
                    output.push_str(&self.format_markdown_violation(violation));
                }
            }

            if !infos.is_empty() {
                output.push_str("### Info\n\n");
                for violation in infos {
                    output.push_str(&self.format_markdown_violation(violation));
                }
            }
        } else {
            output.push_str("## Results\n\nNo violations found. The project is compliant with all checked rules.\n");
        }

        output
    }

    fn format_markdown_violation(&self, violation: &Violation) -> String {
        let mut output = format!(
            "#### `{}`\n\n- **Location:** `{}`\n- **Description:** {}\n- **Issue:** {}\n",
            violation.rule_id, violation.location, violation.description, violation.issue
        );

        if self.config.include_suggestions {
            if let Some(suggestion) = &violation.suggestion {
                output.push_str(&format!("- **Suggestion:** {suggestion}\n"));
            }
        }

        output.push('\n');
        output
    }

    /// Format as HTML for audit-ready reports
    fn format_html(&self, report: &ValidationReport) -> String {
        let mut output = String::new();

        // HTML header with embedded CSS
        output.push_str(&self.html_header(report));

        // Report content
        output.push_str("<div class=\"container\">\n");

        // Header section
        output.push_str(&self.html_title(report));

        // Summary section
        output.push_str(&self.html_summary(report));

        // Violations section
        if !report.violations.is_empty() {
            output.push_str(&self.html_violations(report));
        } else {
            output.push_str(&self.html_no_violations());
        }

        // Footer
        output.push_str(&self.html_footer());

        output.push_str("</div>\n</body>\n</html>");

        output
    }

    fn html_header(&self, report: &ValidationReport) -> String {
        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Compliance Report - {} v{}</title>
    <style>
        :root {{
            --color-pass: #22c55e;
            --color-fail: #ef4444;
            --color-warning: #f59e0b;
            --color-info: #3b82f6;
            --color-bg: #f8fafc;
            --color-card: #ffffff;
            --color-border: #e2e8f0;
            --color-text: #1e293b;
            --color-muted: #64748b;
        }}
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
            background-color: var(--color-bg);
            color: var(--color-text);
            line-height: 1.6;
        }}
        .container {{
            max-width: 1200px;
            margin: 0 auto;
            padding: 2rem;
        }}
        .header {{
            text-align: center;
            margin-bottom: 2rem;
            padding-bottom: 2rem;
            border-bottom: 2px solid var(--color-border);
        }}
        .header h1 {{
            font-size: 2rem;
            margin-bottom: 0.5rem;
        }}
        .header .framework {{
            color: var(--color-muted);
            font-size: 1.1rem;
        }}
        .status-badge {{
            display: inline-block;
            padding: 0.5rem 1.5rem;
            border-radius: 9999px;
            font-weight: 600;
            font-size: 1rem;
            margin-top: 1rem;
        }}
        .status-passed {{
            background-color: #dcfce7;
            color: #166534;
        }}
        .status-failed {{
            background-color: #fee2e2;
            color: #991b1b;
        }}
        .summary {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 1rem;
            margin-bottom: 2rem;
        }}
        .summary-card {{
            background: var(--color-card);
            border-radius: 0.5rem;
            padding: 1.5rem;
            box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
            text-align: center;
        }}
        .summary-card .value {{
            font-size: 2rem;
            font-weight: 700;
        }}
        .summary-card .label {{
            color: var(--color-muted);
            font-size: 0.875rem;
            text-transform: uppercase;
            letter-spacing: 0.05em;
        }}
        .summary-card.errors .value {{
            color: var(--color-fail);
        }}
        .summary-card.warnings .value {{
            color: var(--color-warning);
        }}
        .violations-section {{
            margin-top: 2rem;
        }}
        .violations-section h2 {{
            font-size: 1.5rem;
            margin-bottom: 1rem;
            padding-bottom: 0.5rem;
            border-bottom: 2px solid var(--color-border);
        }}
        .violation {{
            background: var(--color-card);
            border-radius: 0.5rem;
            padding: 1.5rem;
            margin-bottom: 1rem;
            box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
            border-left: 4px solid var(--color-border);
        }}
        .violation.severity-error {{
            border-left-color: var(--color-fail);
        }}
        .violation.severity-warning {{
            border-left-color: var(--color-warning);
        }}
        .violation.severity-info {{
            border-left-color: var(--color-info);
        }}
        .violation-header {{
            display: flex;
            justify-content: space-between;
            align-items: flex-start;
            margin-bottom: 1rem;
        }}
        .violation-title {{
            font-weight: 600;
            font-size: 1.1rem;
        }}
        .severity-tag {{
            display: inline-block;
            padding: 0.25rem 0.75rem;
            border-radius: 9999px;
            font-size: 0.75rem;
            font-weight: 600;
            text-transform: uppercase;
        }}
        .severity-tag.error {{
            background-color: #fee2e2;
            color: #991b1b;
        }}
        .severity-tag.warning {{
            background-color: #fef3c7;
            color: #92400e;
        }}
        .severity-tag.info {{
            background-color: #dbeafe;
            color: #1e40af;
        }}
        .violation-detail {{
            margin-bottom: 0.75rem;
        }}
        .violation-detail .label {{
            font-weight: 600;
            color: var(--color-muted);
            font-size: 0.875rem;
        }}
        .violation-detail .value {{
            margin-top: 0.25rem;
        }}
        .violation-detail code {{
            background-color: var(--color-bg);
            padding: 0.125rem 0.375rem;
            border-radius: 0.25rem;
            font-family: 'SF Mono', Monaco, 'Cascadia Code', monospace;
            font-size: 0.875rem;
        }}
        .suggestion {{
            background-color: #f0fdf4;
            border: 1px solid #bbf7d0;
            border-radius: 0.375rem;
            padding: 0.75rem 1rem;
            margin-top: 1rem;
        }}
        .suggestion .label {{
            color: #166534;
            font-weight: 600;
            font-size: 0.875rem;
        }}
        .no-violations {{
            background: #f0fdf4;
            border: 2px solid #86efac;
            border-radius: 0.5rem;
            padding: 2rem;
            text-align: center;
            color: #166534;
        }}
        .no-violations h3 {{
            font-size: 1.25rem;
            margin-bottom: 0.5rem;
        }}
        .footer {{
            margin-top: 3rem;
            padding-top: 2rem;
            border-top: 2px solid var(--color-border);
            text-align: center;
            color: var(--color-muted);
            font-size: 0.875rem;
        }}
        @media print {{
            body {{
                background: white;
            }}
            .container {{
                max-width: none;
            }}
            .violation {{
                break-inside: avoid;
            }}
        }}
    </style>
</head>
<body>
"#,
            report.framework_name, report.framework_version
        )
    }

    fn html_title(&self, report: &ValidationReport) -> String {
        let status_class = if report.passed() {
            "status-passed"
        } else {
            "status-failed"
        };
        let status_text = if report.passed() { "PASSED" } else { "FAILED" };

        format!(
            r#"<div class="header">
    <h1>Compliance Validation Report</h1>
    <p class="framework">{} v{}</p>
    <span class="status-badge {}">{}</span>
</div>
"#,
            report.framework_name, report.framework_version, status_class, status_text
        )
    }

    fn html_summary(&self, report: &ValidationReport) -> String {
        let error_class = if report.error_count() > 0 {
            "summary-card errors"
        } else {
            "summary-card"
        };
        let warning_class = if report.warning_count() > 0 {
            "summary-card warnings"
        } else {
            "summary-card"
        };

        format!(
            r#"<div class="summary">
    <div class="summary-card">
        <div class="value">{}</div>
        <div class="label">Modules Validated</div>
    </div>
    <div class="summary-card">
        <div class="value">{}</div>
        <div class="label">Rules Checked</div>
    </div>
    <div class="{}">
        <div class="value">{}</div>
        <div class="label">Errors</div>
    </div>
    <div class="{}">
        <div class="value">{}</div>
        <div class="label">Warnings</div>
    </div>
</div>
"#,
            report.modules_validated,
            report.rules_checked,
            error_class,
            report.error_count(),
            warning_class,
            report.warning_count()
        )
    }

    fn html_violations(&self, report: &ValidationReport) -> String {
        let mut output = String::new();

        // Group by severity
        let errors: Vec<_> = report
            .violations
            .iter()
            .filter(|v| v.severity == Severity::Error)
            .collect();
        let warnings: Vec<_> = report
            .violations
            .iter()
            .filter(|v| v.severity == Severity::Warning)
            .collect();
        let infos: Vec<_> = report
            .violations
            .iter()
            .filter(|v| v.severity == Severity::Info)
            .collect();

        if !errors.is_empty() {
            output.push_str("<div class=\"violations-section\">\n");
            output.push_str("<h2>Errors</h2>\n");
            for violation in &errors {
                output.push_str(&self.format_html_violation(violation));
            }
            output.push_str("</div>\n");
        }

        if !warnings.is_empty() {
            output.push_str("<div class=\"violations-section\">\n");
            output.push_str("<h2>Warnings</h2>\n");
            for violation in &warnings {
                output.push_str(&self.format_html_violation(violation));
            }
            output.push_str("</div>\n");
        }

        if !infos.is_empty() {
            output.push_str("<div class=\"violations-section\">\n");
            output.push_str("<h2>Information</h2>\n");
            for violation in &infos {
                output.push_str(&self.format_html_violation(violation));
            }
            output.push_str("</div>\n");
        }

        output
    }

    fn format_html_violation(&self, violation: &Violation) -> String {
        let severity_class = match violation.severity {
            Severity::Error => "severity-error",
            Severity::Warning => "severity-warning",
            Severity::Info => "severity-info",
        };
        let severity_tag_class = match violation.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Info => "info",
        };
        let severity_text = match violation.severity {
            Severity::Error => "Error",
            Severity::Warning => "Warning",
            Severity::Info => "Info",
        };

        let mut output = format!(
            r#"<div class="violation {}">
    <div class="violation-header">
        <span class="violation-title">{}</span>
        <span class="severity-tag {}">{}</span>
    </div>
    <div class="violation-detail">
        <div class="label">Location</div>
        <div class="value"><code>{}</code></div>
    </div>
    <div class="violation-detail">
        <div class="label">Description</div>
        <div class="value">{}</div>
    </div>
    <div class="violation-detail">
        <div class="label">Issue</div>
        <div class="value">{}</div>
    </div>
"#,
            severity_class,
            html_escape(&violation.rule_id),
            severity_tag_class,
            severity_text,
            html_escape(&violation.location),
            html_escape(&violation.description),
            html_escape(&violation.issue)
        );

        if self.config.include_suggestions {
            if let Some(suggestion) = &violation.suggestion {
                output.push_str(&format!(
                    r#"    <div class="suggestion">
        <div class="label">💡 Suggestion</div>
        <div class="value">{}</div>
    </div>
"#,
                    html_escape(suggestion)
                ));
            }
        }

        output.push_str("</div>\n");
        output
    }

    fn html_no_violations(&self) -> String {
        r#"<div class="no-violations">
    <h3>✓ All Checks Passed</h3>
    <p>No compliance violations were found. The project meets all requirements.</p>
</div>
"#
        .to_string()
    }

    fn html_footer(&self) -> String {
        let now = chrono::Utc::now();
        format!(
            r#"<div class="footer">
    <p>Generated by Crucible Compliance Validator</p>
    <p>Report generated on {}</p>
</div>
"#,
            now.format("%Y-%m-%d %H:%M:%S UTC")
        )
    }
}

/// JSON report structure
#[derive(Debug, Serialize)]
struct JsonReport {
    framework: String,
    version: String,
    passed: bool,
    summary: JsonSummary,
    violations: Vec<JsonViolation>,
}

#[derive(Debug, Serialize)]
struct JsonSummary {
    modules_validated: usize,
    rules_checked: usize,
    error_count: usize,
    warning_count: usize,
    total_violations: usize,
}

#[derive(Debug, Serialize)]
struct JsonViolation {
    rule_id: String,
    severity: String,
    description: String,
    location: String,
    issue: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    suggestion: Option<String>,
}

impl JsonReport {
    fn from_report(report: &ValidationReport, config: &ReportConfig) -> Self {
        Self {
            framework: report.framework_name.clone(),
            version: report.framework_version.clone(),
            passed: report.passed(),
            summary: JsonSummary {
                modules_validated: report.modules_validated,
                rules_checked: report.rules_checked,
                error_count: report.error_count(),
                warning_count: report.warning_count(),
                total_violations: report.violation_count(),
            },
            violations: report
                .violations
                .iter()
                .map(|v| JsonViolation {
                    rule_id: v.rule_id.clone(),
                    severity: format!("{:?}", v.severity).to_lowercase(),
                    description: v.description.clone(),
                    location: v.location.clone(),
                    issue: v.issue.clone(),
                    suggestion: if config.include_suggestions {
                        v.suggestion.clone()
                    } else {
                        None
                    },
                })
                .collect(),
        }
    }
}

/// SARIF report structure (Static Analysis Results Interchange Format)
#[derive(Debug, Serialize)]
struct SarifReport {
    #[serde(rename = "$schema")]
    schema: String,
    version: String,
    runs: Vec<SarifRun>,
}

#[derive(Debug, Serialize)]
struct SarifRun {
    tool: SarifTool,
    results: Vec<SarifResult>,
}

#[derive(Debug, Serialize)]
struct SarifTool {
    driver: SarifDriver,
}

#[derive(Debug, Serialize)]
struct SarifDriver {
    name: String,
    version: String,
    #[serde(rename = "informationUri")]
    information_uri: String,
    rules: Vec<SarifRule>,
}

#[derive(Debug, Serialize)]
struct SarifRule {
    id: String,
    #[serde(rename = "shortDescription")]
    short_description: SarifMessage,
    #[serde(rename = "defaultConfiguration")]
    default_configuration: SarifConfiguration,
}

#[derive(Debug, Serialize)]
struct SarifConfiguration {
    level: String,
}

#[derive(Debug, Serialize)]
struct SarifResult {
    #[serde(rename = "ruleId")]
    rule_id: String,
    level: String,
    message: SarifMessage,
    locations: Vec<SarifLocation>,
}

#[derive(Debug, Serialize)]
struct SarifMessage {
    text: String,
}

#[derive(Debug, Serialize)]
struct SarifLocation {
    #[serde(rename = "physicalLocation")]
    physical_location: SarifPhysicalLocation,
}

#[derive(Debug, Serialize)]
struct SarifPhysicalLocation {
    #[serde(rename = "artifactLocation")]
    artifact_location: SarifArtifactLocation,
}

#[derive(Debug, Serialize)]
struct SarifArtifactLocation {
    uri: String,
}

impl SarifReport {
    fn from_report(report: &ValidationReport, _config: &ReportConfig) -> Self {
        // Collect unique rules
        let mut seen_rules = std::collections::HashSet::new();
        let rules: Vec<SarifRule> = report
            .violations
            .iter()
            .filter(|v| seen_rules.insert(v.rule_id.clone()))
            .map(|v| SarifRule {
                id: v.rule_id.clone(),
                short_description: SarifMessage {
                    text: v.description.clone(),
                },
                default_configuration: SarifConfiguration {
                    level: severity_to_sarif_level(&v.severity),
                },
            })
            .collect();

        let results: Vec<SarifResult> = report
            .violations
            .iter()
            .map(|v| SarifResult {
                rule_id: v.rule_id.clone(),
                level: severity_to_sarif_level(&v.severity),
                message: SarifMessage {
                    text: v.issue.clone(),
                },
                locations: vec![SarifLocation {
                    physical_location: SarifPhysicalLocation {
                        artifact_location: SarifArtifactLocation {
                            uri: v.location.clone(),
                        },
                    },
                }],
            })
            .collect();

        Self {
            schema: "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json".to_string(),
            version: "2.1.0".to_string(),
            runs: vec![SarifRun {
                tool: SarifTool {
                    driver: SarifDriver {
                        name: format!("crucible-compliance-{}", report.framework_name),
                        version: report.framework_version.clone(),
                        information_uri: "https://github.com/crucible/compliance".to_string(),
                        rules,
                    },
                },
                results,
            }],
        }
    }
}

fn severity_to_sarif_level(severity: &Severity) -> String {
    match severity {
        Severity::Error => "error".to_string(),
        Severity::Warning => "warning".to_string(),
        Severity::Info => "note".to_string(),
    }
}

/// Escape special HTML characters
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validator::Violation;
    use crucible_core::types::Severity;

    fn create_test_report() -> ValidationReport {
        ValidationReport {
            framework_name: "HIPAA".to_string(),
            framework_version: "1.0.0".to_string(),
            violations: vec![
                Violation {
                    rule_id: "no-phi-in-logs".to_string(),
                    severity: Severity::Error,
                    description: "PHI must not be logged".to_string(),
                    location: "patient.PatientService.logPatient".to_string(),
                    issue: "Method with logging effect accesses data with @phi annotations"
                        .to_string(),
                    suggestion: Some("Use redacted logging or remove PHI from logs".to_string()),
                },
                Violation {
                    rule_id: "phi-requires-auth".to_string(),
                    severity: Severity::Warning,
                    description: "PHI access should require authentication".to_string(),
                    location: "patient.PatientService.getData".to_string(),
                    issue: "Accessing data with @phi annotations requires @requires-auth"
                        .to_string(),
                    suggestion: Some("Add @requires-auth to the method annotations".to_string()),
                },
            ],
            rules_checked: 8,
            modules_validated: 2,
        }
    }

    fn create_passing_report() -> ValidationReport {
        ValidationReport {
            framework_name: "HIPAA".to_string(),
            framework_version: "1.0.0".to_string(),
            violations: vec![],
            rules_checked: 8,
            modules_validated: 2,
        }
    }

    #[test]
    fn test_text_format_with_violations() {
        let report = create_test_report();
        let reporter = Reporter::new(ReportConfig::default().with_color(false));
        let output = reporter.format(&report);

        assert!(output.contains("Compliance Validation Report"));
        assert!(output.contains("Framework: HIPAA v1.0.0"));
        assert!(output.contains("FAILED"));
        assert!(output.contains("Errors: 1"));
        assert!(output.contains("Warnings: 1"));
        assert!(output.contains("no-phi-in-logs"));
        assert!(output.contains("phi-requires-auth"));
    }

    #[test]
    fn test_text_format_passing() {
        let report = create_passing_report();
        let reporter = Reporter::new(ReportConfig::default().with_color(false));
        let output = reporter.format(&report);

        assert!(output.contains("PASSED"));
        assert!(output.contains("Errors: 0"));
    }

    #[test]
    fn test_json_format() {
        let report = create_test_report();
        let reporter = Reporter::json();
        let output = reporter.format(&report);

        // Parse the output to verify it's valid JSON
        let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();

        assert_eq!(parsed["framework"], "HIPAA");
        assert_eq!(parsed["version"], "1.0.0");
        assert_eq!(parsed["passed"], false);
        assert_eq!(parsed["summary"]["error_count"], 1);
        assert_eq!(parsed["summary"]["warning_count"], 1);
        assert_eq!(parsed["violations"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn test_sarif_format() {
        let report = create_test_report();
        let reporter = Reporter::sarif();
        let output = reporter.format(&report);

        // Parse the output to verify it's valid SARIF JSON
        let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();

        assert_eq!(parsed["version"], "2.1.0");
        assert!(parsed["$schema"].as_str().unwrap().contains("sarif"));
        assert!(parsed["runs"].as_array().unwrap().len() > 0);

        let run = &parsed["runs"][0];
        assert!(run["tool"]["driver"]["name"]
            .as_str()
            .unwrap()
            .contains("HIPAA"));
        assert_eq!(run["results"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn test_markdown_format() {
        let report = create_test_report();
        let reporter = Reporter::markdown();
        let output = reporter.format(&report);

        assert!(output.contains("# Compliance Validation Report"));
        assert!(output.contains("**Framework:** HIPAA v1.0.0"));
        assert!(output.contains("status-failed"));
        assert!(output.contains("## Summary"));
        assert!(output.contains("| Errors | 1 |"));
        assert!(output.contains("### Errors"));
        assert!(output.contains("### Warnings"));
        assert!(output.contains("`no-phi-in-logs`"));
    }

    #[test]
    fn test_markdown_format_passing() {
        let report = create_passing_report();
        let reporter = Reporter::markdown();
        let output = reporter.format(&report);

        assert!(output.contains("status-passed"));
        assert!(output.contains("No violations found"));
    }

    #[test]
    fn test_config_without_suggestions() {
        let report = create_test_report();
        let reporter = Reporter::new(
            ReportConfig::default()
                .with_suggestions(false)
                .with_color(false),
        );
        let output = reporter.format(&report);

        assert!(!output.contains("Suggestion:"));
    }

    #[test]
    fn test_write_to_buffer() {
        let report = create_test_report();
        let reporter = Reporter::new(ReportConfig::default().with_color(false));
        let mut buffer = Vec::new();

        reporter.write(&report, &mut buffer).unwrap();
        let output = String::from_utf8(buffer).unwrap();

        assert!(output.contains("Compliance Validation Report"));
    }

    #[test]
    fn test_json_excludes_suggestion_when_disabled() {
        let report = create_test_report();
        let reporter = Reporter::new(ReportConfig::json().with_suggestions(false));
        let output = reporter.format(&report);

        let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
        let violation = &parsed["violations"][0];

        assert!(violation.get("suggestion").is_none());
    }

    #[test]
    fn test_html_format() {
        let report = create_test_report();
        let reporter = Reporter::html();
        let output = reporter.format(&report);

        // Check HTML structure
        assert!(output.contains("<!DOCTYPE html>"));
        assert!(output.contains("<html lang=\"en\">"));
        assert!(output.contains("</html>"));

        // Check content
        assert!(output.contains("Compliance Validation Report"));
        assert!(output.contains("HIPAA v1.0.0"));
        assert!(output.contains("status-failed"));
        assert!(output.contains("no-phi-in-logs"));
        assert!(output.contains("phi-requires-auth"));

        // Check violations are grouped by severity
        assert!(output.contains("<h2>Errors</h2>"));
        assert!(output.contains("<h2>Warnings</h2>"));

        // Check suggestions are included
        assert!(output.contains("Suggestion"));
        assert!(output.contains("Use redacted logging"));
    }

    #[test]
    fn test_html_format_passing() {
        let report = create_passing_report();
        let reporter = Reporter::html();
        let output = reporter.format(&report);

        assert!(output.contains("<!DOCTYPE html>"));
        assert!(output.contains("status-passed"));
        assert!(output.contains("PASSED"));
        assert!(output.contains("All Checks Passed"));
        assert!(output.contains("No compliance violations were found"));
    }

    #[test]
    fn test_html_escapes_special_characters() {
        let report = ValidationReport {
            framework_name: "Test".to_string(),
            framework_version: "1.0.0".to_string(),
            violations: vec![Violation {
                rule_id: "test-rule".to_string(),
                severity: Severity::Error,
                description: "Test <script>alert('xss')</script>".to_string(),
                location: "module.Class.method".to_string(),
                issue: "Issue with <dangerous> & 'quotes' and \"doubles\"".to_string(),
                suggestion: None,
            }],
            rules_checked: 1,
            modules_validated: 1,
        };

        let reporter = Reporter::html();
        let output = reporter.format(&report);

        // Verify XSS is escaped
        assert!(!output.contains("<script>"));
        assert!(output.contains("&lt;script&gt;"));
        assert!(output.contains("&lt;dangerous&gt;"));
        assert!(output.contains("&amp;"));
        assert!(output.contains("&#x27;quotes&#x27;"));
    }

    #[test]
    fn test_html_without_suggestions() {
        let report = create_test_report();
        let reporter = Reporter::new(ReportConfig::html().with_suggestions(false));
        let output = reporter.format(&report);

        // The suggestion div should not be present
        assert!(!output.contains("Use redacted logging"));
    }

    #[test]
    fn test_html_escape_function() {
        assert_eq!(html_escape("<script>"), "&lt;script&gt;");
        assert_eq!(html_escape("a & b"), "a &amp; b");
        assert_eq!(html_escape("\"quoted\""), "&quot;quoted&quot;");
        assert_eq!(html_escape("it's"), "it&#x27;s");
        assert_eq!(html_escape("normal text"), "normal text");
    }
}
