#[allow(unused_imports)]
mod deserializacao;
mod dispatch_dynamic;
mod entenderndo_regex;
mod funcional;
mod iteradores;
mod jogo_com_rc;
mod minha_senha;
mod my_box_example;
mod my_rc_example;
mod tratamento_erro;

use dispatch_dynamic::entendendo_traits;
use funcional::entendendo_funcoes_de_alta_ordem;
use iteradores::entendendo_iteradores;
use jogo_com_rc::jogar;
use my_box_example::run;

fn main() {
    //run();
    // jogar();
    //principal();
    // entendendo_traits();
    // entendendo_iteradores();
    // entendendo_funcoes_de_alta_ordem();
    // deserializacao::como_deserealizar().expect("Deu ruim");

    // entenderndo_regex::como_usar_regex();
    let resultado = tratamento_erro::como_tratar_erros();

    if resultado.is_err() {
        println!("Erro no main: {}", resultado.err().unwrap())
    }
}
