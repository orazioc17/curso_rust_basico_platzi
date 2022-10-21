fn reto_temperatura() {
    println!("\n\n\n##### Reto Temperatura #####");

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
    println!("\n\n\n##### Clase input usuario #####");

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

    println!(
        "Hola, bienvenido/a {}, tienes una edad de: {}",
        nombre, edad_int
    );

    reto_formulario();
}

fn reto_condicionales() {
    println!("\n\n\n##### Clase y reto de condicionales #####");

    println!("Hola, Neo, tienes la opcion de escoger una pildora y esta cambiara tu vida para siempre.");
    println!("Que pildora eliges? Tienes la opcion de la 'azul' o la 'roja'.");
    println!("La pildora 'azul' te mantendra dormido y no nos volveremos a ver.");
    println!("La pildora 'roja' te despertara de la matrix y me permitira mostrarte la verdad.");

    let mut pildora: String = String::new();
    println!("Ingresa el color de pildora que quieras tomar (azul/roja): ");
    std::io::stdin().read_line(&mut pildora).unwrap();
    pildora = pildora.trim().to_string().to_lowercase();

    if pildora == "azul" {
        println!("Adios, Neo, me equivoque contigo...");
    } else if pildora == "roja" {
        println!("Sabia que podia confiar en ti, ven conmigo, Neo...");
    } else {
        println!("Respuesta equivocada");
    }
}

fn main() {
    // Declarando y definiendo una variable
    let edad: u8 = 25; // u8 es un unsigned int de 8 bytes

    let nombre: &str = "Orazio"; // el string se declara como &str
    println!("Hola soy {} y tengo {} anhos", nombre, edad);

    reto_temperatura();

    input_usuario();

    reto_condicionales();
}
