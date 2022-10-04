use serde::{Deserialize, Serialize};
use elrond_codec::{
    top_encode_to_vec_u8, DefaultErrorHandler,
    top_decode_from_nested_or_handle_err, top_encode_from_nested, DecodeErrorHandler,
    EncodeErrorHandler, NestedDecode, NestedDecodeInput, NestedEncode, NestedEncodeOutput,
    TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput,
};
use std::fmt; // Import `fmt`


#[derive(Serialize, Deserialize)]
struct MyValueModel {
    name: String,
    int: u16,
    seq: Vec<u8>,
    uint_64: u64,
}

// the trait `fmt::Display` is implemented
impl fmt::Display for MyValueModel {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "name:{}, int:{}, seq:[{:?}], uint_64:{}", self.name, self.int, self.seq, self.uint_64)
    }
}

impl NestedEncode for MyValueModel {
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.name.dep_encode_or_handle_err(dest, h)?;
        self.int.dep_encode_or_handle_err(dest, h)?;
        self.seq.dep_encode_or_handle_err(dest, h)?;
        self.uint_64.dep_encode_or_handle_err(dest, h)?;

        Ok(())
    }
}

impl TopEncode for MyValueModel {
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        top_encode_from_nested(self, output, h)
    }
}

impl NestedDecode for MyValueModel {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(MyValueModel {
            name: String::dep_decode_or_handle_err(input, h)?,
            int: u16::dep_decode_or_handle_err(input, h)?,
            seq: Vec::<u8>::dep_decode_or_handle_err(input, h)?,
            uint_64: u64::dep_decode_or_handle_err(input, h)?,
        })
    }
}

impl TopDecode for MyValueModel {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        top_decode_from_nested_or_handle_err(input, h)
    }
}

fn main() {
    let json_data = r#"
    {
        "name": "aaa",
        "int": 20,
        "seq": [1, 2, 3, 4],
        "uint_64": 200
    }"#;

    let p: MyValueModel = serde_json::from_str(json_data).unwrap();

    let result_bytes = top_encode_to_vec_u8(&p).unwrap(); 

    println!("Res: {:?}", result_bytes);

    let result_obj = MyValueModel::top_decode_or_handle_err(result_bytes, DefaultErrorHandler).unwrap();

    println!("{}", result_obj);
}
