mod builtins;
mod modes;

use std::io::{self, BufReader, BufWriter, IsTerminal, Write};
use std::process;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn use_color(stream: &impl IsTerminal) -> bool {
    stream.is_terminal()
        && std::env::var_os("NO_COLOR").is_none()
        && std::env::var("TERM").map_or(true, |t| t != "dumb")
}

fn print_help() {
    let color = use_color(&io::stdout());
    if color {
        println!(
            "\x1b[1;38;2;166;227;161msnag\x1b[0m \x1b[38;2;147;153;178m{}\x1b[0m
\x1b[38;2;186;194;222mextract patterns from text streams\x1b[0m

\x1b[1;38;2;137;180;250musage:\x1b[0m
  \x1b[38;2;249;226;175msnag\x1b[0m \x1b[38;2;203;166;247m<target>\x1b[0m
  <stream> \x1b[38;2;108;112;134m|\x1b[0m \x1b[38;2;249;226;175msnag\x1b[0m \x1b[38;2;203;166;247m<target>\x1b[0m

\x1b[1;38;2;137;180;250msemantic targets:\x1b[0m
  \x1b[1;38;2;203;166;247mip\x1b[0m                    \x1b[38;2;186;194;222mipv4 and ipv6 addresses\x1b[0m
  \x1b[1;38;2;203;166;247murl\x1b[0m                   \x1b[38;2;186;194;222mhttp:// and https:// urls\x1b[0m
  \x1b[1;38;2;203;166;247mnum\x1b[0m                   \x1b[38;2;186;194;222mstandalone integers\x1b[0m
  \x1b[1;38;2;203;166;247mhash\x1b[0m                  \x1b[38;2;186;194;222mhex strings 7–64 chars (sha, md5)\x1b[0m
  \x1b[1;38;2;203;166;247mpath\x1b[0m                  \x1b[38;2;186;194;222mabsolute unix file paths\x1b[0m
  \x1b[1;38;2;203;166;247memail\x1b[0m                 \x1b[38;2;186;194;222memail addresses\x1b[0m

\x1b[1;38;2;137;180;250mstructured targets:\x1b[0m
  \x1b[1;38;2;203;166;247m.<key>\x1b[0m                \x1b[38;2;186;194;222mtop-level json field from ndjson\x1b[0m
  \x1b[1;38;2;203;166;247m:<n>\x1b[0m, \x1b[1;38;2;203;166;247m/<n>\x1b[0m, \x1b[1;38;2;203;166;247m,<n>\x1b[0m         \x1b[38;2;186;194;222m1-indexed field split by ':', '/', or ','\x1b[0m
  \x1b[1;38;2;203;166;247m<pre>{{}}<post>\x1b[0m         \x1b[38;2;186;194;222mliteral template with wildcard capture\x1b[0m
  \x1b[1;38;2;203;166;247m<regex>\x1b[0m               \x1b[38;2;186;194;222mraw regex (captures group 1 or full match)\x1b[0m

\x1b[1;38;2;137;180;250moptions:\x1b[0m
  \x1b[38;2;148;226;213m-h\x1b[0m, \x1b[38;2;148;226;213m--help\x1b[0m            \x1b[38;2;186;194;222mprint help\x1b[0m
  \x1b[38;2;148;226;213m-V\x1b[0m, \x1b[38;2;148;226;213m--version\x1b[0m         \x1b[38;2;186;194;222mprint version\x1b[0m",
            VERSION
        );
    } else {
        println!(
            "snag {}
extract patterns from text streams

usage:
  snag <target>
  <stream> | snag <target>

semantic targets:
  ip                    ipv4 and ipv6 addresses
  url                   http:// and https:// urls
  num                   standalone integers
  hash                  hex strings 7–64 chars (sha, md5)
  path                  absolute unix file paths
  email                 email addresses

structured targets:
  .<key>                top-level json field from ndjson
  :<n>, /<n>, ,<n>         1-indexed field split by ':', '/', or ','
  <pre>{{}}<post>         literal template with wildcard capture
  <regex>               raw regex (captures group 1 or full match)

options:
  -h, --help            print help
  -V, --version         print version",
            VERSION
        );
    }
}

fn print_version() {
    let color = use_color(&io::stdout());
    if color {
        println!(
            "\x1b[1;38;2;166;227;161msnag\x1b[0m \x1b[38;2;147;153;178m{}\x1b[0m",
            VERSION
        );
    } else {
        println!("snag {}", VERSION);
    }
}

fn main() {
    let code = run();
    process::exit(code);
}

fn run() -> i32 {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.iter().any(|a| a == "--help" || a == "-h") {
        print_help();
        return 0;
    }

    if args.iter().any(|a| a == "--version" || a == "-V") {
        print_version();
        return 0;
    }

    if args.is_empty() {
        eprintln!("snag: missing target argument\n\nFor more information try --help");
        return 1;
    }

    let target = &args[0];

    let mode = match modes::parse_target(target) {
        Ok(m) => m,
        Err(msg) => {
            eprintln!("{}", msg);
            return 1;
        }
    };

    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    match modes::run_lossy(&mode, reader, &mut writer) {
        Ok(()) => {}
        Err(e) => {
            if e.kind() == io::ErrorKind::BrokenPipe {
                return 0;
            }
            eprintln!("snag: {}", e);
            return 1;
        }
    }

    match writer.flush() {
        Ok(()) => 0,
        Err(e) => {
            if e.kind() == io::ErrorKind::BrokenPipe {
                return 0;
            }
            eprintln!("snag: {}", e);
            1
        }
    }
}
