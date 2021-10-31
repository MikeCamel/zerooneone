use std::io::{stdin, stdout, Read, Result, Write};

const INPUT: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const OUTPUT: &[u8] = b"NOPQRSTUVWXYZABCDEFGHIJKLMnopqrstuvwxyzabcdefghijklm";

fn main() -> Result<()> {
    for result in stdin().bytes() {
        let input = result?;

        // If the input is in the Latin alphabet, do ROT13.
        // Otherwise, output the input unmodified.
        let output = match INPUT.iter().position(|b| b == &input) {
            Some(index) => OUTPUT[index],
            None => input,
        };

        stdout().write_all(&[output])?;
    }

    Ok(())
}
