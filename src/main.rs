fn reto_temperatura() {

    println!("##### Reto Temperatura #####");

    let temperatura_minima: i8 = -13;
    let temperatura_maxima: i8 = 23;

    println!("La temperatura minima fue: {}", temperatura_minima);
    println!("La temperatura maxima fue: {}", temperatura_maxima);
}

fn reto_formulario() {
    println!("\n\n\n##### Reto formulario #####");

    let mut nombre: String = String::new();
    println!("Bienvenido/a, ingrese su nombre: ");
    std::io::stdin().read_line(&mut nombre).unwrap();

    let mut pais: String = String::new();
    println!("Ingresa tu pais de origen: ");
    std::io::stdin().read_line(&mut pais).unwrap();

    nombre = nombre.trim().to_string();
    pais = pais.trim().to_string();

    println!("Te llamas {} y eres de {}", nombre, pais)
}

fn input_usuario() {
    println!("##### Clase input usuario #####");

    // Al declarar el String se inicializa de la siguiente forma
    let mut nombre: String = String::new(); // String es mas pesado que &str pero tiene mas funcionalidades
    println!("Por favor ingresa tu nombre: ");
    std::io::stdin().read_line(&mut nombre).unwrap();
    nombre = nombre.trim().to_string(); // Aqui se castea a String con el to_string porque trim retorna un &str y nombre es de tipo String

    // Obteniendo edad de la consola
    println!("Por favor, ingresa tu edad: ");
    let mut edad: String = String::new(); // Inicializando como se inicializo el nombre
    std::io::stdin().read_line(&mut edad).unwrap();

    // Convirtiendo edad a numero
    let edad_int: u8 = edad.trim().parse().unwrap();

    println!("Hola, bienvenido/a {}, tienes una edad de: {}", nombre, edad_int);

    reto_formulario();
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

    println!();
    println!();
    println!();
    input_usuario();
}
