# flutter-unused-files

Check for unused files and assets in a Flutter project

## How it works

It will take the asset and/or file name and search all the files to see if it is referenced. If it is not, it is seen as an unused asset or file.

## Inputs

- `path` - Optional - Relative path to the Flutter app/package that needs to checked for unused assets or files. It will expect a `lib` folder to be present at the path e.g. `path: packages/my-package`. Default: `.`
- `warning` - Optional - Instead of exiting with an error, just list the unused assets/files as a warning. Default: `false`
- `assets` - Optional - Whether the assets checks should be ignored. Needs to be considered if app has displays the asset based off of a API response. Default: `false`

## TODO

- [x] Update this README with the details of this action
- [x] Update inputs/outputs in `action.yaml`
- [x] Implement the action's logic in `src/main.rs`
- [ ] Trigger a release in GitHub Actions
- [ ] Edit the triggered release to set release notes and publish the action to GitHub Marketplace

## Example

### Normal App

```yaml
name: Check for unused files
on:
  pull_request:
    types: [opened, edited, synchronize]

jobs:
  check_files:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: ZimboPro/flutter-unused-files@v1.0.0
```

### Ignore Assets

```yaml
name: Check for unused files
on:
  pull_request:
    types: [opened, edited, synchronize]

jobs:
  check_files:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: ZimboPro/flutter-unused-files@v1.0.0
        with:
            assets: true
```

### Specific package

```yaml
name: Check for unused files
on:
  pull_request:
    types: [opened, edited, synchronize]

jobs:
  check_files:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: ZimboPro/flutter-unused-files@v1.0.0
        with:
            path: packages/my-package
```

### Output warnings instead of an error

```yaml
name: Check for unused files
on:
  pull_request:
    types: [opened, edited, synchronize]

jobs:
  check_files:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: ZimboPro/flutter-unused-files@v1.0.0
        with:
            warning: true
```
