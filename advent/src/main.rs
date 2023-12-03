use std::fs::File;
use std::io::Read;

fn main(){
    // Load the input
    let mut data = File::open("input.txt").unwrap();
    let mut content = String::new();
    data.read_to_string(&mut content).unwrap();
    let input_vec = content.split("\n");

    let mut toy_data = File::open("toy_input.txt").unwrap();
    let mut toy_content = String::new();
    toy_data.read_to_string(&mut toy_content).unwrap();
    let toy_input_vec = toy_content.split("\n");
}
