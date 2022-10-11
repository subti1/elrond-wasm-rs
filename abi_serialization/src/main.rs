mod input_type;
mod output_type;
mod endpoint_type;
mod abi_type;

use crate::abi_type::AbiStruct;
use elrond_codec::top_encode_to_vec_u8;
use std::fs;
use std::env;
use std::path::Path;

fn main() {
    let json_path = Path::join(&env::current_dir().unwrap(), "src\\input.json");
    let contents = fs::read_to_string(&json_path).unwrap();
    
    let abi_struct: AbiStruct = serde_json::from_str(&contents).unwrap();

    let abi_ser = top_encode_to_vec_u8(&abi_struct).unwrap(); 

    println!("{:?}", abi_ser);
    println!("{}", String::from_utf8(abi_ser).unwrap());

    // let result_obj = AbiStruct::top_decode_or_handle_err(result_bytes, DefaultErrorHandler).unwrap();

    // println!("{}", result_obj.name);
}
