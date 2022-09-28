use elrond_codec::{
    top_encode_from_nested, NestedEncodeNoErr,
    EncodeErrorHandler, NestedEncode, NestedEncodeOutput,
    TopEncode, TopEncodeOutput, top_encode_to_vec_u8,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum MyValueModel {
    Int32,
    Struct { a: u32, b: u32 },
}

impl NestedEncodeNoErr for MyValueModel {
    fn dep_encode_no_err<O: NestedEncodeOutput>(&self, dest: &mut O) {
        match self {
            MyValueModel::Int32 => {
                0i32.dep_encode_no_err(dest);
            },
            MyValueModel::Struct { a, b } => {
                a.dep_encode_no_err(dest);
                b.dep_encode_no_err(dest);
            },
        }
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

impl NestedEncode for MyValueModel {
    #[inline]
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, _h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.dep_encode_no_err(dest);
        Ok(())
    }
}

fn main() {
    //let s = MyValueModel::Struct { a: 1, b: 5 };
    //let result_bytes = top_encode_to_vec_u8(&s).unwrap(); 
    //println!("Res: {:?}", result_bytes);

    
}
