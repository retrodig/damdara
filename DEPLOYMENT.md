# GitHub Pages Deployment Guide

This guide explains how to deploy the Damdara WASM demo to GitHub Pages.

## Prerequisites

- GitHub repository with the Damdara project
- WASM package built and copied to `docs/pkg/`

## Setup GitHub Pages

### 1. Build the WASM Package

From the project root, build the WASM package:

```bash
./build-wasm.sh dev
```

This will:
- Build the WASM package to `pkg/`
- Automatically copy it to `docs/pkg/` for GitHub Pages

### 2. Commit and Push

Commit the `docs/` directory (excluding `docs/pkg/` which is in .gitignore):

```bash
git add docs/
git add .gitignore
git add build-wasm.sh
git commit -m "Add GitHub Pages deployment"
git push origin main
```

**Important**: The `docs/pkg/` directory should NOT be committed to the repository. Instead, you'll build it as part of the deployment process.

### 3. Configure GitHub Pages

1. Go to your GitHub repository
2. Click on **Settings** > **Pages**
3. Under **Source**, select:
   - **Branch**: `main` (or your default branch)
   - **Folder**: `/docs`
4. Click **Save**

### 4. Add GitHub Actions Workflow (Recommended)

To automatically build and deploy the WASM package, create `.github/workflows/deploy.yml`:

```yaml
name: Deploy to GitHub Pages

on:
  push:
    branches: [ main ]
  workflow_dispatch:

permissions:
  contents: read
  pages: write
  id-token: write

jobs:
  build-and-deploy:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown

      - name: Install wasm-pack
        run: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

      - name: Build WASM
        run: |
          ./build-wasm.sh dev

      - name: Setup Pages
        uses: actions/configure-pages@v4

      - name: Upload artifact
        uses: actions/upload-pages-artifact@v3
        with:
          path: './docs'

      - name: Deploy to GitHub Pages
        id: deployment
        uses: actions/deploy-pages@v4
```

### 5. Manual Deployment (Alternative)

If you prefer manual deployment without GitHub Actions:

1. Build locally:
   ```bash
   ./build-wasm.sh dev
   ```

2. Temporarily remove `docs/pkg` from `.gitignore`:
   ```bash
   # Comment out this line in .gitignore
   # /docs/pkg
   ```

3. Commit and push:
   ```bash
   git add docs/pkg
   git commit -m "Deploy WASM package"
   git push origin main
   ```

4. Restore `.gitignore`:
   ```bash
   # Uncomment the line
   /docs/pkg
   git add .gitignore
   git commit -m "Restore .gitignore"
   git push origin main
   ```

## Accessing Your Deployment

After deployment, your site will be available at:
```
https://<username>.github.io/<repository-name>/
```

For example:
```
https://retrodig.github.io/damdara/
```

## Updating the Deployment

To update the deployed version:

### With GitHub Actions:
1. Make your changes
2. Commit and push to `main` branch
3. GitHub Actions will automatically rebuild and deploy

### Manual Update:
1. Build the package: `./build-wasm.sh dev`
2. Follow the manual deployment steps above

## Troubleshooting

### 404 Error
- Verify GitHub Pages is enabled in repository settings
- Ensure `/docs` folder is selected as the source
- Check that `index.html` exists in the `docs/` directory

### WASM Loading Errors
- Verify `docs/pkg/` contains:
  - `damdara.js`
  - `damdara_bg.wasm`
  - `damdara.d.ts`
- Check browser console for CORS or module loading errors
- Ensure you're accessing via HTTPS (GitHub Pages enforces HTTPS)

### Outdated Content
- Clear browser cache
- Wait a few minutes for GitHub Pages to update (can take up to 10 minutes)
- Check the deployment status in the **Actions** tab

## Local Testing

Before deploying, test locally:

```bash
cd docs
python -m http.server 8080
```

Then open http://localhost:8080 in your browser.

## Security Notes

- Never commit sensitive data to the repository
- The WASM binary is public and can be inspected
- All game logic is client-side and visible to users
- GitHub Pages serves static content over HTTPS

## Cost

GitHub Pages hosting is free for public repositories.
