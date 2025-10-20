use std::{fs::File, io::{BufReader, Read}, vec};

pub struct FileBitReader{
    pub buf_reader: BufReader<File>,
    buff_index: usize,
    buffer: u8
}

impl FileBitReader{
    pub fn read_bits(&mut self, len: usize)  -> Vec<bool> {
        let mut result = vec![];
        let requided_bits = len - (7 - self.buff_index);
        let all_bytes_output = requided_bits % 8 == 0;
        let required_bytes = (requided_bits) / 8 + {match all_bytes_output {true => 0, false => 1}};
        let mut buffer =vec![0; required_bytes];
        let _ = self.buf_reader.read_exact(&mut buffer);
        for i in self.buff_index .. 8{
            result.push((self.buffer >> i) & 1 == 1);
        }
        match all_bytes_output{
            true => {
                self.buff_index = 7;
                self.buffer = 0;
                buffer.iter().for_each(|buffer|{
                    for i in 0..8{
                        result.push((buffer >> i) & 1 == 1);
                    }
                });
            },
            false => {
                
            },
        }
        result
    }
    pub fn new(file: File) -> Self{
        Self {buf_reader: BufReader::new(file), buff_index: 0, buffer: 0 }
    }
}