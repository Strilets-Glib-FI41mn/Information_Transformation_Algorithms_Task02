use std::{fmt::Display, fs::File};

use task2::bit_reader::FileBitReader;
use task2::bit_writter::FileBitWriter;
use bool_vec::bool_vec;
use inquire_derive::Selectable;

#[derive(PartialEq)]
enum State{
    Started,
    Exiting
}
#[derive(Debug, Copy, Clone, Selectable)]
#[allow(clippy::upper_case_acronyms)]


enum Action{
    Read,
    Write,
    Exit
}
impl Display for Action{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Read => write!(f, "Read file"),
            Action::Write => write!(f, "Write file"),
            Action::Exit => write!(f, "Exit"),
        }
    }
}

use task2::bool_vec_from_string;

fn main() -> Result<(), Box<dyn std::error::Error>>  {
    if cfg!(feature = "write_foo"){
        let file = File::create("foo").unwrap();
        let mut file_writter = FileBitWriter::new(file);
        file_writter.write_bits(bool_vec!(0110_0000_1))?;
        file_writter.write_bits(bool_vec!(1110_0000_1))?;
        file_writter.write_bits(bool_vec_from_string("1100_1100_1"))?;
        drop(file_writter);
    }

    let mut file_writter: Option<FileBitWriter> = None;
    let mut file_reader: Option<FileBitReader> = None;
    let mut state = State::Started;
    while state != State::Exiting{
        let selected = Action::select("").prompt();
        match selected{
            Ok(Action::Exit) =>{
                state = State::Exiting;
            }
            Ok(Action::Write) =>{
                if file_reader.is_some(){
                    file_reader = None;
                }
                if file_writter.is_none(){
                    let file_name = inquire::Text::new("Into what file write binary data")
                    .prompt().unwrap();
                    let file = File::create(&file_name).unwrap();
                    file_writter = Some(FileBitWriter::new(file));
                }
                let data =  inquire::Text::new("Data:").prompt().unwrap();
                if let Some(writter) = & mut file_writter{
                    let _ = writter.write_bits(bool_vec_from_string(&data));
                }
            }

            Ok(Action::Read) =>{
                if file_writter.is_some(){
                    file_writter = None;
                }
                if file_reader.is_none(){
                    let file_name = inquire::Text::new("Into what file write binary data")
                    .prompt().unwrap();
                    let file = File::open(&file_name).unwrap();
                    file_reader = Some(FileBitReader::new(file));
                }
                if let Some(reader) = &mut file_reader{
                    let input_string =  inquire::Text::new("How many bits?:").prompt().unwrap();
                    let len = input_string.parse().expect("Not a valid number");
                    let result = reader.read_bits_binary(len);
                    match result{
                        Ok(text) => println!("{}", text),
                        Err(er) => println!("{}", er)
                    }
                    //println!("{:?}", result);

                }
            }
            _ => {
                println!("Not implemented yet")
            }
        }
    }
    Ok(())
}