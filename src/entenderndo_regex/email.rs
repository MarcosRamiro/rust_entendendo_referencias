use regex::Regex;

pub struct Email(String);

impl Email {
    pub fn new(email: String) -> Result<Self, String> {
        if is_valid_email(&email) {
            Ok(Self(email))
        } else {
            return Err("Email inválido!".to_string());
        }
    }
}

impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn is_valid_email(email: &str) -> bool {
    let re = Regex::new(r"^([a-zA-Z0-9_\-\.]+)@([a-zA-Z0-9_\-\.]+)\.([a-zA-Z]{2,5})$").unwrap();
    re.is_match(email)
}
