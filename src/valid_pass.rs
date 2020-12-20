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

fn main() -> Result<(), Box<dyn Error>> {
    let mut valid_lines = 0;
    let mut line = String::with_capacity(10);
    while std::io::stdin().read_line(&mut line).is_ok() {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            break;
        }
        let parsed_input = parse_input(&trimmed_line)?;
        if validate_input(parsed_input) {
            valid_lines += 1;
        }
        line.truncate(0);
    }
    println!("{}", valid_lines);
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_inputs() {
        let inputs = vec![
            ("1-3 a: abcde", true),
            ("1-3 b: cdefg", false),
            ("2-9 c: ccccccccc", true),
        ];

        for (i, r) in inputs {
            assert_eq!(parse_input(i).map(validate_input).unwrap(), r);
        }
    }

}