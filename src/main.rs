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

    println!(
        "Hola, Neo, tienes la opcion de escoger una pildora y esta cambiara tu vida para siempre."
    );
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

fn ciclo_loop() {
    println!("\n\n\n##### Clase del ciclo Loop #####");

    let numero_1: u8 = 123;
    let numero_2: u16 = 321;

    let suma = numero_1 as u16 + numero_2;

    loop {
        let mut suma_usuario = String::new(); // Aqui no se tiene que especificar el tipo de dato, ya que lo infiere Rust
        println!(
            "Por favor escribir la suma de {} y {}: ",
            numero_1, numero_2
        );
        std::io::stdin().read_line(&mut suma_usuario).unwrap();

        let suma_usuario: u16 = suma_usuario.trim().parse().unwrap();

        if suma_usuario == suma {
            println!("Acertaste! El resultado {} es correcto", suma);
            break;
        } else {
            println!(
                "El resultado ingresado ({}) no es correcto, intentalo de nuevo",
                suma_usuario
            );
        }
    }
}

fn arrays_y_for() {
    println!("\n\n\n##### Clase de arrays y ciclo for #####");

    // Declarando un vector
    // Si no se le agregara un String seria necesario indicar el tipo del vector, de lo contrario no compilaria
    let mut nombres: Vec<String> = Vec::new();
    print!("El vector vacio es: {:#?}\n", nombres); // :#? indica que sera un "pretty print" segun la documentacion, Vec no tiene implementado un metodo de impresion en consola como tal

    let mut nombre: String = String::new();
    for _ in 0..3 {
        // El prof usa 'i' como variable, pero yo uso _ ya que no la voy a usar y es valido

        println!("Por favor introduce un nombre:");
        // Interesante, la siguiente linea, si no se limpia la cadena de "nombre", concatenara lo que lea del input en "nombre"
        // Por eso hay que resetear la variable
        std::io::stdin().read_line(&mut nombre).unwrap();

        nombres.push(nombre.trim().to_string());
        nombre = "".to_string();
    }

    nombres.push("Camilo".to_string());
    println!(
        "Impresion normal con ':?': {:?}\nImpresion con ':#?': {:#?}",
        nombres, nombres
    );
    println!("{}", nombres.len());

    let len_nombres = nombres.len();
    // For con indices
    // No se por que no funcionaba for index in 0..nombres.len()
    // Debi declarar y asignar len_nombres antes del for nombre in nombres porque el mismo compilador de Rust
    // Me decia que despues de ese for se habia "movido" nombres, o algo asi... investigar
    println!("Imprimiendo con index variable");
    for index in 0..len_nombres {
        println!("{}", nombres[index]);
    }

    // Recorrido del array con un for, practicamente igual que en python
    for nombre in nombres {
        println!("Nombre: {}", nombre);
    }

    // Creando un array inmutable
    let hola = ["H", "O", "L", "A"];
    println!("Indice 0: {}", hola[0]);
    println!("Indice 1: {}", hola[1]);
    println!("Indice 2: {}", hola[2]);
    println!("Indice 3: {}", hola[3]);
    
}

fn main() {
    // Declarando y definiendo una variable
    let edad: u8 = 25; // u8 es un unsigned int de 8 bytes

    let nombre: &str = "Orazio"; // el string se declara como &str
    println!("Hola soy {} y tengo {} anhos", nombre, edad);

    reto_temperatura();

    input_usuario();

    reto_condicionales();

    ciclo_loop();

    arrays_y_for();
}
