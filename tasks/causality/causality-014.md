---
id: causality-014
title: Update AGENTIC_AI_TASKGUARD_GUIDE.md in all /data/ projects (level 3)
status: done
priority: high
tags:
- causality
- v0.4.0
- deploy
- propagate
dependencies:
- causality-013
assignee: developer
created: 2025-12-21T12:00:00Z
estimate: 30m
complexity: 4
area: causality
---

# Update AGENTIC_AI_TASKGUARD_GUIDE.md in all /data/ projects (level 3)

> **AI Agent Instructions:**
> 1. Find all AGENTIC_AI_TASKGUARD_GUIDE.md files in /data/ (up to 3 levels deep)
> 2. Copy the updated guide from TaskGuard project
> 3. Verify each copy is correct

## Steps

### 1. Find All Guide Files
```bash
find /data -maxdepth 3 -name "AGENTIC_AI_TASKGUARD_GUIDE.md" -type f 2>/dev/null
```

### 2. Source File
The canonical source is:
```
/data/git/Guard8.ai/TaskGuard/AGENTIC_AI_TASKGUARD_GUIDE.md
```

### 3. Copy to All Locations
```bash
# Get the source file
SOURCE="/data/git/Guard8.ai/TaskGuard/AGENTIC_AI_TASKGUARD_GUIDE.md"

# Find and update all copies
find /data -maxdepth 3 -name "AGENTIC_AI_TASKGUARD_GUIDE.md" -type f 2>/dev/null | while read file; do
    if [ "$file" != "$SOURCE" ]; then
        echo "Updating: $file"
        cp "$SOURCE" "$file"
    fi
done
```

### 4. Verify Updates
```bash
# Check all files have same content
find /data -maxdepth 3 -name "AGENTIC_AI_TASKGUARD_GUIDE.md" -type f 2>/dev/null | while read file; do
    echo -n "$file: "
    grep -q "Causality Tracking" "$file" && echo "✅ Updated" || echo "❌ Not updated"
done
```

## Files to Update

Expected locations (example):
- `/data/git/Guard8.ai/TaskGuard/AGENTIC_AI_TASKGUARD_GUIDE.md` (source)
- `/data/projects/*/AGENTIC_AI_TASKGUARD_GUIDE.md`
- `/data/eliran/*/AGENTIC_AI_TASKGUARD_GUIDE.md`
- Other project directories with TaskGuard

## Acceptance Criteria

- [x] All AGENTIC_AI_TASKGUARD_GUIDE.md files found
- [x] All copies updated with v0.4.0 content
- [x] All copies contain "Causality Tracking" section
- [x] No file permission errors

## Session Handoff

**Completed:** 2025-12-21

**What was done:**
- Found 12 AGENTIC_AI_TASKGUARD_GUIDE.md files in /data (level 4)
- Updated all files with v0.4.0 causality tracking content
- Verified all contain "v0.4.0" marker

**Projects updated:**
- /data/git/Controlis/
- /data/git/FortuitaSolutions/
- /data/git/Guard8.ai/AutonomousDialogue/
- /data/git/Guard8.ai/ContextGuard/
- /data/git/Guard8.ai/CookbookGuard/
- /data/git/Guard8.ai/grapheme-nn/
- /data/git/Guard8.ai/mitmproxy-rs/
- /data/git/Guard8.ai/SEAR-Enterprise/
- /data/git/Guard8.ai/sear-workbench/
- /data/git/Guard8.ai/SteganoGuard/
- /data/git/Guard8.ai/VisionGuard/
- /data/git/Guard8.ai/TaskGuard/ (source)