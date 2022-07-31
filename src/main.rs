use std::io;

use calculator::string_decoder::string_decoder::decode;

fn main() {
    loop {
        let mut operation = String::new();

        println!("Input your mathematical operation:");

        // read operation vom command line
        io::stdin().read_line(&mut operation).unwrap();

        // calculate result
        let result = decode(&operation);

        // output result
        println!("The result is: {result}")
    }
}
