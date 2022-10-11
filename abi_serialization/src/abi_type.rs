use serde::{Deserialize, Serialize};
use elrond_codec::{
    top_decode_from_nested_or_handle_err, top_encode_from_nested, DecodeErrorHandler,
    EncodeErrorHandler, NestedDecode, NestedDecodeInput, NestedEncode, NestedEncodeOutput,
    TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput,
};

use crate::endpoint_type::Endpoint;

#[derive(Serialize, Deserialize)]
pub struct AbiStruct
{
    #[serde(default)]
    pub name: String,

    #[serde(default)]
    endpoints: Vec<Endpoint>,

    #[serde(default)]
    #[serde(rename = "hasCallback")]
    has_callback: bool, 
}

impl Default for AbiStruct {
    fn default() -> Self {
        AbiStruct {
            name: String::default(),
            endpoints: Vec::new(),
            has_callback: false,
        }
    }
}

impl NestedEncode for AbiStruct {
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.name.dep_encode_or_handle_err(dest, h)?;
        self.endpoints.as_slice().dep_encode_or_handle_err(dest, h)?;
        self.has_callback.dep_encode_or_handle_err(dest, h)?;
        Ok(())
    }
}

impl TopEncode for AbiStruct {
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        top_encode_from_nested(self, output, h)
    }
}

impl NestedDecode for AbiStruct {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(AbiStruct {
            name: String::dep_decode_or_handle_err(input, h)?,
            endpoints: Vec::<Endpoint>::dep_decode_or_handle_err(input, h)?,
            has_callback: bool::dep_decode_or_handle_err(input, h)?,
        })
    }
}

impl TopDecode for AbiStruct {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        top_decode_from_nested_or_handle_err(input, h)
    }
}