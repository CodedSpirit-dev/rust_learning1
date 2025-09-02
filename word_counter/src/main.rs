use std::collections::HashMap;
use std::io;
use show_results::*;

fn main() {
    println!("Introduce un texto para contar las palabras:");

    io::stdin()
        .read_line(&mut texto)
        .expect("Error al leer la entrada");

    let frequencies = show_results()
    println!("\nFrecuencia de palabras:");
    for (palabra, count) in frecuencias {
        println!("{}: {}", palabra, count);
    }
}

fn contar_palabras(texto: &str) -> HashMap<String, u32> {
    let mut frecuencias = HashMap::new();

    // 4. Usar iteradores para procesar el texto
    for palabra in texto.trim().to_lowercase().split_whitespace() {
        // 5. Pattern matching con .entry() y or_insert()
        let contador = frecuencias.entry(palabra.to_string()).or_insert(0);
        *contador += 1;
    }

    frecuencias
}

// 6. Pruebas unitarias para verificar el funcionamiento
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contar_palabras_repetidas() {
        let texto = "hello hello world";
        let frecuencias = contar_palabras(texto);

        assert_eq!(frecuencias.get("hello"), Some(&2));
        assert_eq!(frecuencias.get("world"), Some(&1));
    }

    #[test]
    fn ignorar_mayusculas() {
        let texto = "Hello hello";
        let frecuencias = contar_palabras(texto);

        assert_eq!(frecuencias.get("hello"), Some(&2));
    }

    #[test]
    fn ignorar_espacios_extra() {
        let texto = "  hello   world  ";
        let frecuencias = contar_palabras(texto);

        assert_eq!(frecuencias.get("hello"), Some(&1));
        assert_eq!(frecuencias.get("world"), Some(&1));
    }

    #[test]
    fn texto_vacio() {
        let texto = "";
        let frecuencias = contar_palabras(texto);

        assert_eq!(frecuencias.len(), 0);
    }
}
