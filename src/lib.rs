pub mod bit_writter;
pub mod bit_reader;
#[cfg(feature = "bool_vec_macro")]
pub extern crate bool_vec;

pub fn bool_vec_from_string(input: &str) -> Vec<bool>{
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