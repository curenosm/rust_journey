fn main() {
    // El tipo del valor debe ser especificado
    // para las constantes.
    const HELLO: &str = "SUP";
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

}

fn main2() {
    let x = 5;

    // Podemos hacer un shadow de la variable
    // en este mismo alcance.
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn main3() {
    // El shadowing nos permite, usar el mismo
    // identificador para diferentes tipos.

    // Pues en realidad siempre se introducen
    // variables nuevas.
    let spaces = "   ";
    let spaces = spaces.len();

}