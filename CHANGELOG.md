# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-08-27

### Added
- Builtin semantic target classes: `ip` (IPv4/IPv6), `url`, `num`, `hash`, `path`, `email`.
- NDJSON top-level field extraction via `.<key>`.
- 1-indexed column delimiter extraction via `:<n>`, `/<n>`, `,<n>`.
- Literal wildcard template extraction via `<prefix>{}<suffix>`.
- Raw regular expression fallback with automatic capture group 1 detection.
- Lossy stream handling for invalid UTF-8 byte sequences using `U+FFFD`.
- Clean broken-pipe handling for downstream consumers (`SIGPIPE`).
