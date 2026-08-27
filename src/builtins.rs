use regex::Regex;

pub fn builtin_regex(name: &str) -> Option<Regex> {
    let pattern = match name {
        "ip" => concat!(
            r"(?:",
            r"\b(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\b",
            r"|",
            r"(?:[0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}",
            r"|(?:[0-9a-fA-F]{1,4}:){1,7}:",
            r"|(?:[0-9a-fA-F]{1,4}:){1,6}:[0-9a-fA-F]{1,4}",
            r"|(?:[0-9a-fA-F]{1,4}:){1,5}(?::[0-9a-fA-F]{1,4}){1,2}",
            r"|(?:[0-9a-fA-F]{1,4}:){1,4}(?::[0-9a-fA-F]{1,4}){1,3}",
            r"|(?:[0-9a-fA-F]{1,4}:){1,3}(?::[0-9a-fA-F]{1,4}){1,4}",
            r"|(?:[0-9a-fA-F]{1,4}:){1,2}(?::[0-9a-fA-F]{1,4}){1,5}",
            r"|[0-9a-fA-F]{1,4}:(?::[0-9a-fA-F]{1,4}){1,6}",
            r"|::(?:[0-9a-fA-F]{1,4}:){0,5}[0-9a-fA-F]{1,4}",
            r"|::(?:[fF]{4}:)?(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)",
            r"|::",
            r")",
        ),
        "url" => r"https?://[^\s]+",
        "num" => r"\b[0-9]+\b",
        "hash" => r"\b[0-9a-fA-F]{7,64}\b",
        "path" => r"/(?:[a-zA-Z0-9._-]+/)*[a-zA-Z0-9._-]+",
        "email" => r"\b[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}\b",
        _ => return None,
    };

    Some(Regex::new(pattern).expect("invalid builtin pattern"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_builtins_compile() {
        for name in &["ip", "url", "num", "hash", "path", "email"] {
            assert!(
                builtin_regex(name).is_some(),
                "builtin '{}' should compile",
                name
            );
        }
    }

    #[test]
    fn unknown_returns_none() {
        assert!(builtin_regex("unknown").is_none());
    }

    #[test]
    fn ip_matches_ipv4() {
        let re = builtin_regex("ip").unwrap();
        let m: Vec<&str> = re
            .find_iter("addr 192.168.1.1 end")
            .map(|m| m.as_str())
            .collect();
        assert_eq!(m, vec!["192.168.1.1"]);
    }

    #[test]
    fn url_matches_https() {
        let re = builtin_regex("url").unwrap();
        let m: Vec<&str> = re
            .find_iter("visit https://example.com/path end")
            .map(|m| m.as_str())
            .collect();
        assert_eq!(m, vec!["https://example.com/path"]);
    }

    #[test]
    fn num_matches_integers() {
        let re = builtin_regex("num").unwrap();
        let m: Vec<&str> = re
            .find_iter("port 8080 and 443")
            .map(|m| m.as_str())
            .collect();
        assert_eq!(m, vec!["8080", "443"]);
    }

    #[test]
    fn hash_matches_sha() {
        let re = builtin_regex("hash").unwrap();
        let m: Vec<&str> = re
            .find_iter("commit abc1234 done")
            .map(|m| m.as_str())
            .collect();
        assert_eq!(m, vec!["abc1234"]);
    }

    #[test]
    fn path_matches_absolute() {
        let re = builtin_regex("path").unwrap();
        let m: Vec<&str> = re
            .find_iter("file at /usr/local/bin end")
            .map(|m| m.as_str())
            .collect();
        assert_eq!(m, vec!["/usr/local/bin"]);
    }

    #[test]
    fn email_matches() {
        let re = builtin_regex("email").unwrap();
        let m: Vec<&str> = re
            .find_iter("contact user@example.com today")
            .map(|m| m.as_str())
            .collect();
        assert_eq!(m, vec!["user@example.com"]);
    }
}
