use std::{fs::File, io::{BufReader, Read}, vec};

pub struct FileBitReader{
    pub buf_reader: BufReader<File>,
    buff_index: Option<usize>,
    buffer: u8
}

impl FileBitReader{
    pub fn read_bits(&mut self, len: usize)  -> std::io::Result<Vec<bool>> {
        let mut result = vec![];
        if cfg!(feature = "debug_data"){
            println!("self.buff_index: {:?}", self.buff_index);
        }
        let left_in_buffer = 8 - self.buff_index.unwrap_or(8);
        if len < left_in_buffer{
            for i in 0..len{
                let j = i + self.buff_index.unwrap_or(0);
                result.push((self.buffer >> 7 - j) & 1 == 1);
            }
            self.buff_index = Some(self.buff_index.unwrap_or(0) + len);
            return Ok(result);
        }
        let required_bits = len - left_in_buffer;
        let all_bytes_output = required_bits % 8 == 0;
        let required_bytes = (required_bits) / 8 + {match all_bytes_output {true => 0, false => 1}};
        let mut buffer =vec![0; required_bytes];

        if cfg!(feature = "debug_data"){
            println!("required_bits: {}", required_bits);
            println!("required_bytes: {}", required_bytes);
            println!("all_bytes_output: {}", all_bytes_output);
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
                for i in 0..required_bits % 8{
                    result.push((buffer[required_bytes - 1] >> 7 - i) & 1 == 1);
                }
                self.buff_index = Some((required_bits) % 8);
                self.buffer = buffer[required_bytes - 1];

            },
        }
        Ok(result)
    }
    pub fn new(file: File) -> Self{
        Self {buf_reader: BufReader::new(file), buff_index: None, buffer: 0 }
    }
    pub fn read_bits_binary(&mut self, len: usize)  -> std::io::Result<String>{
        let result = self.read_bits(len)?;
        let result:String = result.iter().map(|x| {match x {true => '1', false => '0'}}).collect();
        Ok(result)
    }
}