use std::fs::File;

use bit_writter::FileBitWriter;
mod bit_writter;
mod bit_reader;
use bool_vec::bool_vec;
fn main()  {
    println!("Hello, world!");
    //compile_error!();
    let file = File::create("foo").unwrap();
    let mut file_writter = FileBitWriter::new(file);
    file_writter.write_bits(bool_vec!(0110_0000_1));
    file_writter.write_bits(bool_vec_from_string("1111_1111_1"));
    file_writter.write_bits(bool_vec!(011));
    file_writter.write_bits(bool_vec!(111));

}


fn bool_vec_from_string(input: &str) -> Vec<bool>{
    input.chars().filter_map(|ch|{
        match ch{
            '_' => None,
            '0' => Some(false),
            '1' => Some(true),
            _ => {panic!("Bool vector is created from 0s and 1s!")}
        }
    })
    .collect()
}