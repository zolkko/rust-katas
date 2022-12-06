use std::io::{self, Write, BufRead};
use std::num::NonZeroUsize;


const WORD: [char; 6] = ['Z', 'O', 'L', 'K', 'K', 'O'];


#[inline(always)]
fn padding(v: usize) -> usize {
    (v - 1) * 2
}

fn print_pyramid<W: Write>(mut out: W, height: NonZeroUsize) -> io::Result<()> {
    let max_padding: usize = padding(height.get());
    let mut current = WORD[0].to_string();

    let row_padding_str = " ".repeat(max_padding);
    out.write_fmt(format_args!("{}{}", row_padding_str, current))?;

    for row in 2..=height.get() {
        let row_padding = max_padding - padding(row);
        let row_char = WORD[row % WORD.len()];

        current.insert(0, ' ');
        current.insert(0, row_char);

        current.push(' ');
        current.push(row_char);

        let row_padding_str = " ".repeat(row_padding);
        out.write_fmt(format_args!("{}{}", row_padding_str, current))?;
    }
    Ok(())
}

fn main() {
    let input_buff = {
        let mut input_buff = String::new();
        let mut stdin_guard = io::stdin().lock();
        stdin_guard.read_line(&mut input_buff).unwrap();
        input_buff
    };
    let height = input_buff.trim().parse::<usize>().unwrap();
    let Some(valid_height) = NonZeroUsize::new(height) else { panic!("height must be greater than zero") };

    print_pyramid(io::stdout().lock(), valid_height).unwrap();
}
