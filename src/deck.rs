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
    fn shuffle(&self){
        
    }
}