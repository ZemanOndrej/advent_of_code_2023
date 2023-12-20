pub fn split<'a>(str: &'a str, pat: &str) -> Vec<&'a str> {
    str.split(pat)
        .map(|v| v.trim())
        .filter(|s| !s.is_empty())
        .collect()
}
