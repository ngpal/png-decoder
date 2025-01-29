use std::{
    fs::File,
    io::{stdin, stdout, Read, Write},
};

const PNG_SIGNATURE: [u8; 8] = [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];
fn is_png(content: &File) -> bool {
    // Check file signature

    content
        .bytes()
        .take(8)
        .map(Result::ok)
        .eq(PNG_SIGNATURE.iter().copied().map(Some))
}

fn main() {
    print!("Enter file name: ");
    stdout().flush().unwrap();
    let mut filename = String::new();
    stdin()
        .read_line(&mut filename)
        .expect("Unable to read from stdin");
    filename = filename.trim().to_string();

    let file = File::open(filename).expect("Unable to open file");
    if is_png(&file) {
        println!("This is a PNG file");
    } else {
        println!("This is not a PNG file");
    }
}
