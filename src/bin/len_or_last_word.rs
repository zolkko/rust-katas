use std::io::{stdin, BufRead};
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    println!("Input a sentence that has at least one word: ");

    let mut input_buff = String::new();
    stdin().lock().read_line(&mut input_buff)?;
    let input_buff = input_buff.trim_end_matches('\n').to_string();

    let length_of_last_word = input_buff.rsplit(' ').map(str::len).filter(|x| *x >= 0).next().expect("at least one word");
    println!("The length of the last word is in [{input_buff}] {length_of_last_word}");

    Ok(())
}