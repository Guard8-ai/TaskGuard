---
id: backend-023
title: Domain-specific task templates with causation chain prompts
status: done
priority: high
tags:
- backend
- templates
- causality
- ux
dependencies:
- causality-upgrade-001
assignee: developer
created: 2025-12-13T17:09:12.122311475Z
estimate: 8h
complexity: 7
area: backend
---

# Domain-specific task templates with causation chain prompts

> **⚠️ SESSION WORKFLOW NOTICE (for AI Agents):**
>
> **This task should be completed in ONE dedicated session.**
>
> When you mark this task as `done`, you MUST:
> 1. Fill the "Session Handoff" section at the bottom with complete implementation details
> 2. Document what was changed, what runtime behavior to expect, and what dependencies were affected
> 3. Create a clear handoff for the developer/next AI agent working on dependent tasks
>
> **If this task has dependents,** the next task will be handled in a NEW session and depends on your handoff for context.

## Intent
Provide domain-specific task templates that include area-appropriate causation chain prompts and pre-flight verification checks. This guides AI agents to verify actual code behavior rather than making assumptions.

## Context
Currently, `taskguard create` uses a single generic template for all areas. Different domains (api, auth, backend, data, security, etc.) have distinct causation patterns that AI agents should trace. By providing domain-specific templates with tailored prompts and verification commands, we reduce hallucinations and improve code-grounded task execution.

## Objectives
- Create domain-specific templates for each standard area
- Include causation chain prompts that guide tracing actual code flows
- Add pre-flight check commands (grep patterns) for verification
- Allow custom templates in `.taskguard/templates/`
- Fall back to generic template for unknown areas

## Domain-Specific Causation Chain Prompts

### api/
```
"Trace the request lifecycle: HTTP verb → middleware chain → handler →
service → data layer → response serialization. Verify actual route
registration and middleware order in code."
```
**Verify with:** `grep -r "route|path|endpoint" src/`

### auth/
```
"Trace the authentication flow: credential input → validation → token
generation → storage → verification → session state. Check actual
token expiry logic and refresh mechanism in implementation."
```
**Verify with:** `grep -r "verify|validate|decode" src/`

### backend/
```
"Trace the service orchestration: entry point → dependency injection →
business logic → side effects → return. Verify actual error propagation
paths in the codebase."
```
**Verify with:** `grep -r "import|from.*service" src/`

### data/
```
"Trace the data lifecycle: schema → migration → connection pool →
query execution → result mapping → cache invalidation. Check actual
transaction boundaries and rollback behavior in code."
```
**Verify with:** `grep -r "SELECT|INSERT|query" src/`

### deployment/
```
"Trace the deployment pipeline: source → build → artifact →
environment config → runtime injection → health check. Verify actual
env var usage and fallback defaults in config files."
```
**Verify with:** `grep -r "env|getenv|process.env" src/`

### docs/
```
"Trace the documentation chain: code signature → docstring → generated
docs → published output. Check actual code-to-docs sync status - are
examples runnable?"
```
**Verify with:** Check doc examples against actual API signatures

### integration/
```
"Trace the integration boundary: our code → serialization → transport →
external API → response parsing → error mapping. Verify actual retry
logic and timeout handling in implementation."
```
**Verify with:** `grep -r "fetch|request|axios|http" src/`

### security/
```
"Trace the attack surface: user input → validation → sanitization →
storage → retrieval → output encoding. Check actual input validation
at each boundary in code."
```
**Verify with:** `grep -r "escape|sanitize|validate" src/`

### setup/
```
"Trace the initialization chain: env detection → dependency check →
config load → service bootstrap → ready state. Verify actual failure
modes and error messages in bootstrap code."
```
**Verify with:** `grep -r "init|bootstrap|main" src/`

### testing/
```
"Trace the test execution flow: fixture setup → precondition → action →
assertion → teardown. Check actual test isolation - are tests
independent or order-dependent?"
```
**Verify with:** Read the test file directly

## Tasks
- [ ] Create `TemplateManager` struct to handle template loading
- [ ] Add domain-specific templates as embedded strings or files
- [ ] Modify `create.rs` to use `TemplateManager`
- [ ] Support custom templates in `.taskguard/templates/{area}.md`
- [ ] Add `--template` flag to `create` command for explicit template selection
- [ ] Include pre-flight grep commands in each template
- [ ] Update documentation with template customization guide
- [ ] Write tests for template loading and fallback behavior

## Acceptance Criteria

✅ **Domain-specific templates:**
- Each standard area (api, auth, backend, data, deployment, docs, integration, security, setup, testing) has a tailored template
- Templates include the causation chain prompt for that domain
- Templates include pre-flight verification commands

✅ **Custom templates:**
- Users can add `.taskguard/templates/{area}.md` to override defaults
- Custom templates are loaded preferentially over built-ins
- Unknown areas fall back to generic template

✅ **CLI integration:**
- `taskguard create --area api` uses api-specific template
- `taskguard create --template custom` allows explicit template selection
- Template used is indicated in output

## Technical Notes

### Template Loading Order
1. `.taskguard/templates/{area}.md` (user custom)
2. Built-in domain-specific template
3. Generic fallback template

### Template Variables
Templates should support these variables:
- `{title}` - Task title
- `{area}` - Task area
- `{id}` - Generated task ID
- `{date}` - Creation date
- `{causation_prompt}` - Domain-specific causation chain prompt
- `{verification_commands}` - Pre-flight grep commands

### File Structure
```
.taskguard/
├── templates/
│   ├── api.md        # Custom api template
│   ├── backend.md    # Custom backend template
│   └── _default.md   # Custom default template
```

## Task Dependencies
- **Blocks**: Template-based task quality improvements
- **Blocked by**: causality-upgrade-001 (base template must exist first)
- **Related**: causality-upgrade-002, causality-upgrade-003

## Testing
- [ ] Unit test: TemplateManager loads correct template for each area
- [ ] Unit test: Custom templates override built-ins
- [ ] Unit test: Unknown areas fall back to generic
- [ ] Integration test: `taskguard create --area api` produces api-specific content
- [ ] Edge case: Missing template directory doesn't crash

## Updates
- 2025-12-13: Task created with full specification

## Session Handoff (AI: Complete this when marking task done)
**For the next session/agent working on dependent tasks:**

### What Changed
- [Document code changes, new files, modified functions]
- [What runtime behavior is new or different]

### Causality Impact
- [What causal chains were created or modified]
- [What events trigger what other events]
- [Any async flows or timing considerations]

### Dependencies & Integration
- [What dependencies were added/changed]
- [How this integrates with existing code]
- [What other tasks/areas are affected]

### Verification & Testing
- [How to verify this works]
- [What to test when building on this]
- [Any known edge cases or limitations]

### Context for Next Task
- [What the next developer/AI should know]
- [Important decisions made and why]
- [Gotchas or non-obvious behavior]