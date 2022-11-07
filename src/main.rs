fn main() {
    // declaracion de variables
    // let mut edad: u8 = 27;
    // let nombre: &str = "Ismael Torres";
    // edad = edad+1;

    // let temperatura_max: i8 = 40;
    // let temperatura_min: i8 = -15;

    // println!("Hola soy {} y tengo {} años",nombre, edad);
    // println!("En mi ciudad la temperatura maxima de este año fue de: {}\n y la temperatura minima fue: {}", temperatura_max,temperatura_min);

    // // Introdocir datos
    // println!("Porfavor introduce tu nomnbre...");

    // let mut nombre: String  = String::new(); 
    
    // std::io::stdin().read_line(&mut nombre ).unwrap();

    // nombre = nombre.trim().to_string();

    // // Optener la edad de la consola y convertirla en un numero
    // println!("Porfavor introduce tu edad...");
    
    // let mut edad : String  = String::new(); 
    
    // std::io::stdin().read_line(&mut edad ).unwrap();

    // let edad_in : u8 = edad.trim().parse().unwrap();
    
    // println!("Hola bienvenido {} de {} años", nombre, edad_in);

    let mut nombre : String = String::new();
    let mut pais: String = String::new();

    println!("Hola bienvenido porfavor indique su nombre completo...");

    std::io::stdin().read_line(&mut nombre).unwrap();

    nombre = nombre.trim().to_string();


    println!("indique su pais de recidencia...");

    std::io::stdin().read_line(&mut pais).unwrap();
    
    pais = pais.trim().to_string();

    println!("Tu nomnbre es {} y vives en {}", nombre, pais);

    
}

