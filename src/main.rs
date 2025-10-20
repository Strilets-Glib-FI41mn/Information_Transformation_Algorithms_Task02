use std::fs::File;

use bit_reader::FileBitReader;
use bit_writter::FileBitWriter;
mod bit_writter;
mod bit_reader;
use bool_vec::bool_vec;
fn main() -> Result<(), Box<dyn std::error::Error>>  {
    println!("Hello, world!");
    //compile_error!();
    let file = File::create("foo").unwrap();
    let mut file_writter = FileBitWriter::new(file);
    file_writter.write_bits(bool_vec!(0110_0000_1))?;
    file_writter.write_bits(bool_vec!(1110_0000_1))?;
    file_writter.write_bits(bool_vec_from_string("1100_1100_1"))?;
    
    file_writter.write_bits(bool_vec!(1111))?;
    let file = File::open("foo").unwrap();
    let mut file_reader = FileBitReader::new(file);
    println!("{:?}",file_reader.read_bits(8));
    println!("{:?}",file_reader.read_bits(10));
    println!("{:?}",file_reader.read_bits(6));
    
    /*


    file_writter.write_bits(bool_vec!(1110));

    file_writter.write_bits(bool_vec!(1111));
    println!("{:?}",file_reader.read_bits(2));
    println!("{:?}",file_reader.read_bits(6));
     */
    Ok(())
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