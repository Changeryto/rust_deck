/* Se pretende incorporar las funciones:
 *  Objeto: deck - Representa una coleccion de cartas jugables
 *  new() - Crea un nuevo objeto deck que contiene una lista de cartas jugables
 * shuffle() - Cambia el orden de las cartas en el seck
 *  deal() - Remueve algunas cartas del deck y las regresa en una nueva lista.
 */ 

 // cargo add rand
use rand::{thread_rng, seq::SliceRandom};


 // Struct necesitan empezar con una mayuscula
 // cards: Lista de campos que este struct va a contener
 // Vec<String> Vector (arreglo que puede cambiar de tamanio) que va a contener Strings.
 
 // struct puede ser construido (instanciado) en multiples ocasiones.
 // No se llaman variables si no uniones, pero funcionan de forma similar

// Solo debe usarse en debug. 
// Una directiva define atributos para el struct Deck
// Da al compilador instrucciones adicionales
// En otras palabras, la directiva le da la instruccion al compilador de aniadir las funciones Debug al struct
#[derive(Debug)]
struct Deck {
    cards: Vec<String>
}
// Implementacion de inherencia
// Puede entenderse como forma de aniadir funciones (de clase) a un struct.
impl Deck {
    //fn nombre() -> Tipo de regreso
    // Self es referencia al tipo padre creado
    // Funcion asociada, funcion no asociada a una instancia particular
    fn new() -> Self {
        // Lista de cartas
        // Lista de valores
        let suits: [&str; 4] = ["Corazones", "Espadas", "Diamantes", "Treboles"];
        let values: [&str; 13] = ["Az", "Dos", "Tres", "Cuatro", "Cinco", "Seis", "Siete", "Ocho", "Nueve", "Diez", "Jokey", "Queen", "King"];

        let mut cards: Vec<String> = vec![];

        // for loop doble anidado
        for suit in suits {
            for value in values {
                // El macro format permite un template string
                let card = format!("{} de {}", value, suit);
                cards.push(card);
            }
        }
        return Deck {cards}; // Puede hacerse solo si tiene el mismo nombre el parametro y el valor
    }

    // Metodo (similar a metodo de clase), usar cuando se debe asociar a una instancia
    // &mut indica que debemos recibir una forma mutable del self
    fn shuffle(&mut self){
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    // usize es un tipo unsigned del tamanio de bits de la placa madre.
    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        return self.cards.split_off(
            self.cards.len() - num_cards
        );
    }
}

fn main() {


    // Se instancia un deck
    // inicializa nomre: tipo = Struct { requerimiento: vecMacro![vacio] }
    let mut deck: Deck = Deck::new(); 
    // Debe ser mut ya que shuffle debe poder cambiarlo

    // Cambia el orden de las cartas llamando a shuffle
    deck.shuffle();
    // Alternativa:
    // let deck: Deck = Deck { cards: Vec::new() };
    print!("Tu mano - {:#?}", deck);
    // {:?} solo debe usarse en debug
    // {:#?} permite impresion 1 a 
    // TODO: Error handeling para intentar subtraer en un overflow
    print!("Descarto 20 cartas {:#?}", deck.deal(20));
    print!("Resto de tu mano - {:#?}", deck);
}


// Mutable vs inmutable
// Inmutable: Por default, no puede modificarse ni cambiarse ahead of time.
// Mutable: Debe declararse.