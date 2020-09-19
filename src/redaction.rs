use anyhow::{Context, Result};
use std::io::Write;

const REDACT_TOKEN: &str = "█████";

pub fn redact<T>(keywords: &[T], target: &str) -> Result<String>
where
    T: AsRef<str>,
{
    let pattern = keywords.iter().map(|x| x.as_ref()).collect::<Vec<_>>();
    let ac = aho_corasick::AhoCorasickBuilder::new()
        .ascii_case_insensitive(true)
        .build(pattern);
    let mut redacted = Vec::new();
    ac.stream_replace_all_with(target.as_bytes(), &mut redacted, |_, _, w| {
        w.write_all(REDACT_TOKEN.as_bytes())
    })
    .with_context(|| "Failed to replace string")?;
    let redacted = String::from_utf8_lossy(&redacted);
    Ok((*redacted).to_owned())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_redact() {
        let keywords = &["hello", "world", "lorem ipsum"];
        let result = redact(keywords, "Hello world! abcde lorem ipsum").unwrap();

        assert_eq!("█████ █████! abcde █████", result);
    }

    #[test]
    fn test_redact_replace_all_occurances() {
        let keywords = &["abc"];
        let result = redact(keywords, "abcabcabcabcabc").unwrap();

        assert_eq!("█████████████████████████", result);
    }
}
