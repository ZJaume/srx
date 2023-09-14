use fancy_regex::{self, Regex};

pub fn full_regex<S: AsRef<str>>(re: S) -> Result<Regex, fancy_regex::Error> {
    let pattern = format!("^{}$", re.as_ref());

    Regex::new(&pattern)
}
