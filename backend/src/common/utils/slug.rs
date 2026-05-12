use once_cell::sync::Lazy;
use regex::Regex;

static SLUG_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^a-z0-9]+").unwrap());

pub fn slugify(s: &str) -> String {
    let mut slug = s.to_lowercase();

    slug = SLUG_REGEX
        .replace_all(&slug, "-")
        .trim_matches('-')
        .to_string();

    if slug.is_empty() {
        slug = "org".to_string()
    }

    slug
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_slugify() {
        assert_eq!(slugify("Hello World"), "hello-world");
        assert_eq!(slugify("My Company Ltd."), "my-company-ltd");
        assert_eq!(slugify("   ABC  Nigeria  "), "abc-nigeria");
    }
}
