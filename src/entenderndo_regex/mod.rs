mod email;

use email::Email;

struct Pessoa {
    nome: String,
    email: Email,
}

impl Pessoa {
    fn new(nome: String, email: String) -> Result<Self, String> {
        let email_validado = Email::new(email)?;

        Ok(Self {
            nome,
            email: email_validado,
        })
    }
}

impl std::fmt::Display for Pessoa {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Pessoa(nome: {}, email: {})", self.nome, self.email)
    }
}

pub fn como_usar_regex() -> () {
    let maria = Pessoa::new("Maria".to_string(), "maria@gmail.com".to_string()).unwrap();
    println!("maria = {}", maria);
    println!("email = {}", maria.email);
}
