use anyhow::Result;

use std::{
    env,
    fs::File,
    io::{stdout, BufRead, BufReader, Write},
    path::Path,
};

fn main() -> Result<()> {
    let file =
        File::open(Path::new(&env::args().nth(1).ok_or_else(|| {
            anyhow::anyhow!("where's the program file you dumbass!")
        })?))
        .map_err(|_| anyhow::anyhow!("can't open the file, try giving a valid path."))?;

    let mut bytes: Vec<u8> = Vec::new();
    for line in BufReader::new(&file).lines() {
        for token in line?.split_whitespace() {
            bytes.push(
                u8::from_str_radix(token, 16).map_err(|x| anyhow::anyhow!("parse fail: {}", x))?,
            );
        }
    }
    let mut stdout = stdout().lock();
    stdout
        .write_all(&bytes)
        .map_err(|x| anyhow::anyhow!("{}", x))?;
    Ok(())
}
