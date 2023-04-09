# Creatorly: a CLI Tool in Rust to manage creation of projects

Note: This project is inspired by [cookiecutter](https://github.com/cookiecutter/cookiecutter)!

[![ci](https://github.com/BuriKizilkaya/creatorly/actions/workflows/ci.yml/badge.svg)](https://github.com/BuriKizilkaya/creatorly/actions/workflows/ci.yml)

Creatorly is a project to generate repository from template. Addionally, it set up the remote repository with the desired policies. Main features are:

* Create a project folder from a template with specified template variables. These variables can be parsed from the command line.

## Installation

```bash
curl https://raw.githubusercontent.com/BuriKizilkaya/creatorly/main/install.sh | bash
```

## Documentation

Here are some documentations about the project:

* [Usage](docs/usage/usage.md)
* [Architecture](docs/architecture/architecture.md)

## Contributing

### Release

To release a new version, follow these steps:

1. Update the version in `Cargo.toml`
2. Run `release-plz update` to generate a new changelog
3. Commit the changes and create a new PR
4. Once the PR is merged, create a git tag with the version number and push it to the repository
