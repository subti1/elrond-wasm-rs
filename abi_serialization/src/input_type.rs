use serde::{Deserialize, Serialize};
use elrond_codec::{
    top_decode_from_nested_or_handle_err, top_encode_from_nested, DecodeErrorHandler,
    EncodeErrorHandler, NestedDecode, NestedDecodeInput, NestedEncode, NestedEncodeOutput,
    TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput,
};

#[derive(Serialize, Deserialize)]
pub struct Input {
    #[serde(default)]
    name: String,

    #[serde(default)]
    #[serde(rename = "type")]
    field_type: String,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    multi_arg: Option<bool>,
}

impl Default for Input {
    fn default() -> Self {
        Input {
            name: String::default(),
            field_type: String::default(),
            multi_arg: None,
        }
    }
}

impl NestedEncode for Input {
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.name.dep_encode_or_handle_err(dest, h)?;
        self.field_type.dep_encode_or_handle_err(dest, h)?;
        self.multi_arg.dep_encode_or_handle_err(dest, h)?;
        Ok(())
    }
}

impl TopEncode for Input {
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        top_encode_from_nested(self, output, h)
    }
}

impl NestedDecode for Input {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(Input {
            name: String::dep_decode_or_handle_err(input, h)?,
            field_type: String::dep_decode_or_handle_err(input, h)?,
            multi_arg: None, // todo: implement for Option<bool>
        })
    }
}

impl TopDecode for Input {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        top_decode_from_nested_or_handle_err(input, h)
    }
}