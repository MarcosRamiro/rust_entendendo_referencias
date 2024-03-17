use std::io::stdin;
use std::rc::Rc;

const MSG_JOGADA_DUPLICADA: &str = "Jogada duplicada.";

#[derive(Debug)]
struct Jogador {
    nome: String,
    ganhei: bool,
}

impl PartialEq for Jogador {
    fn eq(&self, other: &Self) -> bool {
        self.get_nome().eq(other.get_nome())
    }
}

impl Jogador {
    fn maria() -> Self {
        Self {
            nome: "Maria".to_string(),
            ganhei: false,
        }
    }
    fn joao() -> Self {
        Self {
            nome: "Joao".to_string(),
            ganhei: false,
        }
    }
    fn get_nome(&self) -> &str {
        self.nome.as_str()
    }

    /***
    caso seja necessário atualizar o campo Jogador.ganhei, será necessário mudar o tipo,
    pois apenas o Rc<T> não permite mutação.
    Isso será resolvido em outro múdulo.
     */
    fn atualizar_ganhador(&self, ganhei: bool) {
        println!("atualizar_ganhador {}", ganhei);
    }
}

#[derive(Debug, Clone)]
struct Jogada {
    posicao: i32,
    jogador: Rc<Jogador>,
}

impl Jogada{
    fn new(posicao: i32, jogador: Rc<Jogador>) -> Self {
        Self {
            posicao: posicao,
            jogador: jogador,
        }
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

fn is_jogada_duplicada(jogadas: &Vec<Jogada>, jogada: &Jogada) -> bool {
    jogadas
        .iter()
        .any(|jogada_| jogada_.get_posicao() == jogada.get_posicao())
}

impl FazerJogada for Jogo {
    fn jogar(&mut self, posicao: i32, jogador: Rc<Jogador>) -> Result<EstadoJogo, String> {
        if self.estado_jogo != EstadoJogo::Aberto {
            return Err("Estado inválido.".to_string());
        }

        let jogada = Jogada::new(posicao, Rc::clone(&jogador));

        if is_jogada_duplicada(&self.jogadas, &jogada) {
            return Err(MSG_JOGADA_DUPLICADA.to_string());
        }

        self.jogadas.push(jogada);

        for (posicao_1, posicao_2, posicao_3) in get_jogadas_vencedoras(Rc::clone(&jogador)) {
            if self.jogadas.contains(&posicao_1)
                && self.jogadas.contains(&posicao_2)
                && self.jogadas.contains(&posicao_3)
            {
                self.estado_jogo = EstadoJogo::Finalizado(Rc::clone(&jogador));
                jogador.atualizar_ganhador(true);
                return Ok(EstadoJogo::Finalizado(jogador));
            }
        }

        if self.jogadas.len() == 9 {
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
}

fn criar_jogada(posicao: i32, jogador: Rc<Jogador>) -> Jogada {
    Jogada::new(posicao, jogador)
}

fn construir_tupla(
    n1: i32,
    n2: i32,
    n3: i32,
    jogador: Rc<Jogador>,
) -> (Jogada, Jogada, Jogada) {
    (
        criar_jogada(n1, Rc::clone(&jogador)),
        criar_jogada(n2, Rc::clone(&jogador)),
        criar_jogada(n3, Rc::clone(&jogador)),
    )
}

fn get_jogadas_vencedoras(jogador: Rc<Jogador>) -> Vec<(Jogada, Jogada, Jogada)> {
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

fn is_posicao_invalida(posicao: i32) -> bool {
    posicao < 1 || posicao > 9
}

pub fn jogar() {
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

        println!("Numero de cópias da Maria {}", Rc::strong_count(&maria));
        println!("Numero de cópias do João {}", Rc::strong_count(&joao));
    }

    println!("Numero de cópias da Maria {}", Rc::strong_count(&maria));
    println!("Numero de cópias do João {}", Rc::strong_count(&joao));

}
