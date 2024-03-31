use serde::{Deserialize, Serialize};
//use serde_json::to_string;

#[derive(Serialize, Deserialize)]
struct Cachorro<T> {
    nome: String,
    idade: T,
    tipo: TipoCachorro,
}

#[derive(Serialize, Deserialize)]
enum TipoCachorro {
    ViraLata,
    DeRaca,
}

impl std::fmt::Display for TipoCachorro {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TipoCachorro::DeRaca => write!(f, "De Raça"),
            TipoCachorro::ViraLata => write!(f, "Vira Latas"),
        }
    }
}

impl<T> Cachorro<T> {
    fn new(nome: String, idade: T, tipo: TipoCachorro) -> Self {
        Self { nome, idade, tipo }
    }
    fn latir(&self) -> () {
        println!("Cachorro {} está latindo...", self.nome);
    }
}

impl<T> std::fmt::Display for Cachorro<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Cachorro(nome: {}, idade: {}, tipo: {})",
            self.nome, self.idade, self.tipo
        )
    }
}

pub fn como_deserealizar() -> Result<(), String> {
    let caramelo: Cachorro<_> = Cachorro::new("Caramelo".to_string(), 5_u8, TipoCachorro::ViraLata);
    let serialized: String = serde_json::to_string(&caramelo).unwrap();

    println!("serialized = {}", serialized);

    let deserialized: Cachorro<_> = serde_json::from_str::<Cachorro<u8>>(&serialized).unwrap();
    println!("deserialized = {}", deserialized);
    deserialized.latir();

    Ok(())
}
