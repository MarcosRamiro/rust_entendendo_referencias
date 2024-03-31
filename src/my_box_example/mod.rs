// https://rust-br.github.io/rust-book-pt-br/ch15-01-box.html

#[derive(Debug)]
struct Pessoa {
    nome: String,
    idade: i32,
}

struct NovaPessoa {
    nome: String,
}

impl Pessoa {
    fn new(nome: String, idade: i32) -> Self {
        Self {
            nome: nome,
            idade: idade,
        }
    }
    // essa é uma função mutável
    fn atualizar_idade(&mut self, nova_idade: i32) {
        self.idade = nova_idade
    }

    fn dizer_ola(&self, prefixo: &str) {
        println!(
            "{} - Eu sou {}, e tenho {} anos",
            prefixo, self.nome, self.idade
        );
    }
}

trait Saudar {
    fn fazer_saudacao(&self, saudacao: &str);
}

impl Saudar for Pessoa {
    fn fazer_saudacao(&self, saudacao: &str) {
        println!("Pessoa {} Saudando: {}", self.nome, saudacao);
    }
}

impl Saudar for NovaPessoa {
    fn fazer_saudacao(&self, saudacao: &str) {
        println!("Nova Pessoa {} Saudando: {}", self.nome, saudacao);
    }
}

fn saudar(alguem_que_sauda: &Box<impl Saudar>) {
    alguem_que_sauda.fazer_saudacao("Olá mundo!");
}

#[allow(dead_code)]
pub fn run() {
    // **** Entendendo o Box<T> ****
    let mut pessoa = Pessoa::new("Marcos".into(), 34);

    // esse é um método mutável, portanto só pode ser chamado a partir de uma referência mutável
    pessoa.atualizar_idade(35);
    pessoa.dizer_ola("main1");

    let mut box_pessoa = Box::new(pessoa);

    box_pessoa.atualizar_idade(36);

    // esse código não funciona pois "pessoa" já tem um novo dono, que é o box_pessoa
    // println!("Pessoa {:?}", pessoa);

    box_pessoa.dizer_ola("main2");

    usando_box(&mut box_pessoa);

    // nao funciona pq a variavel box_pessoa foi movida para a função usando_box.
    // box_pessoa.dizer_ola("main3");

    saudar(&box_pessoa);

    let box_nova_pessoa = Box::new(NovaPessoa {
        nome: "Maria".into(),
    });

    saudar(&box_nova_pessoa);

    box_pessoa.dizer_ola("main fim");
}

#[allow(dead_code)]
fn usando_box(pessoa: &mut Box<Pessoa>) {
    // Não permite executar um método que faça mutação quando o emprestimo não eh mutável
    // pessoa.atualizar_idade(35);
    pessoa.atualizar_idade(37);
    pessoa.dizer_ola("usando_box");
    mais_uma_funcao_mais_baixo_nivel(pessoa);
    pessoa.dizer_ola("usando_box 2");
}

#[allow(dead_code)]
fn mais_uma_funcao_mais_baixo_nivel(pessoa: &mut Pessoa) {
    pessoa.atualizar_idade(38);
    pessoa.dizer_ola("mais_uma_funcao_mais_baixo_nivel");
}
