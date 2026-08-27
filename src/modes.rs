use regex::Regex;
use serde_json::Value;
use std::io::{BufRead, Write};

use crate::builtins;

#[derive(Debug)]
pub enum Mode {
    Builtin(Regex),
    Column { delimiter: u8, index: usize },
    Template(Regex),
    Json(String),
    RawRegex(Regex),
}

pub fn parse_target(target: &str) -> Result<Mode, String> {
    if let Some(re) = builtins::builtin_regex(target) {
        return Ok(Mode::Builtin(re));
    }

    if let Some(key) = target.strip_prefix('.') {
        if !key.is_empty() {
            return Ok(Mode::Json(key.to_string()));
        }
    }

    if target.len() >= 2 {
        let first = target.as_bytes()[0];
        if matches!(first, b':' | b'/' | b',') {
            let rest = &target[1..];
            if let Ok(n) = rest.parse::<usize>() {
                if n > 0 {
                    return Ok(Mode::Column {
                        delimiter: first,
                        index: n,
                    });
                }
            }
        }
    }

    if target.contains("{}") {
        let parts: Vec<&str> = target.splitn(2, "{}").collect();
        let prefix = regex::escape(parts[0]);
        let suffix_raw = if parts.len() > 1 { parts[1] } else { "" };
        let suffix = regex::escape(suffix_raw).replace(r"\{\}", "(.*?)");

        let pattern = format!("{}(.*?){}", prefix, suffix);
        let re = Regex::new(&pattern)
            .map_err(|e| format!("snag: invalid pattern: {}: {}", target, e))?;
        return Ok(Mode::Template(re));
    }

    let re = Regex::new(target).map_err(|e| format!("snag: invalid pattern: {}: {}", target, e))?;
    Ok(Mode::RawRegex(re))
}

pub fn run_lossy<R: std::io::Read, W: Write>(
    mode: &Mode,
    mut reader: std::io::BufReader<R>,
    writer: &mut W,
) -> std::io::Result<()> {
    let mut buf = Vec::new();
    loop {
        buf.clear();
        let bytes_read = reader.read_until(b'\n', &mut buf)?;
        if bytes_read == 0 {
            break;
        }

        let content = if buf.ends_with(b"\r\n") {
            &buf[..buf.len() - 2]
        } else if buf.ends_with(b"\n") {
            &buf[..buf.len() - 1]
        } else {
            &buf[..]
        };

        let line = String::from_utf8_lossy(content);

        match mode {
            Mode::Builtin(re) | Mode::RawRegex(re) => {
                write_regex_matches(re, &line, writer)?;
            }
            Mode::Template(re) => {
                write_regex_matches(re, &line, writer)?;
            }
            Mode::Column { delimiter, index } => {
                write_column_match(&line, *delimiter, *index, writer)?;
            }
            Mode::Json(key) => {
                write_json_match(&line, key, writer)?;
            }
        }
    }
    Ok(())
}

fn write_regex_matches(re: &Regex, line: &str, writer: &mut dyn Write) -> std::io::Result<()> {
    let has_captures = re.captures_len() > 1;

    if has_captures {
        for caps in re.captures_iter(line) {
            if let Some(m) = caps.get(1) {
                writeln!(writer, "{}", m.as_str())?;
            }
        }
    } else {
        for m in re.find_iter(line) {
            writeln!(writer, "{}", m.as_str())?;
        }
    }
    Ok(())
}

fn write_column_match(
    line: &str,
    delimiter: u8,
    index: usize,
    writer: &mut dyn Write,
) -> std::io::Result<()> {
    let delim = delimiter as char;
    if let Some(field) = line.split(delim).nth(index - 1) {
        writeln!(writer, "{}", field)?;
    }
    Ok(())
}

fn write_json_match(line: &str, key: &str, writer: &mut dyn Write) -> std::io::Result<()> {
    let parsed: Value = match serde_json::from_str(line) {
        Ok(v) => v,
        Err(_) => return Ok(()),
    };

    let val = match parsed.get(key) {
        Some(v) => v,
        None => return Ok(()),
    };

    match val {
        Value::Null => {}
        Value::String(s) => {
            writeln!(writer, "{}", s)?;
        }
        Value::Number(n) => {
            writeln!(writer, "{}", n)?;
        }
        Value::Bool(b) => {
            writeln!(writer, "{}", b)?;
        }
        other => {
            writeln!(writer, "{}", other)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn extract(target: &str, input: &str) -> String {
        let mode = parse_target(target).expect("parse_target failed");
        let reader = std::io::BufReader::new(input.as_bytes());
        let mut output = Vec::new();
        run_lossy(&mode, reader, &mut output).expect("run_lossy failed");
        String::from_utf8(output).expect("output not utf8")
    }

    #[test]
    fn builtin_ip_extracts() {
        assert_eq!(extract("ip", "addr 10.0.0.1 end\n"), "10.0.0.1\n");
    }

    #[test]
    fn builtin_url_extracts() {
        assert_eq!(
            extract("url", "see https://example.com\n"),
            "https://example.com\n"
        );
    }

    #[test]
    fn column_colon() {
        assert_eq!(extract(":2", "a:b:c\n"), "b\n");
    }

    #[test]
    fn column_out_of_range() {
        assert_eq!(extract(":5", "a:b:c\n"), "");
    }

    #[test]
    fn column_no_delimiter_field_one() {
        assert_eq!(extract(":1", "hello world\n"), "hello world\n");
    }

    #[test]
    fn template_simple() {
        assert_eq!(extract("hello {} world", "hello foo world\n"), "foo\n");
    }

    #[test]
    fn template_with_metachar() {
        assert_eq!(extract("port=8080.{}$", "port=8080.data$ end\n"), "data\n");
    }

    #[test]
    fn json_string_value() {
        assert_eq!(extract(".name", r#"{"name":"alice","age":30}"#), "alice\n");
    }

    #[test]
    fn json_number_value() {
        assert_eq!(extract(".age", r#"{"name":"alice","age":30}"#), "30\n");
    }

    #[test]
    fn json_null_skipped() {
        assert_eq!(extract(".x", r#"{"x":null}"#), "");
    }

    #[test]
    fn json_missing_key_skipped() {
        assert_eq!(extract(".x", r#"{"y":1}"#), "");
    }

    #[test]
    fn json_malformed_skipped() {
        assert_eq!(extract(".x", "not json\n"), "");
    }

    #[test]
    fn json_nested_object() {
        assert_eq!(
            extract(".data", r#"{"data":{"a":1,"b":2}}"#),
            "{\"a\":1,\"b\":2}\n"
        );
    }

    #[test]
    fn regex_no_groups() {
        assert_eq!(extract(r"\d+", "abc 42 def 7\n"), "42\n7\n");
    }

    #[test]
    fn regex_with_group() {
        assert_eq!(extract(r"id=(\d+)", "id=123 id=456\n"), "123\n456\n");
    }

    #[test]
    fn regex_invalid_pattern() {
        let result = parse_target(r"(unbalanced");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("invalid pattern"));
    }

    #[test]
    fn empty_input() {
        assert_eq!(extract("ip", ""), "");
    }

    #[test]
    fn multiple_matches_per_line() {
        assert_eq!(
            extract("ip", "10.0.0.1 and 192.168.1.1\n"),
            "10.0.0.1\n192.168.1.1\n"
        );
    }
}
