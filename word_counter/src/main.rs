use std::io;
use show_results::*;

fn main() {
    println!("Introduce un texto para contar las palabras:");

    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("Error al leer la entrada");

    let frequencies = count_words(&text);
    println!("\nFrecuencia de palabras:");
    for (word, count) in frequencies {
        println!("{}: {}", word, count);
    }
}

// 6. Pruebas unitarias para verificar el funcionamiento
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_repeated_words() {
        let texto = "hello hello world";
        let frecuencias = count_words(texto);

        assert_eq!(frecuencias.get("hello"), Some(&2));
        assert_eq!(frecuencias.get("world"), Some(&1));
    }

    #[test]
    fn ignorar_mayusculas() {
        let texto = "Hello hello";
        let frecuencias = count_words(texto);

        assert_eq!(frecuencias.get("hello"), Some(&2));
    }

    #[test]
    fn ignorar_espacios_extra() {
        let texto = "  hello   world  ";
        let frecuencias = count_words(texto);

        assert_eq!(frecuencias.get("hello"), Some(&1));
        assert_eq!(frecuencias.get("world"), Some(&1));
    }

    #[test]
    fn texto_vacio() {
        let texto = "";
        let frecuencias = count_words(texto);

        assert_eq!(frecuencias.len(), 0);
    }
}
