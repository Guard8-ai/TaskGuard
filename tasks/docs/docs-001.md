---
id: docs-001
title: Update agentic AI guide to reflect new CLI-first approach
status: done
priority: medium
tags:
- docs
dependencies:
- backend-004
- backend-005
assignee: developer
created: 2025-09-28T07:13:47.850199160Z
estimate: ~
complexity: 3
area: docs
---

# Update agentic AI guide to reflect new CLI-first approach

## Context
Current AGENTIC_AI_TASKGUARD_GUIDE.md promotes manual file editing workflows that create friction for AI agents. After implementing new CLI update commands (backend-004, backend-005, backend-006), the guide needs comprehensive updates to reflect the new CLI-first approach for deterministic task management.

## Objectives
- Replace manual editing recommendations with CLI-first workflows
- Update examples to use new `taskguard update` commands
- Maintain best practices while reducing AI friction
- Preserve the core philosophy of tool hygiene and validation

## Tasks
- [ ] Review current guide structure and identify manual editing sections
- [ ] Replace "edit task files immediately" with CLI update commands
- [ ] Update workflow examples to use `taskguard update status <id> <status>`
- [ ] Add new CLI command examples and best practices
- [ ] Update dependency management workflow to use CLI
- [ ] Revise "Template vs Real Content" section for new approach
- [ ] Add granular task item management examples (if backend-006 completed)
- [ ] Update "AI Agent Best Practices" section with CLI commands
- [ ] Revise example workflows to be CLI-centric
- [ ] Test updated guide with sample TaskGuard session

## Acceptance Criteria
✅ **CLI-First Approach:**
- Guide promotes CLI commands over manual file editing
- Examples use `taskguard update` commands instead of vim/editing
- Workflow maintains validation steps with `taskguard validate`

✅ **Content Quality:**
- All examples are accurate and tested
- Guide maintains tool hygiene principles
- Best practices section reflects new CLI capabilities
- Legacy manual editing remains as fallback option

✅ **Completeness:**
- Covers all new CLI commands implemented in backend-004
- Includes template improvements from backend-005
- Documents any granular item management from backend-006
- Maintains compatibility with existing TaskGuard installations

## Technical Notes
- Depends on completion of backend-004 (CLI update commands)
- Should reference new CLI commands with accurate syntax
- Preserve existing validation and dependency chain concepts
- Update example command sequences to reflect new CLI workflow
- Keep platform-agnostic approach and general workflow principles

## Updates
- 2025-09-28: Task created