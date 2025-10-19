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
            crate::set_bit(&mut self.buffer, self.buff_index, b).unwrap();
            written += 1;
            self.buff_index += 1;
            if self.buff_index == 8{
                println!("{}", &self.buffer);
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
            println!("dropping last: {}", self.buffer);
            self.file.write(&[self.buffer]).unwrap();
        }
    }
}