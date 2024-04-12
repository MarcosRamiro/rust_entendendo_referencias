use std::error::Error;

#[derive(Debug)]
pub enum ErroPersonalizado {
    mensagem(String),
    NaoLocalizado,
    OutroErro(Box<dyn Error>),
}

impl Error for ErroPersonalizado {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ErroPersonalizado::OutroErro(causa) => Some(causa.as_ref()),
            _ => None,
        }
    }
}

impl std::fmt::Display for ErroPersonalizado {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErroPersonalizado::mensagem(msg) => write!(f, "{}", msg),
            _ => write!(f, "Erro desconhecido"),
        }
    }
}

pub fn como_tratar_erros() -> Result<(), Box<dyn Error>> {
    let resultado = outra_funcao_que_da_erro();
    if let Err(erro) = resultado {
        if let Some(erro_source) = erro.source() {
            println!("Causa: {}", erro_source);
        }
        return Err(erro);
    }
    Ok(())
}

fn outra_funcao_que_da_erro() -> MyErro<()> {
    if rand::random() {
        // generates a boolean
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Erro de exemplo",
        )));
    }

    Err(Box::new(ErroPersonalizado::OutroErro(
        Box::new(ErroPersonalizado::mensagem("Meu erro personalizado.".to_string())),
    )))
}

pub type MyErro<T> = Result<T, Box<dyn Error>>;
