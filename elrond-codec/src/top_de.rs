use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::num::NonZeroUsize;

use crate::codec_err::DecodeError;
use crate::nested_de::*;
use crate::top_de_input::TopDecodeInput;
use crate::transmute::*;
use crate::TypeInfo;

/// Trait that allows zero-copy read of values from an underlying API in big endian format.
///
/// 'Top' stands for the fact that values are deserialized on their own,
/// so we have the benefit of knowing their length.
/// This is useful in many scnearios, such as not having to encode Vec length and others.
///
/// The opther optimization that can be done when deserializing top-level objects
/// is using special functions from the underlying API that do some of the work for the deserializer.
/// These include getting values directly as i64/u64 or wrapping them directly into an owned Box<[u8]>.
///
/// BigInt/BigUint handling is not included here, because these are API-dependent
/// and would overly complicate the trait.
///
pub trait TopDecode: Sized {
    #[doc(hidden)]
    const TYPE_INFO: TypeInfo = TypeInfo::Unknown;

    /// Attempt to deserialize the value from input.
    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError>;

    /// Version of `top_decode` that exits quickly in case of error.
    /// Its purpose is to create smaller implementations
    /// in cases where the application is supposed to exit directly on decode error.
    fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
        input: I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        match Self::top_decode(input) {
            Ok(v) => v,
            Err(e) => exit(c, e),
        }
    }

    /// Allows types to provide optimized implementations for their boxed version.
    /// Especially designed for byte arrays that can be transmuted directly from the input sometimes.
    #[doc(hidden)]
    #[inline]
    fn top_decode_boxed<I: TopDecodeInput>(input: I) -> Result<Box<Self>, DecodeError> {
        Ok(Box::new(Self::top_decode(input)?))
    }

    #[doc(hidden)]
    #[inline]
    fn top_decode_boxed_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
        input: I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Box<Self> {
        Box::new(Self::top_decode_or_exit(input, c, exit))
    }
}

/// Top-decodes the result using the NestedDecode implementation.
pub fn top_decode_from_nested<T, I>(input: I) -> Result<T, DecodeError>
where
    I: TopDecodeInput,
    T: NestedDecode,
{
    let bytes = input.into_boxed_slice_u8();
    let mut_slice = &mut &*bytes;
    let result = T::dep_decode(mut_slice)?;
    if !mut_slice.is_empty() {
        return Err(DecodeError::INPUT_TOO_LONG);
    }
    Ok(result)
}

/// Top-decodes the result using the NestedDecode implementation.
/// Uses the fast-exit mechanism in case of error.
pub fn top_decode_from_nested_or_exit<T, I, ExitCtx: Clone>(
    input: I,
    c: ExitCtx,
    exit: fn(ExitCtx, DecodeError) -> !,
) -> T
where
    I: TopDecodeInput,
    T: NestedDecode,
{
    let bytes = input.into_boxed_slice_u8();
    let mut_slice = &mut &*bytes;
    let result = T::dep_decode_or_exit(mut_slice, c.clone(), exit);
    if !mut_slice.is_empty() {
        exit(c, DecodeError::INPUT_TOO_LONG);
    }
    result
}

impl TopDecode for () {
    const TYPE_INFO: TypeInfo = TypeInfo::Unit;

    fn top_decode<I: TopDecodeInput>(_: I) -> Result<Self, DecodeError> {
        Ok(())
    }

    fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
        _: I,
        _: ExitCtx,
        _: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
    }
}

impl<T: TopDecode> TopDecode for Box<T> {
    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        T::top_decode_boxed(input)
    }

    fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
        input: I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        T::top_decode_boxed_or_exit(input, c, exit)
    }
}

// Allowed to implement this because [T] cannot implement NestedDecode, being ?Sized.
impl<T: NestedDecode> TopDecode for Box<[T]> {
    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        if let TypeInfo::U8 = T::TYPE_INFO {
            let bytes = input.into_boxed_slice_u8();
            let cast_bytes: Box<[T]> = unsafe { core::mem::transmute(bytes) };
            Ok(cast_bytes)
        } else {
            let vec = Vec::<T>::top_decode(input)?;
            Ok(vec_into_boxed_slice(vec))
        }
    }

    /// Quick exit for any of the contained types
    fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
        input: I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        if let TypeInfo::U8 = T::TYPE_INFO {
            let bytes = input.into_boxed_slice_u8();
            let cast_bytes: Box<[T]> = unsafe { core::mem::transmute(bytes) };
            cast_bytes
        } else {
            let vec = Vec::<T>::top_decode_or_exit(input, c, exit);
            vec_into_boxed_slice(vec)
        }
    }
}

