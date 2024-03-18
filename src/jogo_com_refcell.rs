/***
OWNER:
    Rc<T> permite múltiplos possuidores
    Já Box<T> e RefCell<T> têm possuidores únicos

BORROW:
    Box<T> permite empréstimos imutáveis ou mutáveis checados em tempo de compilação
    Rc<T> permite apenas empréstimos imutáveis em tempo de compilação
    RefCell<T> permite empréstimos imutáveis ou mutáveis checados em tempo de execução.
    Como o RefCell<T> permite empréstimos mutáveis checados em tempo de execução, nós podemos
        modificar o valor dentro de um RefCell<T> mesmo quando o RefCell<T> é imutável.


 */
use std::collections::{BTreeSet, HashMap, HashSet};
use std::rc::Rc;

#[derive(Debug)]
struct Pessoa {
    nome: String,
}

impl Pessoa {
    fn set_nome(&mut self, novo_nome: String) {
        self.nome = novo_nome;
    }
}

pub fn principal() {
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    let res = set.insert(3);

    assert_eq!(res, false);
    assert_eq!(set.len(), 3);

    let box_pessoa: Box<Pessoa> = Box::new(Pessoa {
        nome: "Maria".to_string(),
    });
    print_pessoa(&box_pessoa);
}

fn print_pessoa(pessoa: &Pessoa) {
    println!("{:?}", pessoa)
}

fn teste_box(pessoa: &mut Box<Pessoa>, novo_nome: String) {
    pessoa.set_nome(novo_nome);
}

/***
essa função nao compila pois não é permito chamar um
 */
// fn teste_rc_box(pessoa: Rc<Box<Pessoa>>, novo_nome: String) {
//     pessoa.set_nome(novo_nome);
// }
