//! Domain-specific task templates with causation chain prompts
//!
//! Provides tailored templates for different task areas (api, auth, backend, etc.)
//! with domain-appropriate causation chain prompts and verification commands.

use std::fs;
use std::path::Path;

/// Template manager for loading domain-specific or custom templates
pub struct TemplateManager;

impl TemplateManager {
    /// Get template content for a given area
    /// Priority: custom template > domain-specific > generic
    pub fn get_template(area: &str, taskguard_root: Option<&Path>) -> String {
        // 1. Check for custom template
        if let Some(root) = taskguard_root {
            let custom_path = root
                .join(".taskguard")
                .join("templates")
                .join(format!("{}.md", area));
            if let Ok(content) = fs::read_to_string(&custom_path) {
                return content;
            }
            // Also check for _default.md custom template
            let default_custom = root
                .join(".taskguard")
                .join("templates")
                .join("_default.md");
            if let Ok(content) = fs::read_to_string(&default_custom) {
                return content;
            }
        }

        // 2. Return domain-specific or generic template
        Self::get_builtin_template(area)
    }

    /// Get the causation chain prompt for a specific area
    pub fn get_causation_prompt(area: &str) -> &'static str {
        match area {
            "api" => {
                r#"Trace the request lifecycle: HTTP verb → middleware chain → handler →
service → data layer → response serialization. Verify actual route
registration and middleware order in code."#
            }

            "auth" => {
                r#"Trace the authentication flow: credential input → validation → token
generation → storage → verification → session state. Check actual
token expiry logic and refresh mechanism in implementation."#
            }

            "backend" => {
                r#"Trace the service orchestration: entry point → dependency injection →
business logic → side effects → return. Verify actual error propagation
paths in the codebase."#
            }

            "data" => {
                r#"Trace the data lifecycle: schema → migration → connection pool →
query execution → result mapping → cache invalidation. Check actual
transaction boundaries and rollback behavior in code."#
            }

            "deployment" => {
                r#"Trace the deployment pipeline: source → build → artifact →
environment config → runtime injection → health check. Verify actual
env var usage and fallback defaults in config files."#
            }

            "docs" => {
                r#"Trace the documentation chain: code signature → docstring → generated
docs → published output. Check actual code-to-docs sync status - are
examples runnable?"#
            }

            "integration" => {
                r#"Trace the integration boundary: our code → serialization → transport →
external API → response parsing → error mapping. Verify actual retry
logic and timeout handling in implementation."#
            }

            "security" => {
                r#"Trace the attack surface: user input → validation → sanitization →
storage → retrieval → output encoding. Check actual input validation
at each boundary in code."#
            }

            "setup" => {
                r#"Trace the initialization chain: env detection → dependency check →
config load → service bootstrap → ready state. Verify actual failure
modes and error messages in bootstrap code."#
            }

            "testing" => {
                r#"Trace the test execution flow: fixture setup → precondition → action →
assertion → teardown. Check actual test isolation - are tests
independent or order-dependent?"#
            }

            "frontend" => {
                r#"Trace the component lifecycle: props → state init → render →
effects → event handlers → state updates → re-render. Verify actual
data flow and side effect cleanup in components."#
            }

            "github" => {
                r#"Trace the GitHub integration flow: local state → API call →
response parsing → state update → sync verification. Check actual
rate limiting and error handling in implementation."#
            }

            "causality" => {
                r#"Trace the causality chain: trigger event → state mutation →
dependent updates → side effects → completion. Verify actual
event ordering and failure propagation in code."#
            }

            _ => {
                r#"Trace the execution flow: input → processing → side effects →
output. Verify actual implementation matches expected behavior."#
            }
        }
    }

    /// Get verification commands for a specific area
    pub fn get_verification_commands(area: &str) -> &'static str {
        match area {
            "api" => {
                r#"- [ ] `grep -r "route\|path\|endpoint" src/` - Verify route registration
- [ ] Check actual middleware order in router setup
- [ ] Verify response serialization matches API contract"#
            }

            "auth" => {
                r#"- [ ] `grep -r "verify\|validate\|decode" src/` - Find token validation
- [ ] Check actual token expiry configuration
- [ ] Verify session state management implementation"#
            }

            "backend" => {
                r#"- [ ] `grep -r "impl.*Service\|fn.*service" src/` - Find service definitions
- [ ] Check actual dependency injection patterns
- [ ] Verify error propagation through service layers"#
            }

            "data" => {
                r#"- [ ] `grep -r "SELECT\|INSERT\|query\|execute" src/` - Find queries
- [ ] Check actual transaction boundaries
- [ ] Verify migration files match schema expectations"#
            }

            "deployment" => {
                r#"- [ ] `grep -r "env\|getenv\|std::env" src/` - Find env var usage
- [ ] Check actual config file loading order
- [ ] Verify health check endpoints exist"#
            }

            "docs" => {
                r#"- [ ] Compare doc examples with actual API signatures
- [ ] Check that code snippets are runnable
- [ ] Verify cross-references are valid"#
            }

            "integration" => {
                r#"- [ ] `grep -r "fetch\|request\|Client::new" src/` - Find HTTP calls
- [ ] Check actual retry and timeout configuration
- [ ] Verify error mapping for external API responses"#
            }

            "security" => {
                r#"- [ ] `grep -r "escape\|sanitize\|validate" src/` - Find input handling
- [ ] Check actual input validation at boundaries
- [ ] Verify output encoding prevents injection"#
            }

            "setup" => {
                r#"- [ ] `grep -r "init\|bootstrap\|main" src/` - Find initialization
- [ ] Check actual failure modes and error messages
- [ ] Verify dependency checks are comprehensive"#
            }

            "testing" => {
                r#"- [ ] Read test files to verify actual assertions
- [ ] Check test isolation (no shared mutable state)
- [ ] Verify fixture setup and teardown completeness"#
            }

            "frontend" => {
                r#"- [ ] Check component prop types and defaults
- [ ] Verify effect cleanup functions exist
- [ ] Trace state update propagation through components"#
            }

            "github" => {
                r#"- [ ] `grep -r "GitHubClient\|gh " src/` - Find GitHub API calls
- [ ] Check actual rate limit handling
- [ ] Verify sync state persistence"#
            }

            "causality" => {
                r#"- [ ] Trace event chain from trigger to completion
- [ ] Verify failure modes are handled at each step
- [ ] Check async flow ordering and race conditions"#
            }

            _ => {
                r#"- [ ] Search codebase for relevant implementations
- [ ] Verify actual behavior matches expected
- [ ] Check error handling paths"#
            }
        }
    }

    /// Get the built-in template for an area
    fn get_builtin_template(area: &str) -> String {
        let causation_prompt = Self::get_causation_prompt(area);
        let verification_commands = Self::get_verification_commands(area);

        format!(
            r#"# {{{{title}}}}

## Causation Chain
> {causation_prompt}

## Pre-flight Checks
- [ ] Read dependency task files for implementation context (Session Handoff)
{verification_commands}
- [ ] `git log --oneline -10` - Check recent related commits

## Context
[Why this task exists and what problem it solves]

## Tasks
- [ ] [Specific actionable task]
- [ ] [Another task]
- [ ] Build + test + run to verify

## Acceptance Criteria
- [ ] [Testable criterion 1]
- [ ] [Testable criterion 2]

## Notes
[Technical details, constraints, gotchas]

---
**Session Handoff** (fill when done):
- Changed: [files/functions modified]
- Causality: [what triggers what]
- Verify: [how to test this works]
- Next: [context for dependent tasks]
"#
        )
    }

    /// Render a template with the given variables
    pub fn render(template: &str, title: &str, date: &str) -> String {
        template
            .replace("{{title}}", title)
            .replace("{{date}}", date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_causation_prompt_api() {
        let prompt = TemplateManager::get_causation_prompt("api");
        assert!(prompt.contains("request lifecycle"));
        assert!(prompt.contains("middleware"));
    }

    #[test]
    fn test_get_causation_prompt_unknown() {
        let prompt = TemplateManager::get_causation_prompt("unknown_area");
        assert!(prompt.contains("execution flow"));
    }

    #[test]
    fn test_get_verification_commands() {
        let cmds = TemplateManager::get_verification_commands("auth");
        assert!(cmds.contains("verify"));
        assert!(cmds.contains("token"));
    }

    #[test]
    fn test_get_builtin_template() {
        let template = TemplateManager::get_builtin_template("backend");
        assert!(template.contains("Causation Chain"));
        assert!(template.contains("service orchestration"));
    }

    #[test]
    fn test_render_template() {
        let template = "# {{title}}\nCreated: {{date}}";
        let result = TemplateManager::render(template, "My Task", "2025-01-01");
        assert_eq!(result, "# My Task\nCreated: 2025-01-01");
    }

    #[test]
    fn test_get_template_fallback() {
        // Without custom path, should return builtin
        let template = TemplateManager::get_template("api", None);
        assert!(template.contains("request lifecycle"));
    }
}
