fn reto_temperatura() {

    println!("##### Reto Temperatura #####");

    let temperatura_minima: i8 = -13;
    let temperatura_maxima: i8 = 23;

    println!("La temperatura minima fue: {}", temperatura_minima);
    println!("La temperatura maxima fue: {}", temperatura_maxima);
}

fn main() {
    // Declarando y definiendo una variable
    let edad: u8 = 25; // u8 es un unsigned int de 8 bytes

    let nombre: &str = "Orazio"; // el string se declara como &str
    println!("Hola soy {} y tengo {} anhos", nombre, edad);

    println!();
    println!();
    println!();
    reto_temperatura();
}
