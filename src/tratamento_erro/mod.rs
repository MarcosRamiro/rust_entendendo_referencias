use std::error::Error;

#[derive(Debug)]
pub enum Erro {
    mensagem(String),
    NaoLocalizado,
}

impl Error for Erro {}

impl std::fmt::Display for Erro {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Erro::mensagem(msg) => write!(f, "{}", msg),
            _ => write!(f, "Erro desconhecido"),
        }
    }
}

pub fn como_tratar_erros() -> Result<(), Box<dyn Error>> {
    let valor = outra_funcao_que_da_erro()?;
    Ok(())
}

fn outra_funcao_que_da_erro() -> MyErro<()> {
    if rand::random() {
        // generates a boolean
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Erro de exemplo",
        )));
    }

    Err(Box::new(Erro::mensagem("Aqui deu ruim".to_string())))
}

pub type MyErro<T> = Result<T, Box<dyn Error>>;
