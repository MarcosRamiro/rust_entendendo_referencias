use std::cell::RefCell;
use std::io::stdin;
use std::rc::Rc;

const MSG_JOGADA_DUPLICADA: &str = "Jogada duplicada.";

#[derive(Debug)]
struct Jogador {
    nome: String,
    eh_vencedor: bool,
}

impl PartialEq for Jogador {
    fn eq(&self, other: &Self) -> bool {
        self.get_nome().eq(other.get_nome())
    }
}

impl Jogador {
    #[allow(dead_code)]
    fn maria() -> Self {
        Self::new("Maria".to_string(), false)
    }
    #[allow(dead_code)]
    fn joao() -> Self {
        Self::new("Joao".to_string(), false)
    }

    fn get_nome(&self) -> &str {
        self.nome.as_str()
    }

    fn new(nome: String, eh_vencedor: bool) -> Self {
        Self { nome, eh_vencedor }
    }

    fn tornar_vencedor(vencedor: &Self) -> Self {
        Self::new(vencedor.get_nome().to_string(), true)
    }
}

#[derive(Debug, Clone)]
struct Jogada {
    posicao: i32,
    jogador: Rc<Jogador>,
}

impl Jogada {
    fn new(posicao: i32, jogador: Rc<Jogador>) -> Self {
        Self { posicao, jogador }
    }

    fn get_posicao(&self) -> i32 {
        self.posicao
    }
}

impl PartialEq for Jogada {
    fn eq(&self, other: &Self) -> bool {
        self.posicao == other.posicao && self.jogador == other.jogador
    }
}

#[derive(PartialEq, Debug, Clone)]
enum EstadoJogo {
    Aberto,
    Empate,
    Finalizado(Rc<Jogador>),
}

struct Jogo {
    estado_jogo: RefCell<EstadoJogo>,
    jogadas: RefCell<Vec<Jogada>>,
}

trait FazerJogada {
    fn jogar(&self, posicao: i32, jogador: Rc<Jogador>) -> Result<EstadoJogo, String>;
}

impl FazerJogada for Jogo {
    fn jogar(&self, posicao: i32, jogador: Rc<Jogador>) -> Result<EstadoJogo, String> {
        if self.obter_estado_atual() != EstadoJogo::Aberto {
            return Err("Estado inválido.".to_string());
        }

        let jogada = Jogada::new(posicao, Rc::clone(&jogador));

        self.adicionar_jogada(jogada)?;

        if let Some(novo_estado_jogo) = self.is_jogada_vencedora(Rc::clone(&jogador)) {
            self.atribuir_novo_estado(novo_estado_jogo?);
            return Ok(self.obter_estado_atual());
        }

        if !self.tem_jodada_disponivel() {
            self.atribuir_novo_estado(EstadoJogo::Empate);
            return Ok(self.obter_estado_atual());
        }

        Ok(EstadoJogo::Aberto)
    }
}

impl Jogo {
    fn obter_estado_atual(&self) -> EstadoJogo {
        self.estado_jogo.borrow().clone()
    }

    fn atribuir_novo_estado(&self, novo_estado: EstadoJogo) {
        *self.estado_jogo.borrow_mut() = novo_estado;
    }
    
    fn new() -> Jogo {
        Jogo {
            estado_jogo: RefCell::new(EstadoJogo::Aberto),
            jogadas: RefCell::new(Vec::new()),
        }
    }

    fn tem_jodada_disponivel(&self) -> bool {
        !(self.jogadas.borrow().len() == 9)
    }

    fn is_jogada_vencedora(&self, jogador: Rc<Jogador>) -> Option<Result<EstadoJogo, String>> {
        fn is_venceu_jogo(
            jogadas: &Vec<Jogada>,
            posicao_1: &Jogada,
            posicao_2: &Jogada,
            posicao_3: &Jogada,
        ) -> bool {
            jogadas.contains(&posicao_1)
                && jogadas.contains(&posicao_2)
                && jogadas.contains(&posicao_3)
        }

        for (posicao_1, posicao_2, posicao_3) in get_jogadas_vencedoras(Rc::clone(&jogador)) {
            if is_venceu_jogo(&self.jogadas.borrow(), &posicao_1, &posicao_2, &posicao_3) {
                let vencedor: Rc<Jogador> = Rc::new(Jogador::tornar_vencedor(&jogador));
                return Some(Ok(EstadoJogo::Finalizado(vencedor)));
            }
        }
        None
    }

