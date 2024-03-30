use std::rc::Rc;

// https://rust-br.github.io/rust-book-pt-br/ch15-04-rc.html

/***
   Rc é uma boa opção para evitar de usar a "&" referencia em todo lugar...
   Usar a referência & tras uma complicação pois precisamos passar as "dicas" de lifetime,
   e issoeu percebi que é um pouco chato e complicado.
*/

#[derive(Debug)]
struct Pessoa {
    nome: String,
    idade: i32,
}

impl Pessoa {
    fn new(nome: String, idade: i32) -> Self {
        Self {
            nome: nome,
            idade: idade,
        }
    }
    
    // essa é uma função mutável
    #[allow(dead_code)]
    fn atualizar_idade(&mut self, nova_idade: i32) {
        self.idade = nova_idade
    }

    fn dizer_ola(&self) {
        println!("Eu sou {}, e tenho {} anos", self.nome, self.idade);
    }
}

#[allow(dead_code)]
pub fn run() {
    // **** Entendendo o Rc<T> ****
    let marcos = Pessoa::new("Marcos".into(), 34);
    let compartilhado = Rc::new(marcos);
    println!("Pessoa: {:?}", compartilhado);
    compartilhado.dizer_ola();
    // isso não é permitido, pois Rc<T> é imutável
    //comp.atualizar_idade(35);
    usando_rc(Rc::clone(&compartilhado));
    println!("Fim...");
}

/***
   Rc: tudo dentro dele é imutável, não pode ser alterado
   e também não pode chamar um método que seja "&mut self", ou seja que altera o valor da struct
*/
#[allow(dead_code)]
fn usando_rc(pessoa: Rc<Pessoa>) {
    pessoa.dizer_ola();
    // Não permite executar um método que
    // pessoa.atualizar_idade(35);
    println!(
        "usando_rc:: numero de copias: {}",
        Rc::strong_count(&pessoa)
    );
    mais_uma_funcao_mais_baixo_nivel(Rc::clone(&pessoa));
}

#[allow(dead_code)]
fn mais_uma_funcao_mais_baixo_nivel(pessoa: Rc<Pessoa>) {
    println!(
        "mais_uma_funcao_mais_baixo_nivel:: numero de copias: {}",
        Rc::strong_count(&pessoa)
    );
    pessoa.dizer_ola();
}
