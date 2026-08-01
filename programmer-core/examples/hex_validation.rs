use programmer_core::hex;

fn main() {
    if let Err(error) = hex::validate_file("test.hex") {
        eprintln!("{}", error);
    } else {
        println!("Hex file is valid");
    }
}