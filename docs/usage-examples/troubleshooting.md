# Troubleshooting

Common issues and solutions.

---

## Command Not Found

**Issue:** `taskguard: command not found`

**Solution:**
```bash
# Check PATH
echo $PATH | grep -q ".cargo/bin" && echo "✅" || echo "❌"

# Add to PATH
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

---

## Parse Errors

**Issue:** `Failed to parse YAML front-matter`

**Solution:**
- Check `---` delimiters
- Validate YAML syntax
- Ensure required fields present

---

## Circular Dependencies

**Issue:** `Circular dependency detected`

**Solution:**
```bash
taskguard validate
# Fix dependency chain in task files
```

---

## Next Steps

See [Installation](../getting-started/installation.md) for setup issues.
