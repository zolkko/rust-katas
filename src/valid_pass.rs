#![feature(str_split_once)]
use std::error::Error;


#[derive(Debug)]
struct Range {
    start: usize,
    end: usize,
}

#[derive(Debug)]
struct Input<'a> {
    range: Range,
    letter: char,
    password: &'a str,
}


fn parse_input<'a>(input: &'a str) -> Result<Input<'a>, Box<dyn Error>> {

    let data: Vec<_> = input.split_whitespace().collect();
    match &data[..] {
        [range, letter_pat, password] => {

            if let Some((start_str, end_str)) = range.split_once('-') {
                let start: usize = start_str.parse()?;
                let end: usize = end_str.parse()?;
                if start >= end {
                    return Err("start must be smaller than end part of the range".into());
                }

                if let Some(letter) = letter_pat.chars().next() {
                    Ok(Input {
                        range: Range { start, end },
                        letter,
                        password
                    })
                } else {
                    Err("Letter specified in a wrong way".into())
                }
            } else {
                Err("cannot parse range".into())
            }
        }
        _ => Err("invalid input".into()),
    }
}

fn validate_input(input: Input<'_>) -> bool {
    let mut count = 0;
    for c in input.password.chars() {
        if c == input.letter {
            count += 1;
        }
        if count > input.range.end {
            return false;
        }
    }
    count >= input.range.start
}

fn main() {
    let inputs = vec![
        "1-3 a: abcde",
        "1-3 b: cdefg",
        "2-9 c: ccccccccc",
    ];

    for i in inputs {
        match parse_input(i) {
            Ok(x) => {
                print!("PARSED: {:?}", x);
                if validate_input(x) {
                    println!("\tis valid");
                } else {
                    println!("\tis not valid");
                }
            },
            Err(err) => println!("ERROR: {:?}", err),
        }
    }
}