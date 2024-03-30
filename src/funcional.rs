#[derive(Debug)]
struct Cachorro {
    nome: String,
    idade: u32,
}

impl Cachorro {
    fn new(nome: String, idade: u32) -> Self {
        Self { nome, idade }
    }

    fn get_nome(&self) -> &str {
        self.nome.as_str()
    }

    fn get_idade(&self) -> u32 {
        self.idade
    }

    fn set_idade(&mut self, idade: u32) {
        self.idade = idade;
    }
}

impl Default for Cachorro {
    fn default() -> Self {
        Self {
            nome: String::from("Caramelo"),
            idade: 2,
        }
    }
}

fn preparar_cachorros<F>(mut cachorros: Vec<Cachorro>, funcao: F)
where
    F: Fn(Cachorro) -> (),
{
    let cachorro: Option<Cachorro> = cachorros.pop();

    if let Some(cachorro) = cachorro {
        funcao(cachorro);
    } else {
        println!("Não achei o cachorro esperado.");
    }

    println!("Cachorros: {:?}", cachorros);

    let cachorro: Option<Cachorro> = cachorros.pop();

    if let Some(cachorro) = cachorro {
        funcao(cachorro);
    } else {
        println!("Não achei o cachorro esperado.");
    }

    println!("Cachorros: {:?}", cachorros);
}

//fn pegar_cachorro(mut cachorro: Cachorro) -> () {
//    println!("peguei o cachorro {} {}", cachorro.get_nome(), cachorro.get_idade());
//}

pub fn entendendo_funcoes_de_alta_ordem() -> () {
    let cachorro1 = Cachorro::new("Fido".to_string(), 5);
    let cachorro2 = Cachorro::new("Amarelo".to_string(), 5);
    let cachorro3 = Cachorro::new("Duque".to_string(), 5);

    let cachorros = vec![cachorro1, cachorro2, cachorro3];

    //preparar_cachorros(cachorros, pegar_cachorro);
    let my_funcao = criar_funcao("peguei o cachorro da Lambda".to_string());

    preparar_cachorros(cachorros, my_funcao);

    let soma_1 = soma(1);
    let resultado_soma = soma_1(5);
    println!("Resultado: {}", resultado_soma);

    let resultado_soma = soma_1(55);
    println!("Resultado: {}", resultado_soma);

    let cachorro_padrao: Cachorro = Cachorro::default();
    let mut fn_cachorro_padrao = criar_funcao_cachorro_padrao(cachorro_padrao);
    let cachorro1: Cachorro = Cachorro::new("Fido".to_string(), 15);
    let cachorro2: Cachorro = Cachorro::new("Amarelo".to_string(), 4);
    let cachorro3: Cachorro = Cachorro::new("Duque".to_string(), 10);

    println!(
        "Soma idade dos cachorros: {}",
        obter_soma_idade_cachorro(&cachorro1, &mut fn_cachorro_padrao)
    );
    println!(
        "Soma idade dos cachorros: {}",
        obter_soma_idade_cachorro(&cachorro2, &mut fn_cachorro_padrao)
    );
    println!(
        "Soma idade dos cachorros: {}",
        obter_soma_idade_cachorro(&cachorro3, &mut fn_cachorro_padrao)
    );

    println!("Fim");
}

fn obter_soma_idade_cachorro<F>(cachorro: &Cachorro, funcao: &mut F) -> u32
where
    F: FnMut(&Cachorro) -> u32,
{
    funcao(cachorro)
}

fn criar_funcao(msg: String) -> impl Fn(Cachorro) -> () {
    
    let retorno = move |cachorro: Cachorro| {
        println!("{msg} {} {}", cachorro.get_nome(), cachorro.get_idade());
    };

    // msg foi movido para a função retorno
    //println!("Isso nao deve funcionar {}", msg);
    
    retorno
}

fn soma(a: i32) -> impl Fn(i32) -> i32 {
    move |b: i32| a + b
}

fn criar_funcao_cachorro_padrao(mut cachorro: Cachorro) -> impl FnMut(&Cachorro) -> u32 {
    let retorno = move |outro_cachorro: &Cachorro| {
        cachorro.set_idade(cachorro.get_idade() + 100);
        cachorro.get_idade() + outro_cachorro.get_idade()
    };

    // não funciona pq a propriedade foi passada para a função acima
    //println!("Isso não deve funcionar {:?}", cachorro);

    retorno
}
