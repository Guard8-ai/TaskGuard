---
id: backend-017
title: Auto-update config areas when new task area directories are created
status: todo
priority: low
tags:
- backend
- config
- dx
dependencies: []
assignee: developer
created: 2025-11-04T15:47:07.337846131Z
estimate: 2h
complexity: 4
area: backend
---

# Auto-update config areas when new task area directories are created

## Context

When creating tasks in new areas (e.g., `taskguard create --area research`), TaskGuard shows a warning:

```
⚠️  Warning: Area 'github' is not in configured areas: ["setup", "backend", "frontend", "api", "auth", "testing", "deployment", "security"]
   Continuing anyway...
```

**Current state:**
- Configured areas: `["setup", "backend", "frontend", "api", "auth", "testing", "deployment", "security"]` (in `.taskguard/config.toml`)
- Actual areas: `["api", "auth", "backend", "causality", "deployment", "github", "testing", "tools"]`

**Problem:**
1. Areas like "github", "causality", "tools" are not in config but exist in tasks/
2. Users get warnings for valid areas
3. Config becomes out of sync with reality
4. No automatic discovery or synchronization

**Root cause:** Areas are manually configured in `.taskguard/config.toml` and never auto-updated

## Objectives

- Automatically detect new task area directories
- Update config.toml with discovered areas
- Provide validation command to check area sync
- Add option to auto-sync areas during create/validate
- Maintain backward compatibility (don't remove manually-added areas)

## Implementation Options

### Option 1: Auto-sync in `validate` command

Add `--sync-areas` flag to validate command:

```bash
taskguard validate --sync-areas
```

**Behavior:**
1. Scan `tasks/` directory for subdirectories
2. Compare with configured areas
3. Add new areas to config
4. Report what was added
5. Preserve manually-configured areas not in filesystem

**Location:** `src/commands/validate.rs`

```rust
pub fn run(sync_areas: bool) -> Result<()> {
    // ... existing validation ...

    if sync_areas {
        sync_config_areas()?;
    }

    Ok(())
}

fn sync_config_areas() -> Result<()> {
    let tasks_dir = get_tasks_dir()?;
    let config_path = find_taskguard_root()?.join(".taskguard/config.toml");

    // Discover actual areas from filesystem
    let mut discovered_areas: Vec<String> = fs::read_dir(&tasks_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().ok()?.is_dir())
        .filter_map(|entry| entry.file_name().into_string().ok())
        .collect();
    discovered_areas.sort();

    // Load current config
    let mut config: toml::Value = fs::read_to_string(&config_path)?
        .parse()?;

    let current_areas = config["project"]["areas"]
        .as_array()
        .cloned()
        .unwrap_or_default();

    // Find new areas
    let current_area_strings: Vec<String> = current_areas
        .iter()
        .filter_map(|v| v.as_str())
        .map(String::from)
        .collect();

    let new_areas: Vec<&String> = discovered_areas
        .iter()
        .filter(|area| !current_area_strings.contains(area))
        .collect();

    if new_areas.is_empty() {
        println!("✅ Config areas are in sync with task directories");
        return Ok(());
    }

    // Add new areas
    println!("🔄 Syncing config areas with task directories");
    println!("   Adding new areas:");
    for area in &new_areas {
        println!("   + {}", area);
    }

    // Merge and update config
    let mut all_areas = current_area_strings;
    all_areas.extend(new_areas.iter().map(|s| s.to_string()));
    all_areas.sort();

    config["project"]["areas"] = toml::Value::Array(
        all_areas.into_iter().map(toml::Value::String).collect()
    );

    // Write back
    fs::write(&config_path, toml::to_string_pretty(&config)?)?;
    println!("   ✅ Updated .taskguard/config.toml");

    Ok(())
}
```

### Option 2: Add dedicated `sync` command

```bash
taskguard sync-config  # Sync config with filesystem
```

### Option 3: Auto-sync on every `create`

Automatically add area to config when creating task in new area.

**Pros:** Seamless UX, no manual intervention
**Cons:** Potentially unexpected config modifications

## Recommended Approach

**Hybrid approach:**

1. Add `--sync-areas` flag to `validate` command (Option 1)
2. Auto-sync during `create` if area not in config (Option 3)
3. Add helpful message: "New area 'github' added to config"

This provides:
- Automatic sync during normal workflow (create)
- Manual sync capability (validate --sync-areas)
- Clear user feedback

## Tasks

- [ ] Add `sync_areas` parameter to validate command
- [ ] Implement `sync_config_areas()` function
- [ ] Add area discovery from tasks/ directory
- [ ] Update config.toml with new areas
- [ ] Add auto-sync to create command
- [ ] Update CLI help text for new flag
- [ ] Test with new areas
- [ ] Test preserves manually-added areas

## Acceptance Criteria

✅ **Validate command syncs areas:**
- `taskguard validate --sync-areas` discovers new areas
- New areas added to config.toml
- Manually-configured areas preserved
- Clear output showing what was added

✅ **Create command auto-syncs:**
- Creating task in new area auto-adds to config
- No warning shown for newly-added areas
- User sees confirmation: "Area 'github' added to config"

✅ **Config integrity maintained:**
- Existing areas not removed
- Areas sorted alphabetically
- TOML format preserved
- No data loss

## Technical Notes

**Config location:** `.taskguard/config.toml`

**Current format:**
```toml
[project]
name = "TaskGuard"
version = "0.3.0-dev"
areas = ["setup", "backend", "frontend", "api", "auth", "testing", "deployment", "security"]
```

**Dependencies:**
- `toml` crate (already used)
- `fs::read_dir` for directory scanning
- `config.rs` module updates

**Edge cases:**
- Hidden directories (start with `.`) - should skip
- Non-area directories (e.g., temp files) - skip non-dirs
- Concurrent modifications to config - use file locking or atomic writes
- Empty areas (directory exists but no tasks) - still add to config

## Alternative Solutions Considered

**Option: Warn-only mode**
Just warn about missing areas, don't auto-add
- Pros: No unexpected config changes
- Cons: Annoying warnings, manual maintenance required

**Option: Remove area concept entirely**
Scan filesystem dynamically, no config
- Pros: Always in sync, no config maintenance
- Cons: Breaking change, removes user control

**Rejected:** Both alternatives reduce UX quality

## Updates
- 2025-11-04: Task created
- Current mismatch: Config has 8 areas, filesystem has 8 (4 different)
- Recommended: Hybrid approach with auto-sync + manual flag
