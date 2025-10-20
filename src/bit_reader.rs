use std::{fs::File, io::{BufReader, Read}, vec};

pub struct FileBitReader{
    pub buf_reader: BufReader<File>,
    buff_index: Option<usize>,
    buffer: u8
}

impl FileBitReader{
    pub fn read_bits(&mut self, len: usize)  -> std::io::Result<Vec<bool>> {
        let mut result = vec![];
        let requided_bits = len - (8 - self.buff_index.unwrap_or(8));
        //let requided_bits = len - self.buff_index.unwrap_or(0);
        let all_bytes_output = requided_bits % 8 == 0;
        let required_bytes = (requided_bits) / 8 + {match all_bytes_output {true => 0, false => 1}};
        let mut buffer =vec![0; required_bytes];

        if cfg!(feature = "debug_data"){
            println!("requided_bits: {}", requided_bits);
            println!("required_bytes: {}", required_bytes);
            println!("all_bytes_output: {}", all_bytes_output);
            println!("self.buff_index: {:?}", self.buff_index);
        }

        self.buf_reader.read_exact(&mut buffer)?;
        if let Some(inx) = self.buff_index{
            for i in inx .. 8{
                result.push((self.buffer >> 7 - i) & 1 == 1);
            }
        }
        match all_bytes_output{
            true => {
                self.buff_index = None;
                self.buffer = 0;
                buffer.iter().for_each(|buffer|{
                    for i in 0..8{
                        //result.push((buffer >> i) & 1 == 1);
                        result.push((buffer >> 7 - i) & 1 == 1);
                    }
                });
            },
            false => {
                for j in 0..required_bytes - 1{
                    for i in 0..8{
                        result.push((buffer[j] >> 7 - i) & 1 == 1);
                    }
                }
                for i in 0..requided_bits % 8{
                    result.push((buffer[required_bytes - 1] >> 7 - i) & 1 == 1);
                }
                self.buff_index = Some((requided_bits) % 8);
                self.buffer = buffer[required_bytes - 1];

            },
        }
        Ok(result)
    }
    pub fn new(file: File) -> Self{
        Self {buf_reader: BufReader::new(file), buff_index: None, buffer: 0 }
    }
}