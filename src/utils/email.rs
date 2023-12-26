use regex::Regex;

pub fn is_valid_email(email: &str) -> bool {
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();

    if !email_regex.is_match(email) {
        return false;
    }

    let parts: Vec<&str> = email.split('@').collect();

    if parts.len() != 2 {
        return false;
    }

    let domain = parts[1];

    matches!(domain, "tutanota.com" | "tuta.com")
}
