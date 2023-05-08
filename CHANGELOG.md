# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

* Add support for async with tokio
* Now the files are rendered in parallel

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
