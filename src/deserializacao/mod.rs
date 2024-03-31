use serde::{Deserialize, Serialize};
//use serde_json::to_string;

#[derive(Debug, Serialize, Deserialize)]
struct Cachorro<T> {
    nome: String,
    idade: T,
}

impl<T> Cachorro<T> {
    fn new(nome: String, idade: T) -> Self {
        Self { nome, idade }
    }
    fn latir(&self) -> () {
        println!("Chachorro {} está latindo...", self.nome);
    }
}

impl<T> std::fmt::Display for Cachorro<T> 
where T: std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cachorro(nome: {}, idade: {})", self.nome, self.idade)
    }
}


pub fn como_deserealizar() -> Result<(), String> {

    let caramelo: Cachorro<_> = Cachorro::new("Caramelo".to_string(), 5_u8);
    let serialized: String = serde_json::to_string(&caramelo).unwrap();

    println!("serialized = {}", serialized);

    let deserialized: Cachorro<_> = serde_json::from_str::<Cachorro<u8>>(&serialized).unwrap();
    println!("deserialized = {}", deserialized);
    deserialized.latir();

    if deserialized.nome == String::from("Caramelo") {
        return Err("Nome não pode ser caramelo".to_string())
    }

    Ok(())
    
}
