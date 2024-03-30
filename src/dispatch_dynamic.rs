trait Animal {
    fn emitir_som(&self) -> String;
}

trait Domestico {}
trait Selvagem {}

struct Cachorro {}
struct Gato {}
struct Urso {}

impl Animal for Cachorro {
    fn emitir_som(&self) -> String {
        format!("Cachorro latindo!")
    }
}

impl Animal for Gato {
    fn emitir_som(&self) -> String {
        format!("Gato miando!")
    }
}

impl Animal for Urso {
    fn emitir_som(&self) -> String {
        format!("Urso urgindo!")
    }
}

trait AnimalSelvagem: Animal + Selvagem {}
impl Selvagem for Urso {}
impl AnimalSelvagem for Urso {}

#[allow(dead_code)]
fn emitir_som(animal: &dyn Animal) -> String {
    animal.emitir_som()
}

#[allow(dead_code)]
fn emitir_som_generics<T: Animal>(animal: &T) -> String {
    animal.emitir_som()
}

#[allow(dead_code)]
fn emitir_som_selvagem_generics<T: AnimalSelvagem>(animal: &T) -> String {
    println!("animal selvagem irá emitir um som:");
    animal.emitir_som()
}

#[allow(dead_code)]
pub fn entendendo_traits() {
    let animais = criar_animais();

    imprimir_vetor(&animais);

    let box_cachorro = Box::new(Cachorro {});
    let box_gato = Box::new(Gato {});
    let box_urso = Box::new(Urso {});

    println!("{}", emitir_som_generics(box_cachorro.as_ref()));
    println!("{}", emitir_som_generics(box_gato.as_ref()));
    println!("{}", emitir_som_generics(box_urso.as_ref()));

    println!("{}", emitir_som_selvagem_generics(box_urso.as_ref()));
}

#[allow(dead_code)]
fn criar_animais() -> Vec<Box<dyn Animal>> {
    let cachorro: Box<dyn Animal> = Box::from(Cachorro {});
    let gato: Box<dyn Animal> = Box::from(Gato {});
    let urso: Box<dyn Animal> = Box::from(Urso {});

    vec![cachorro, gato, urso]
}

#[allow(dead_code)]
fn imprimir_vetor(animais: &Vec<Box<dyn Animal>>) -> () {
    for animal in animais {
        println!("{}", emitir_som(animal.as_ref()));
    }
}
