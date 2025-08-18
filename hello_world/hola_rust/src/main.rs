use std::io;
use std::io::Write;

fn main() {
    println!("{},{}!", "Hello", "world"); // Hello, world!
    println!("{0},{1}!", "Hello", "world"); // Hello, world!
    println!("{greeting},{name}!", greeting = "Hello", name = "world"); // Hello, world!

    let name = "world";
    println!("Hello, {name}!"); // Hello, world!

    let name = "world";
    println!("Hello, {name}!"); // Hello, world!

    let (greeting, name) = ("Hello", "world");
    println!("Hey, {greeting} {name}!"); // Hey, Hello world!

    println!("{:?}", [1, 2, 3]); // [1, 2, 3]
    println!("{:#?}", [1, 2, 3]); //
                                  /*
                                  [
                                      1,
                                      2,
                                      3,
                                  ]
                                  */

    // The format! macro is used to store the formatted string in a variable.
    let x = format!("{}, {}!", "Hello", "world");
    println!("{}", x); // Hello world!

    // Rust has a print!() macro that is similar to println!() but does not add a newline.
    print!("Hello, world!"); // Hello, world! without a newline
    println!(); // Prints a newline

    print!("Hello, world!\n"); // Hello, world! With a newline

    say_hello_world()
}

fn get_string_from_user(prompt: &str) -> String {
    // Muestra el texto del prompt al usuario con salto de línea
    println!("{prompt}");
    // Forzamos a que el contenido en el buffer de stdout se imprima inmediatamente
    io::stdout().flush().unwrap();

    // Creamos un String vacío llamado `input` donde se guardará lo que escriba el usuario
    let mut input = String::new();

    // Leemos una línea desde la entrada estándar (teclado) y la almacenamos en `input`
    // Si hay un error al leer, el programa hace panic con el mensaje "Failed to read line"
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // .trim() quita espacios en blanco al inicio y final (incluye el `\n` del Enter)
    // .to_string() convierte el &str resultante en un String propio
    input.trim().to_string()
}

fn say_hello_world() {
    // Llama a `get_string_from_user` mostrando un mensaje
    // El usuario escribe algo → se devuelve un String guardado en `user_greeting`
    let user_greeting = get_string_from_user("Enter a greeting:");

    // Repite lo mismo pero con otro mensaje
    // Aquí el usuario debería escribir la palabra "world"
    let user_world = get_string_from_user("Enter word \"world\" :");

    // Construimos una nueva cadena formateada con los valores capturados
    // format! devuelve un String con el resultado
    let result = format!("{}, {}!", user_greeting, user_world);

    // Mostramos en pantalla el saludo final
    println!("{result}");
}
