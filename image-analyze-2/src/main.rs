use std::env;
use std::fs::File;
use std::iter::FromIterator;

use png::Decoder;
use png::Transformations;

use morse::decode;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let image_filename = &args[1];
    println!("Using image file {}", image_filename);

    let mut decoder = Decoder::new(File::open(image_filename)?);
    decoder.set_transformations(Transformations::EXPAND);
    let (info, mut reader) = decoder.read_info()?;

    println!("{} x {}: {}", info.width, info.height, info.buffer_size());

    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf)?;

    let mut result = Vec::new();
    let mut lasti = 0;
    for (i, elem) in buf.into_iter().step_by(3).enumerate() {
        if elem == 255 {
            result.push((i - lasti) as u8 as char);
            lasti = i;
        }
    }

    let morse = String::from_iter(result);
    println!("{}", morse);

    let s = decode::decode(morse).unwrap();
    println!("{}", s);

    Ok(())
}
