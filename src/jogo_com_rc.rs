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
    fn maria() -> Self {
        Self::new("Maria".to_string(), false)
    }

    fn joao() -> Self {
        Self::new("Joao".to_string(), false)
    }

    fn get_nome(&self) -> &str {
        self.nome.as_str()
    }

    fn new(nome: String, eh_vencedor: bool) -> Self {
        Self { nome, eh_vencedor }
    }

    fn tornar_vencedor(&self) -> Self {
        Self::new(self.get_nome().to_string(), true)
    }

    /***
    caso seja necessário atualizar o campo Jogador.ganhei, será necessário mudar o tipo,
    pois apenas o Rc<T> não permite mutação.
    Isso será resolvido em outro múdulo.
    Estive pensando em progração funcional. O ideal é evitar dado mutável, seria melhor
    criar um novo Jogador, com o status atualizado.
     */
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

#[derive(PartialEq, Debug)]
enum EstadoJogo {
    Aberto,
    Empate,
    Finalizado(Rc<Jogador>),
}

struct Jogo {
    estado_jogo: EstadoJogo,
    jogadas: Vec<Jogada>,
}

trait FazerJogada {
    fn jogar(&mut self, posicao: i32, jogador: Rc<Jogador>) -> Result<EstadoJogo, String>;
}

impl FazerJogada for Jogo {
    fn jogar(&mut self, posicao: i32, jogador: Rc<Jogador>) -> Result<EstadoJogo, String> {
        if self.estado_jogo != EstadoJogo::Aberto {
            return Err("Estado inválido.".to_string());
        }

        let jogada = Jogada::new(posicao, Rc::clone(&jogador));

        self.adicionar_jogada(jogada)?;

        if let Some(value) = self.is_jogada_vencedora(&jogador) {
            return value;
        }

        if !self.tem_jodada_disponivel() {
            self.estado_jogo = EstadoJogo::Empate;
            return Ok(EstadoJogo::Empate);
        }

        Ok(EstadoJogo::Aberto)
    }
}

impl Jogo {
    fn new() -> Jogo {
        Jogo {
            estado_jogo: EstadoJogo::Aberto,
            jogadas: Vec::new(),
        }
    }

    fn is_venceu_jogo(&self, posicao_1: &Jogada, posicao_2: &Jogada, posicao_3: &Jogada) -> bool {
        self.jogadas.contains(&posicao_1)
            && self.jogadas.contains(&posicao_2)
            && self.jogadas.contains(&posicao_3)
    }

    fn tem_jodada_disponivel(&self) -> bool {
        !(self.jogadas.len() == 9)
    }

    fn is_jogada_vencedora(&mut self, jogador: &Rc<Jogador>) -> Option<Result<EstadoJogo, String>> {
        for (posicao_1, posicao_2, posicao_3) in get_jogadas_vencedoras(Rc::clone(&jogador)) {
            if self.is_venceu_jogo(&posicao_1, &posicao_2, &posicao_3) {
                let vencedor: Rc<Jogador> = Rc::new(jogador.tornar_vencedor());
                self.estado_jogo = EstadoJogo::Finalizado(Rc::clone(&vencedor));
                return Some(Ok(EstadoJogo::Finalizado(vencedor)));
            }
        }
        None
    }

    fn adicionar_jogada(&mut self, jogada: Jogada) -> Result<(), String> {
        if self.is_jogada_duplicada(&jogada) {
            return Err(MSG_JOGADA_DUPLICADA.to_string());
        }

        self.jogadas.push(jogada);

        Ok(())
    }

    fn is_jogada_duplicada(&self, jogada: &Jogada) -> bool {
        self.jogadas
            .iter()
            .any(|jogada_| jogada_.get_posicao() == jogada.get_posicao())
    }
}



fn get_jogadas_vencedoras(jogador: Rc<Jogador>) -> Vec<(Jogada, Jogada, Jogada)> {

    fn construir_tupla(n1: i32, n2: i32, n3: i32, jogador: Rc<Jogador>) -> (Jogada, Jogada, Jogada) {

        let criar_jogada = |posicao: i32| -> Jogada {
            Jogada::new(posicao, Rc::clone(&jogador))
        };

        (
            criar_jogada(n1),
            criar_jogada(n2),
            criar_jogada(n3),
        )
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


pub fn jogar() {

    let is_posicao_invalida = |posicao: i32| -> bool {
        posicao < 1 || posicao > 9
    };

    let maria = Rc::new(Jogador::maria());
    let joao = Rc::new(Jogador::joao());
    let mut jogador_atual = Rc::clone(&maria);
    let mut jogo: Jogo = Jogo::new();

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
