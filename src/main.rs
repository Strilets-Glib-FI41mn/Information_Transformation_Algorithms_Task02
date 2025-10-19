use std::fs::File;

use bit_writter::FileBitWriter;
mod bit_writter;

fn main()  {
    println!("Hello, world!");
    let file = File::create("foo").unwrap();
    let mut file_writter = FileBitWriter::new(file);
    /*
    let r1 = file_writter.write(vec![false, false, true, true, false, false, true, true,
        true]);
    let r2 = file_writter.write(vec![true, true, true, true, true, true, true, true,
            true]);

    println!("{:?} {:?}", &r1, &r2);
    */

    file_writter.write_bits(vec![false, true, true, false, false, false, false, true]);
    file_writter.write_bits(vec![true, true, true, true, true, true, true, true]);
    //file_writter.write(vec![true, true, true, true, true, true, true, true, true]);
    file_writter.write_bits(vec![false, true, true]);
    file_writter.write_bits(vec![true, true, true]);

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