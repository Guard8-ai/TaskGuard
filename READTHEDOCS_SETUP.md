# ReadTheDocs Setup Guide for TaskGuard

## Quick Setup (Recommended - Web UI)

### 1. Sign in to ReadTheDocs
1. Go to [readthedocs.org](https://readthedocs.org)
2. Click "Sign in with GitHub"
3. Authorize ReadTheDocs to access your repositories

### 2. Import TaskGuard Project
1. Click your username → "My Projects"
2. Click "Import a Project"
3. Find "Guard8-ai/TaskGuard" in the list (or click "Import Manually")
4. Click the "+" button next to it

### 3. Configure Project (Auto-detected)
ReadTheDocs will automatically detect:
- ✅ `.readthedocs.yml` configuration
- ✅ `mkdocs.yml` documentation setup
- ✅ Python 3.11 environment
- ✅ Material theme dependencies

### 4. Verify Settings
Check these are correct:
- **Name**: TaskGuard
- **Repository URL**: https://github.com/Guard8-ai/TaskGuard
- **Default branch**: master
- **Default version**: latest

### 5. Build Documentation
1. Click "Build version: latest"
2. Wait for build to complete (usually 2-3 minutes)
3. View your docs at: https://taskguard.readthedocs.io

### 6. Enable Webhooks (Automatic Builds)
ReadTheDocs automatically adds a webhook to your GitHub repository.
Verify it exists:
1. Go to GitHub: Settings → Webhooks
2. You should see `https://readthedocs.org/api/v2/webhook/...`
3. This triggers builds on every push to master

---

## API Setup (Alternative)

If you prefer using the API, run:

```bash
READTHEDOCS_API_KEY="your_key_here" ./setup-readthedocs.sh
```

---

## Configuration Files

### `.readthedocs.yml`
```yaml
version: 2

build:
  os: ubuntu-22.04
  tools:
    python: "3.11"

mkdocs:
  configuration: mkdocs.yml

python:
  install:
    - requirements: docs/requirements.txt
```

### `docs/requirements.txt`
```
mkdocs>=1.5.0
mkdocs-material>=9.4.0
pymdown-extensions>=10.3
```

---

## Troubleshooting

### Build Failed
1. Check the build log in ReadTheDocs dashboard
2. Verify all files are committed and pushed to GitHub
3. Ensure `docs/requirements.txt` has all dependencies

### 404 Error on Documentation URL
- Wait 5-10 minutes for DNS propagation
- Try the full URL: `https://taskguard.readthedocs.io/en/latest/`

### Documentation Not Updating
- Check webhook is active in GitHub settings
- Manually trigger build in ReadTheDocs dashboard
- Verify you pushed to the correct branch (master)

---

## Custom Domain (Optional)

To use a custom domain like `docs.taskguard.io`:

1. ReadTheDocs → Project Settings → Domains
2. Add your custom domain
3. Add CNAME record in your DNS:
   ```
   docs.taskguard.io → taskguard.readthedocs.io
   ```

---

## Next Steps

After setup:
1. ✅ Documentation builds automatically on every push
2. 📝 Edit docs in `docs/` directory
3. 🔍 Search works out of the box
4. 🎨 Material theme with dark mode enabled
5. 📱 Mobile-responsive design

**Documentation URL**: https://taskguard.readthedocs.io