impl<T: NestedDecode> TopDecode for Vec<T> {
    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        if let TypeInfo::U8 = T::TYPE_INFO {
            let bytes = input.into_boxed_slice_u8();
            let bytes_vec = boxed_slice_into_vec(bytes);
            let cast_vec: Vec<T> = unsafe { core::mem::transmute(bytes_vec) };
            Ok(cast_vec)
        } else {
            let bytes = input.into_boxed_slice_u8();
            let mut_slice = &mut &*bytes;
            let mut result: Vec<T> = Vec::new();
            while !mut_slice.is_empty() {
                result.push(T::dep_decode(mut_slice)?);
            }
            Ok(result)
        }
    }

    fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
        input: I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        if let TypeInfo::U8 = T::TYPE_INFO {
            let bytes = input.into_boxed_slice_u8();
            let bytes_vec = boxed_slice_into_vec(bytes);
            let cast_vec: Vec<T> = unsafe { core::mem::transmute(bytes_vec) };
            cast_vec
        } else {
            let bytes = input.into_boxed_slice_u8();
            let mut_slice = &mut &*bytes;
            let mut result: Vec<T> = Vec::new();
            while !mut_slice.is_empty() {
                result.push(T::dep_decode_or_exit(mut_slice, c.clone(), exit));
            }
            result
        }
    }
}

impl TopDecode for String {
    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        let raw = Vec::<u8>::top_decode(input)?;
        match String::from_utf8(raw) {
            Ok(s) => Ok(s),
            Err(_) => Err(DecodeError::UTF8_DECODE_ERROR),
        }
    }

    fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
        input: I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        let raw = Vec::<u8>::top_decode_or_exit(input, c.clone(), exit);
        match String::from_utf8(raw) {
            Ok(s) => s,
            Err(_) => exit(c, DecodeError::UTF8_DECODE_ERROR),
        }
    }
}

impl TopDecode for Box<str> {
    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        Ok(String::top_decode(input)?.into_boxed_str())
    }

    fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
        input: I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        String::top_decode_or_exit(input, c, exit).into_boxed_str()
    }
}

impl TopDecode for bool {
    const TYPE_INFO: TypeInfo = TypeInfo::Bool;

    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        match input.into_u64() {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(DecodeError::INPUT_OUT_OF_RANGE),
        }
    }

    fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
        input: I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        match input.into_u64() {
            0 => false,
            1 => true,
            _ => exit(c, DecodeError::INPUT_OUT_OF_RANGE),
        }
    }
}

impl<T: NestedDecode> TopDecode for Option<T> {
    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        let bytes = input.into_boxed_slice_u8();
        if bytes.is_empty() {
            Ok(None)
        } else {
            let item = dep_decode_from_byte_slice::<T>(&bytes[1..])?;
            Ok(Some(item))
        }
    }

    fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
        input: I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        let bytes = input.into_boxed_slice_u8();
        if bytes.is_empty() {
            None
        } else {
            let item = dep_decode_from_byte_slice_or_exit(&bytes[1..], c, exit);
            Some(item)
        }
    }
}

impl TopDecode for NonZeroUsize {
    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        if let Some(nz) = NonZeroUsize::new(usize::top_decode(input)?) {
            Ok(nz)
        } else {
            Err(DecodeError::INVALID_VALUE)
        }
    }

    fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
        input: I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        if let Some(nz) = NonZeroUsize::new(usize::top_decode_or_exit(input, c.clone(), exit)) {
            nz
        } else {
            exit(c, DecodeError::INVALID_VALUE)
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::check_top_decode;
    use core::fmt::Debug;

    fn deser_ok<V>(element: V, bytes: &[u8])
    where
        V: TopDecode + PartialEq + Debug + 'static,
    {
        let deserialized: V = check_top_decode::<V>(&bytes[..]);
        assert_eq!(deserialized, element);
    }

    #[test]
    fn test_top_numbers_decompacted() {
        // unsigned positive
        deser_ok(5u8, &[5]);
        deser_ok(5u16, &[5]);
        deser_ok(5u32, &[5]);
        deser_ok(5u64, &[5]);
        deser_ok(5usize, &[5]);
        // signed positive
        deser_ok(5i8, &[5]);
        deser_ok(5i16, &[5]);
        deser_ok(5i32, &[5]);
        deser_ok(5i64, &[5]);
        deser_ok(5isize, &[5]);
        // signed negative
        deser_ok(-5i8, &[251]);
        deser_ok(-5i16, &[251]);
        deser_ok(-5i32, &[251]);
        deser_ok(-5i64, &[251]);
        deser_ok(-5isize, &[251]);
        // non zero usize
        deser_ok(NonZeroUsize::new(5).unwrap(), &[5]);
    }

    #[test]
    fn test_top_numbers_decompacted_2() {
        deser_ok(-1i32, &[255]);
        deser_ok(-1i32, &[255, 255]);
        deser_ok(-1i32, &[255, 255, 255, 255]);
        deser_ok(-1i64, &[255, 255, 255, 255, 255, 255, 255, 255]);
    }

    #[test]
    fn test_top_decode_str() {
        deser_ok(String::from("abc"), &[b'a', b'b', b'c']);
        deser_ok(String::from("abc").into_boxed_str(), &[b'a', b'b', b'c']);
    }
}
