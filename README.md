<div align="center">

# snag

**modern stream pattern extractor**

[![crates.io](https://img.shields.io/crates/v/snag-cli.svg?style=flat-square&color=cba6f7)](https://crates.io/crates/snag-cli)
[![license: mit](https://img.shields.io/badge/license-mit-89b4fa.svg?style=flat-square)](LICENSE)
[![ci](https://img.shields.io/badge/build-passing-a6e3a1.svg?style=flat-square)](#)

</div>

<p align="center">
  <img src="assets/demo.gif" alt="snag demo" width="800">
</p>

`snag` is a single-purpose Unix filter that reads standard input, matches a requested target pattern against each line, and writes matches to standard output.

## Installation

```sh
# Via crates.io (installs the `snag` binary)
cargo install snag-cli

# From source
git clone https://github.com/programmersd21/snag.git
cd snag && cargo install --path .
```

## Usage

```sh
snag <target>
<stream> | snag <target>
```

## Targets

| Target | Description | Example |
| :--- | :--- | :--- |
| `ip` | IPv4 and IPv6 addresses | `echo "host 10.0.0.1" \| snag ip` &rarr; `10.0.0.1` |
| `url` | `http://` and `https://` URLs | `echo "see https://example.com" \| snag url` &rarr; `https://example.com` |
| `num` | Standalone integers | `echo "code: 404 retries: 2" \| snag num` &rarr; `404`, `2` |
| `hash` | Hex strings 7–64 chars (SHA, MD5) | `echo "commit a1b2c3d" \| snag hash` &rarr; `a1b2c3d` |
| `path` | Absolute Unix file paths | `echo "at /var/log/syslog" \| snag path` &rarr; `/var/log/syslog` |
| `email` | Email addresses | `echo "to user@example.com" \| snag email` &rarr; `user@example.com` |
| `.<key>` | Top-level field from NDJSON stream | `echo '{"id": 42}' \| snag .id` &rarr; `42` |
| `:<n>`, `/<n>`, `,<n>` | 1-indexed column delimiter split | `echo "root:x:0" \| snag :1` &rarr; `root` |
| `<pre>{}<post>` | Literal template with wildcard capture | `echo "id=[123]" \| snag 'id=[{}]'` &rarr; `123` |
| `<regex>` | Raw regex fallback (group 1 or match) | `echo "port=8080" \| snag 'port=(\d+)'` &rarr; `8080` |

## Examples

```sh
# Extract unique IP addresses from server logs
tail -n 1000 /var/log/nginx/access.log | snag ip | sort -u

# Extract commit hashes from git log
git log --oneline | snag hash

# Extract JSON fields from an NDJSON stream
docker events --format '{{json .}}' | snag .action

# Column extraction without awk syntax
cat /etc/passwd | snag :1

# Wildcard capture with literal auto-escaping
kubectl get pods | snag 'pod/{}-'
```

## Behavior

- **Delimiter Splitting**: Column mode (`:n`, `/n`, `,n`) performs a literal byte split without CSV quote parsing.
- **UTF-8 Handling**: Invalid byte sequences are replaced per line with the Unicode replacement character (`U+FFFD`) without aborting.
- **Exit Codes**: Returns `0` on clean runs (including 0 matches) and `1` on invalid arguments or uncompilable patterns.

## License

[MIT](LICENSE)
