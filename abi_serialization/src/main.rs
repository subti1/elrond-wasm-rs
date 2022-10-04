use serde::{Deserialize, Serialize};
use elrond_codec::{
    top_encode_from_nested, top_encode_to_vec_u8,
    EncodeErrorHandler, NestedEncode, NestedEncodeOutput,
    TopEncode, TopEncodeOutput,
};


#[derive(Serialize, Deserialize)]
struct MyValueModel {
    name: String,
    int: u16,
    seq: Vec<u8>,
    uint_64: u64,
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
}
