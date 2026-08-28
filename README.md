<div align="center">

# snag

**modern stream pattern extractor**

[![crates.io](https://img.shields.io/crates/v/snag-cli.svg?style=flat-square&color=f38ba8)](https://crates.io/crates/snag-cli)
[![aur](https://img.shields.io/aur/version/snag-bin.svg?style=flat-square&color=cba6f7)](https://aur.archlinux.org/packages/snag-bin)
[![license: mit](https://img.shields.io/badge/license-mit-89dceb.svg?style=flat-square)](LICENSE)
[![ci](https://img.shields.io/github/actions/workflow/status/programmersd21/snag/ci.yml.svg?style=flat-square&label=ci&color=a6e3a1)](https://github.com/programmersd21/snag/actions/workflows/ci.yml)

</div>

<p align="center">
  <img src="assets/demo.gif" alt="snag demo" width="800">
</p>

a single-purpose unix filter that reads stdin, matches a target pattern against each line, and writes matches to stdout.

## install

```sh
# arch linux (aur)
paru -S snag-bin

# from crates.io
cargo install snag-cli

# from source
git clone https://github.com/programmersd21/snag.git
cd snag && cargo install --path .
```

## usage

```sh
snag <target>
<stream> | snag <target>
```

## targets

| target | description | example |
| :--- | :--- | :--- |
| `ip` | ipv4 and ipv6 addresses | `echo "host 10.0.0.1" \| snag ip` &rarr; `10.0.0.1` |
| `url` | `http://` and `https://` urls | `echo "see https://example.com" \| snag url` &rarr; `https://example.com` |
| `num` | standalone integers | `echo "code: 404 retries: 2" \| snag num` &rarr; `404`, `2` |
| `hash` | hex strings 7–64 chars (sha, md5) | `echo "commit a1b2c3d" \| snag hash` &rarr; `a1b2c3d` |
| `path` | absolute unix file paths | `echo "at /var/log/syslog" \| snag path` &rarr; `/var/log/syslog` |
| `email` | email addresses | `echo "to user@example.com" \| snag email` &rarr; `user@example.com` |
| `.<key>` | top-level field from ndjson stream | `echo '{"id": 42}' \| snag .id` &rarr; `42` |
| `:<n>`, `/<n>`, `,<n>` | 1-indexed column delimiter split | `echo "root:x:0" \| snag :1` &rarr; `root` |
| `<pre>{}<post>` | literal template with wildcard capture | `echo "id=[123]" \| snag 'id=[{}]'` &rarr; `123` |
| `<regex>` | raw regex fallback (group 1 or match) | `echo "port=8080" \| snag 'port=(\d+)'` &rarr; `8080` |

## examples

```sh
# extract unique ips from server logs
tail -n 1000 /var/log/nginx/access.log | snag ip | sort -u

# extract commit hashes from git log
git log --oneline | snag hash

# extract json fields from an ndjson stream
docker events --format '{{json .}}' | snag .action

# column extraction without awk syntax
cat /etc/passwd | snag :1

# wildcard capture with literal auto-escaping
kubectl get pods | snag 'pod/{}-'
```

## behavior

- **delimiter splitting**: column mode (`:n`, `/n`, `,n`) performs a literal byte split without csv quote parsing.
- **utf-8 handling**: invalid byte sequences are replaced per line with the unicode replacement character (`u+fffd`) without aborting.
- **exit codes**: returns `0` on clean runs (including 0 matches) and `1` on invalid arguments or uncompilable patterns.

## license

[mit](LICENSE)