    fn adicionar_jogada(&self, jogada: Jogada) -> Result<(), String> {
        let is_duplicada: bool = {
            self.jogadas
                .borrow()
                .iter()
                .any(|jogada_| jogada_.get_posicao() == jogada.get_posicao())
        };

        if is_duplicada {
            return Err(MSG_JOGADA_DUPLICADA.to_string());
        }

        self.jogadas.borrow_mut().push(jogada);

        Ok(())
    }
}

fn get_jogadas_vencedoras(jogador: Rc<Jogador>) -> Vec<(Jogada, Jogada, Jogada)> {
    fn construir_tupla(
        n1: i32,
        n2: i32,
        n3: i32,
        jogador: Rc<Jogador>,
    ) -> (Jogada, Jogada, Jogada) {
        let criar_jogada = |posicao: i32| -> Jogada { Jogada::new(posicao, Rc::clone(&jogador)) };

        (criar_jogada(n1), criar_jogada(n2), criar_jogada(n3))
    }

    let mut retorno = Vec::new();
    retorno.push(construir_tupla(1, 2, 3, Rc::clone(&jogador)));
    retorno.push(construir_tupla(4, 5, 6, Rc::clone(&jogador)));
    retorno.push(construir_tupla(7, 8, 9, Rc::clone(&jogador)));
    retorno.push(construir_tupla(1, 4, 7, Rc::clone(&jogador)));
    retorno.push(construir_tupla(2, 5, 8, Rc::clone(&jogador)));
    retorno.push(construir_tupla(3, 6, 9, Rc::clone(&jogador)));
    retorno.push(construir_tupla(1, 5, 9, Rc::clone(&jogador)));
    retorno.push(construir_tupla(7, 5, 3, Rc::clone(&jogador)));

    retorno
}


#[allow(dead_code)]
pub fn jogar() {
    let is_posicao_invalida = |posicao: i32| -> bool { posicao < 1 || posicao > 9 };

    let maria = Rc::new(Jogador::maria());
    let joao = Rc::new(Jogador::joao());
    let mut jogador_atual = Rc::clone(&maria);
    let jogo: Jogo = Jogo::new();

    let mut buffer = String::new();

    loop {
        buffer.clear();

        println!("Olá {}, informe a sua jogada: ", jogador_atual.get_nome());

        stdin()
            .read_line(&mut buffer)
            .expect("Não foi possível ler a entrada do usuário.");

        let entrada = buffer.trim();
        let entrada_resultado = entrada.parse();
        let posicao: i32 = match entrada_resultado {
            Ok(num) => num,
            Err(_) => {
                println!(
                    "Não foi possível converter '{}' para número inteiro.",
                    entrada
                );
                continue;
            }
        };

        if is_posicao_invalida(posicao) {
            println!("Posição inválida. Digite um número entre 1 e 9.");
            continue;
        }

        let resultado = jogo.jogar(posicao, Rc::clone(&jogador_atual));

        let estado_jogo = match resultado {
            Ok(estado) => estado,
            Err(msg) => {
                if MSG_JOGADA_DUPLICADA == msg {
                    println!("{}", MSG_JOGADA_DUPLICADA);
                    continue;
                }
                panic!("Erro: {}", msg);
            }
        };

        match estado_jogo {
            EstadoJogo::Aberto => {
                jogador_atual = if jogador_atual == maria {
                    Rc::clone(&joao)
                } else {
                    Rc::clone(&maria)
                };
            }
            EstadoJogo::Empate => {
                println!("Empate!");
                break;
            }
            EstadoJogo::Finalizado(vencedor) => {
                println!("Parabéns {}, você venceu!!", vencedor.get_nome());
                break;
            }
        }
    }
}
