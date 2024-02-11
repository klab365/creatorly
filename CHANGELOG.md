# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## 1.4.2 - 11.02.2024

* Bump version of creatorly

## 1.4.1 - 11.02.2024

* Add a cli print interface to better print messages to console
* Symlink follow works now with creatorly

## 1.4.0 - 11.01.2024

### Added

* Move `check` command to a seperate crate 

### Fixed

* Fix install.sh issue because of the move of the project to a new repository

## 1.3.0 - 06.01.2024

* Add `create template` command to create a template from a directory
* Add `dry-run` option to generate command. If this option is set, then the files will not be written to the file system. This is useful for debugging the template.

### Changed

* I render the hole content of a file and not line by line. This is more efficient and it is possible to place `{% raw %}` and `{% endraw %}` in the beginning and end of a file.

## 1.2.0 - 27.12.2023

* Restructure the project
    * Split into business functioanlities (generate)
    * Every business functionality has its own cli structure and will be composed in the main cli with a register function
* Restructure yml configuration (see example project)

## 1.1.0 - 17.08.2023

### Changed

* Rename `create` command to `generate`

## 1.0.4 - 14.05.2023

### Added

* Add support for async with tokio: The files will be rendered in parallel.

## 1.0.3 - 23.04.2023

### Fix

* Render line by line the content of a file, because then it is possible render a file anyway if it has invalid token.

## 1.0.2 - 18.04.2023

* If a content of a file has invalid tokens, then log a warn message and return the content how it is. Issue #19

## 1.0.1 - 16.04.2023

### Fixed

* If a input string can not be rendered than it will log a warn message. Issue #15

## 1.0.0 - 09.04.2023

### Other
- Update create cli error and install.sh
- Update files for Version 1.0.0
- Add Git files loader
- Update cli
- Add logger
- Update ci pipeline
- Fix issue #7 #6
- Update project structure
