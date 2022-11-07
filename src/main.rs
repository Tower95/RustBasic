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

    // Optener la edad de la consola y convertirla en un numero
    // println!("Porfavor introduce tu edad...");

    // let mut edad : String  = String::new();

    // std::io::stdin().read_line(&mut edad ).unwrap();

    // let edad_in : u8 = edad.trim().parse().unwrap();

    // println!("Hola bienvenido {} de {} años", nombre, edad_in);

    //simple formulario para pedir nombre y pais
    // let mut nombre : String = String::new();
    // let mut pais: String = String::new();

    // println!("Hola bienvenido porfavor indique su nombre completo...");

    // std::io::stdin().read_line(&mut nombre).unwrap();

    // nombre = nombre.trim().to_string();

    // println!("indique su pais de recidencia...");

    // std::io::stdin().read_line(&mut pais).unwrap();

    // pais = pais.trim().to_string();

    // println!("Tu nomnbre es {} y vives en {}", nombre, pais);

    // Condicionale

    // println!("Porfavor introduce tu edad...");

    // let mut edad: String = String::new();

    // std::io::stdin().read_line(&mut edad).unwrap();

    // let edad_in: u8 = edad.trim().parse().unwrap();

    // if edad_in >= 18 {
    //     println!("Party in tha club");
    // } else {
    //     println!("A tu casa a ver pocoyo... PATO");
    // }

    // let mut eleccion : String = String::new();

    // println!("Esta es tu última oportunidad. Después, ya no podrás echarte atrás. Si tomas la pastilla azul, fin de la historia. Despertarás en tu cama y creerás lo que quieras creerte. Si tomas la roja, te quedas en el País de las Maravillas y yo te enseñaré hasta dónde llega la madriguera de conejos. Recuerda lo único que te ofrezco es la verdad. Nada más.");
    // println!("Qué pastilla tomarás? roja o azul?");

    // std::io::stdin().read_line(&mut eleccion).unwrap();

    // eleccion.trim().to_string();

    // let pastilla : &str = &eleccion.trim();

    // if pastilla == "roja" {
    //     println!("Despretaste en tu casa");
    // }
    // else if pastilla == "azul" {
    //     println!("Bueno dorty de Kansas puedes despedirte...");
    // } else{
    //     println!("Knock knock despierta NEO!!!");
    // }

    // Ciclo  Loop

    //Creamos los dos numeros que vamos a crear
    let primer_numero: i16 = 102;
    let  segundo_numero: i16 = 32;
    let suma = primer_numero + segundo_numero;

    // Creacion del loop

    loop { // Itera esto asta que el usuario asierte
        // mostrar los dos numeros

        println!(
            "Cual es el resultado de sumar\n{} + {}",
            primer_numero, segundo_numero
        );

        //capturar respuesta del ususario

        let mut respuesta: String = String::new();
        std::io::stdin().read_line(&mut respuesta).unwrap();

        let  respuesta_in: i16 = respuesta.trim().parse().unwrap();

        // Comparamos los resultados
        if respuesta_in == suma {
            println!("Tu respuesta: {} es CORRECTA", respuesta);
            //termina el loop
            break;

        } else {
            println!(
                "Tu respuesta: {} es INCORRECTA intentalo de nuevo",
                respuesta
            );
        }
    }
}
