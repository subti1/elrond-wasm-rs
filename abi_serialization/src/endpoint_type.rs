use serde::{Deserialize, Serialize};
use elrond_codec::{
    top_decode_from_nested_or_handle_err, top_encode_from_nested, DecodeErrorHandler,
    EncodeErrorHandler, NestedDecode, NestedDecodeInput, NestedEncode, NestedEncodeOutput,
    TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput,
};

use crate::input_type::Input;
use crate::output_type::Output;

#[derive(Serialize, Deserialize)]
pub struct Endpoint
{
    #[serde(default)]
    name: String,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    mutability: Option<String>,

    #[serde(default)]
    inputs: Vec<Input>,

    #[serde(default)]
    outputs: Vec<Output>,
}

impl Default for Endpoint {
    fn default() -> Self {
        Endpoint {
            name: String::default(),
            mutability: None,
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }
}

impl NestedEncode for Endpoint {
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.name.dep_encode_or_handle_err(dest, h)?;
        self.inputs.as_slice().dep_encode_or_handle_err(dest, h)?;
        self.outputs.as_slice().dep_encode_or_handle_err(dest, h)?;
        Ok(())
    }
}

impl TopEncode for Endpoint {
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        top_encode_from_nested(self, output, h)
    }
}

impl NestedDecode for Endpoint {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(Endpoint {
            name: String::dep_decode_or_handle_err(input, h)?,
            mutability: None,
            inputs: Vec::<Input>::dep_decode_or_handle_err(input, h)?,
            outputs: Vec::<Output>::dep_decode_or_handle_err(input, h)?,
        })
    }
}

impl TopDecode for Endpoint {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        top_decode_from_nested_or_handle_err(input, h)
    }
}