use std::{fs::File, io::Write};

pub struct FileBitWriter{
    pub file: File,
    buff_index: u8,
    buffer: u8
}

impl FileBitWriter{
    pub fn write_bits(&mut self, input: Vec<bool>)  -> std::io::Result<usize> {
        let mut written = 0;
        for b in input.iter(){
            set_bit(&mut self.buffer, self.buff_index, b);
            written += 1;
            self.buff_index += 1;
            if self.buff_index == 8{
                if cfg!(feature = "debug_data"){
                    println!("bit writter buffer {}", &self.buffer);
                }
                self.file.write(&[self.buffer])?;
                self.buff_index = 0;
                self.buffer = 0;
            }
            
        }
        Ok(written)
    }
    pub fn new(file: File) -> Self{
        Self { file, buff_index: 0, buffer: 0 }
    }
}

impl Drop for FileBitWriter {
    fn drop(&mut self) {
        if self.buff_index != 0 {
            if cfg!(feature = "debug_data"){
                println!("dropping last writter buffer {}", &self.buffer);
            }
            self.file.write(&[self.buffer]).unwrap();
        }
    }
}

fn set_bit(byte: &mut u8, pos: u8, val: &bool){
    match val{
        &true => {
            match pos{
                0 =>{*byte |= 0b1000_0000},
                1 =>{*byte |= 0b0100_0000},
                2 =>{*byte |= 0b0010_0000},
                3 =>{*byte |= 0b0001_0000},
                4 =>{*byte |= 0b0000_1000},
                5 =>{*byte |= 0b0000_0100},
                6 =>{*byte |= 0b0000_0010},
                7 =>{*byte |= 0b0000_0001},
                _ =>{
                    unreachable!();
                }   
            }
        },
        &false => match pos{
            0 =>{*byte &= 0b0111_1111},
            1 =>{*byte &= 0b1011_1111},
            2 =>{*byte &= 0b1101_1111},
            3 =>{*byte &= 0b1110_1111},
            4 =>{*byte &= 0b1111_0111},
            5 =>{*byte &= 0b1111_1011},
            6 =>{*byte &= 0b1111_1101},
            7 =>{*byte &= 0b1111_1110},
            _ =>{
                unreachable!();
            }   
        },
    }
}