use std::io;
use show_results::*;
use std::time::{Instant};

fn main() {
    println!("Enter a text to count the words:");

    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("Failed to read input");

    let start_time = Instant::now();

    let cleaned_string: String = text
        .chars()
        .map(|c|{
            if c.is_alphanumeric() || c.is_whitespace(){
                c
            } else {
                ' '
            }
        }).collect();

    let frequencies = count_words(&cleaned_string);
    let total_counted_words: u32 = frequencies.values().sum();


    let mut sorted_frequencies: Vec<_> = frequencies.into_iter().collect();

    sorted_frequencies.sort_by(|&(_, count1), &(_, count2)| count2.cmp(&count1));

    println!("\nWord frequency:");
    for (word, count) in sorted_frequencies {
        println!("{}: {}", word, count);
    }
     let end_time = Instant::now();
    let duration = end_time - start_time;


    println!("The operation took: {:?}", duration);
    println!("The total counted words was {}", total_counted_words);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_repeated_words() {
        let text = "hello hello world";
        let frequencies = count_words(text);

        assert_eq!(frequencies.get("hello"), Some(&2));
        assert_eq!(frequencies.get("world"), Some(&1));
    }

    #[test]
    fn ignore_case() {
        let text = "Hello hello";
        let frequencies = count_words(text);

        assert_eq!(frequencies.get("hello"), Some(&2));
    }

    #[test]
    fn ignore_extra_spaces() {
        let text = "  hello   world  ";
        let frequencies = count_words(text);

        assert_eq!(frequencies.get("hello"), Some(&1));
        assert_eq!(frequencies.get("world"), Some(&1));
    }

    #[test]
    fn empty_text() {
        let text = "";
        let frequencies = count_words(text);

        assert_eq!(frequencies.len(), 0);
    }
}
