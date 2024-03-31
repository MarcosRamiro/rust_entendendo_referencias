use rpassword::prompt_password;

fn obter_senha_do_prompt() -> String {
    let password = prompt_password("Digite sua senha: ").unwrap();
    println!("Your password is {}", password);
    password
}

pub fn obter_senha() -> () {
    obter_senha_do_prompt();
}
