#[derive(Debug)]
struct Cachorro {
    nome: String,
    idade: u32,
}

impl Cachorro {
    #[allow(dead_code)]
    fn new(nome: String, idade: u32) -> Self {
        Self { nome, idade }
    }
    #[allow(dead_code)]
    fn get_nome(&self) -> &str {
        self.nome.as_str()
    }
    #[allow(dead_code)]
    fn get_idade(&self) -> u32 {
        self.idade
    }
}
    
#[allow(dead_code)]
pub fn entendendo_iteradores() -> () {
    let dog = Cachorro::new("Fido".to_string(), 5);
    let dog1 = Cachorro::new("Duque".to_string(), 10);
    let dog2 = Cachorro::new("Max".to_string(), 1);

    let mut dogs = vec![dog, dog1, dog2];

    dogs.push(Cachorro::new("Amarelo".to_string(), 2));

    let new_iter = dogs
        .iter()
        .filter(|c: &&Cachorro| c.get_idade() > 2)
        .map(|cachorro: &Cachorro| (cachorro.get_nome(), cachorro.idade));

    for dog in new_iter {
        println!("{} {}", dog.0, dog.1);
    }

    let idade_total = dogs
        .iter()
        .fold(0_u32, |acumulador: u32, cachorro: &Cachorro| {
            calcula_idade(acumulador, cachorro, acumular_idade)
        });

    println!("A idade total dos cachorros é: {} anos", idade_total);

    println!("Cachorros: {:?}", dogs);
}

#[allow(dead_code)]
fn acumular_idade(acumulador: u32, cachorro: &Cachorro) -> u32 {
    acumulador + cachorro.idade
}

#[allow(dead_code)]
fn calcula_idade(acumulador: u32, cachorro: &Cachorro, funcao: fn(u32, &Cachorro) -> u32) -> u32 {
    funcao(acumulador, cachorro)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acumular_simples() {
        let max = Box::new(Cachorro::new("Max".to_string(), 2));
        let max2 = Box::new(Cachorro::new("Max".to_string(), 3));

        let mut acumulador = acumular_idade(0, &max);
        acumulador = acumular_idade(acumulador, &max2);

        assert_eq!(5, acumulador);
    }

    #[test]
    fn test_acumular_idade() {
        let cachorro1 = Cachorro::new("Fido".to_string(), 5);
        let cachorro2 = Cachorro::new("Duque".to_string(), 10);
        let cachorro3 = Cachorro::new("Max".to_string(), 1);

        let mut cachorros = vec![cachorro1, cachorro2, cachorro3];

        cachorros.push(Cachorro::new("Amarelo".to_string(), 2));

        let resultado_esperado = 18_u32;
        let resultado = cachorros.iter().fold(0_u32, acumular_idade);

        assert_eq!(resultado, resultado_esperado);
    }
}
