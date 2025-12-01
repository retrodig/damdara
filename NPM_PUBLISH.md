# NPM Publishing Guide

This guide explains how to publish the Damdara WASM package to npm.

## Prerequisites

1. **npm account**: Create an account at [npmjs.com](https://www.npmjs.com/)
2. **npm CLI**: Installed with Node.js
3. **Login**: `npm login` to authenticate

## Publishing Process

### 1. Build the WASM Package

Build the optimized production package:

```bash
# Build for production (optimized for size)
wasm-pack build --target web --out-dir pkg

# Or use the build script
./build-wasm.sh
```

This generates the `pkg/` directory with:
- `damdara.js` - JavaScript bindings
- `damdara_bg.wasm` - WebAssembly binary
- `damdara.d.ts` - TypeScript definitions
- `package.json` - npm package metadata

### 2. Update npm-specific README

Copy the npm-focused README:

```bash
cp README_NPM.md pkg/README.md
```

### 3. Verify Package Contents

Check what will be published:

```bash
cd pkg
npm pack --dry-run
```

This shows you what files will be included in the package.

### 4. Test Locally

Before publishing, test the package locally:

```bash
# In the pkg directory
npm pack

# In a test project
npm install /path/to/damdara/pkg/damdara-0.8.4.tgz

# Test the import
node
> const damdara = await import('damdara');
> console.log(damdara);
```

### 5. Publish to npm

```bash
cd pkg

# For first-time publish
npm publish

# For scoped packages (e.g., @retrodig/damdara)
npm publish --access public
```

### 6. Verify Publication

Visit your package page:
```
https://www.npmjs.com/package/damdara
```

Test installation:
```bash
npm install damdara
```

## Version Management

### Updating the Version

1. Update version in `Cargo.toml`:
   ```toml
   [package]
   version = "0.8.5"  # Increment version
   ```

2. Rebuild the package:
   ```bash
   ./build-wasm.sh
   ```

3. The version in `pkg/package.json` is automatically updated

4. Publish the new version:
   ```bash
   cd pkg
   npm publish
   ```

### Semantic Versioning

Follow [semver](https://semver.org/):
- **MAJOR** (1.0.0): Breaking API changes
- **MINOR** (0.9.0): New features, backward compatible
- **PATCH** (0.8.5): Bug fixes, backward compatible

## Automated Publishing with GitHub Actions

Create `.github/workflows/npm-publish.yml`:

```yaml
name: Publish to npm

on:
  release:
    types: [created]

jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown

      - name: Install wasm-pack
        run: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

      - name: Build WASM
        run: wasm-pack build --target web --out-dir pkg

      - name: Copy README
        run: cp README_NPM.md pkg/README.md

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'
          registry-url: 'https://registry.npmjs.org'

      - name: Publish to npm
        working-directory: ./pkg
        run: npm publish
        env:
          NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}
```

### Setup npm Token

1. Go to npmjs.com → Account → Access Tokens
2. Generate a new "Automation" token
3. Add to GitHub: Repository → Settings → Secrets → New secret
   - Name: `NPM_TOKEN`
   - Value: Your npm token

### Publishing Flow

1. Update version in `Cargo.toml`
2. Commit and push:
   ```bash
   git add Cargo.toml
   git commit -m "chore: bump version to 0.8.5"
   git push
   ```

3. Create a GitHub release:
   ```bash
   git tag v0.8.5
   git push origin v0.8.5
   ```
   Or create via GitHub UI

4. GitHub Actions automatically publishes to npm

## Package Scope

### Publishing as Scoped Package

If you want to publish as `@retrodig/damdara`:

1. Update `Cargo.toml`:
   ```toml
   [package]
   name = "damdara"  # Keep simple name for Rust
   ```

2. Update `pkg/package.json` after build:
   ```json
   {
     "name": "@retrodig/damdara",
     ...
   }
   ```

3. Publish with public access:
   ```bash
   npm publish --access public
   ```

## Troubleshooting

### "Package already exists"
- You're trying to publish a version that already exists
- Increment the version in `Cargo.toml`

### "You must be logged in"
```bash
npm login
```

### "402 Payment Required"
- Trying to publish a scoped package without access
- Use `npm publish --access public`

### "WASM file too large"
- Check release build optimization in `Cargo.toml`
- Verify `wasm-opt` is running
- Current size should be ~800KB

### "Module not found"
- Verify `files` array in `package.json` includes all necessary files
- Check that `main` and `types` fields are correct

## Best Practices

1. **Always test before publishing**
   ```bash
   npm pack --dry-run
   ```

2. **Use version tags**
   ```bash
   npm publish --tag beta  # For pre-releases
   npm publish --tag latest  # For stable releases
   ```

3. **Keep CHANGELOG.md updated**
   Document all changes for each version

4. **Deprecate old versions if needed**
   ```bash
   npm deprecate damdara@0.8.3 "Critical bug, please upgrade to 0.8.4+"
   ```

5. **Use .npmignore** (if needed)
   Create `pkg/.npmignore` to exclude test files or docs:
   ```
   tests/
   *.test.js
   .gitignore
   ```

## Package Statistics

After publishing, monitor:
- **Downloads**: https://npm-stat.com/charts.html?package=damdara
- **Bundle size**: https://bundlephobia.com/package/damdara
- **Package quality**: https://snyk.io/advisor/npm-package/damdara

## Unpublishing

⚠️ **Warning**: Unpublishing is discouraged and has restrictions

```bash
# Only within 72 hours of publishing
npm unpublish damdara@0.8.4

# Better: deprecate instead
npm deprecate damdara@0.8.4 "This version has been deprecated"
```

## Support

- npm documentation: https://docs.npmjs.com/
- wasm-pack guide: https://rustwasm.github.io/docs/wasm-pack/
- Questions: https://github.com/retrodig/damdara/issues
