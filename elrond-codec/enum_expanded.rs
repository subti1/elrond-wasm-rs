#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
extern crate elrond_codec_derive;
use elrond_codec_derive::*;
use elrond_codec::test_util::{
    check_dep_encode_decode, check_top_decode, check_top_encode_decode,
};
enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
impl elrond_codec::TopEncode for DayOfWeek {
    fn top_encode_or_handle_err<O, H>(
        &self,
        output: O,
        h: H,
    ) -> core::result::Result<(), H::HandledErr>
    where
        O: elrond_codec::TopEncodeOutput,
        H: elrond_codec::EncodeErrorHandler,
    {
        match self {
            DayOfWeek::Monday => {
                elrond_codec::TopEncode::top_encode_or_handle_err(&0u8, output, h)
            }
            DayOfWeek::Tuesday => {
                elrond_codec::TopEncode::top_encode_or_handle_err(&1u8, output, h)
            }
            DayOfWeek::Wednesday => {
                elrond_codec::TopEncode::top_encode_or_handle_err(&2u8, output, h)
            }
            DayOfWeek::Thursday => {
                elrond_codec::TopEncode::top_encode_or_handle_err(&3u8, output, h)
            }
            DayOfWeek::Friday => {
                elrond_codec::TopEncode::top_encode_or_handle_err(&4u8, output, h)
            }
            DayOfWeek::Saturday => {
                elrond_codec::TopEncode::top_encode_or_handle_err(&5u8, output, h)
            }
            DayOfWeek::Sunday => {
                elrond_codec::TopEncode::top_encode_or_handle_err(&6u8, output, h)
            }
        }
    }
}
impl elrond_codec::TopDecode for DayOfWeek {
    fn top_decode_or_handle_err<I, H>(
        top_input: I,
        h: H,
    ) -> core::result::Result<Self, H::HandledErr>
    where
        I: elrond_codec::TopDecodeInput,
        H: elrond_codec::DecodeErrorHandler,
    {
        if top_input.byte_len() == 0 {
            return core::result::Result::Ok(DayOfWeek::Monday);
        }
        match <u8 as elrond_codec::TopDecode>::top_decode_or_handle_err(top_input, h)? {
            0u8 => core::result::Result::Ok(DayOfWeek::Monday),
            1u8 => core::result::Result::Ok(DayOfWeek::Tuesday),
            2u8 => core::result::Result::Ok(DayOfWeek::Wednesday),
            3u8 => core::result::Result::Ok(DayOfWeek::Thursday),
            4u8 => core::result::Result::Ok(DayOfWeek::Friday),
            5u8 => core::result::Result::Ok(DayOfWeek::Saturday),
            6u8 => core::result::Result::Ok(DayOfWeek::Sunday),
            _ => {
                core::result::Result::Err(
                    h.handle_error(elrond_codec::DecodeError::INVALID_VALUE),
                )
            }
        }
    }
}
impl elrond_codec::NestedEncode for DayOfWeek {
    fn dep_encode_or_handle_err<O, H>(
        &self,
        dest: &mut O,
        h: H,
    ) -> core::result::Result<(), H::HandledErr>
    where
        O: elrond_codec::NestedEncodeOutput,
        H: elrond_codec::EncodeErrorHandler,
    {
        match self {
            DayOfWeek::Monday => {
                elrond_codec::NestedEncode::dep_encode_or_handle_err(&0u8, dest, h)?;
            }
            DayOfWeek::Tuesday => {
                elrond_codec::NestedEncode::dep_encode_or_handle_err(&1u8, dest, h)?;
            }
            DayOfWeek::Wednesday => {
                elrond_codec::NestedEncode::dep_encode_or_handle_err(&2u8, dest, h)?;
            }
            DayOfWeek::Thursday => {
                elrond_codec::NestedEncode::dep_encode_or_handle_err(&3u8, dest, h)?;
            }
            DayOfWeek::Friday => {
                elrond_codec::NestedEncode::dep_encode_or_handle_err(&4u8, dest, h)?;
            }
            DayOfWeek::Saturday => {
                elrond_codec::NestedEncode::dep_encode_or_handle_err(&5u8, dest, h)?;
            }
            DayOfWeek::Sunday => {
                elrond_codec::NestedEncode::dep_encode_or_handle_err(&6u8, dest, h)?;
            }
        };
        core::result::Result::Ok(())
    }
}
impl elrond_codec::NestedDecode for DayOfWeek {
    fn dep_decode_or_handle_err<I, H>(
        input: &mut I,
        h: H,
    ) -> core::result::Result<Self, H::HandledErr>
    where
        I: elrond_codec::NestedDecodeInput,
        H: elrond_codec::DecodeErrorHandler,
    {
        match <u8 as elrond_codec::NestedDecode>::dep_decode_or_handle_err(input, h)? {
            0u8 => core::result::Result::Ok(DayOfWeek::Monday),
            1u8 => core::result::Result::Ok(DayOfWeek::Tuesday),
            2u8 => core::result::Result::Ok(DayOfWeek::Wednesday),
            3u8 => core::result::Result::Ok(DayOfWeek::Thursday),
            4u8 => core::result::Result::Ok(DayOfWeek::Friday),
            5u8 => core::result::Result::Ok(DayOfWeek::Saturday),
            6u8 => core::result::Result::Ok(DayOfWeek::Sunday),
            _ => {
                core::result::Result::Err(
                    h.handle_error(elrond_codec::DecodeError::INVALID_VALUE),
                )
            }
        }
    }
}
impl ::core::marker::StructuralPartialEq for DayOfWeek {}
#[automatically_derived]
impl ::core::cmp::PartialEq for DayOfWeek {
    #[inline]
    fn eq(&self, other: &DayOfWeek) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag
    }
}
impl ::core::marker::StructuralEq for DayOfWeek {}
#[automatically_derived]
impl ::core::cmp::Eq for DayOfWeek {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::clone::Clone for DayOfWeek {
    #[inline]
    fn clone(&self) -> DayOfWeek {
        match self {
            DayOfWeek::Monday => DayOfWeek::Monday,
            DayOfWeek::Tuesday => DayOfWeek::Tuesday,
            DayOfWeek::Wednesday => DayOfWeek::Wednesday,
            DayOfWeek::Thursday => DayOfWeek::Thursday,
            DayOfWeek::Friday => DayOfWeek::Friday,
            DayOfWeek::Saturday => DayOfWeek::Saturday,
            DayOfWeek::Sunday => DayOfWeek::Sunday,
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for DayOfWeek {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            DayOfWeek::Monday => ::core::fmt::Formatter::write_str(f, "Monday"),
            DayOfWeek::Tuesday => ::core::fmt::Formatter::write_str(f, "Tuesday"),
            DayOfWeek::Wednesday => ::core::fmt::Formatter::write_str(f, "Wednesday"),
            DayOfWeek::Thursday => ::core::fmt::Formatter::write_str(f, "Thursday"),
            DayOfWeek::Friday => ::core::fmt::Formatter::write_str(f, "Friday"),
            DayOfWeek::Saturday => ::core::fmt::Formatter::write_str(f, "Saturday"),
            DayOfWeek::Sunday => ::core::fmt::Formatter::write_str(f, "Sunday"),
        }
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker]
pub const fieldless_enum: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("fieldless_enum"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(fieldless_enum())),
};
fn fieldless_enum() {
    check_top_encode_decode(DayOfWeek::Monday, &[]);
    check_top_encode_decode(DayOfWeek::Tuesday, &[1]);
    check_top_encode_decode(DayOfWeek::Wednesday, &[2]);
    check_top_encode_decode(DayOfWeek::Thursday, &[3]);
    check_top_encode_decode(DayOfWeek::Friday, &[4]);
    check_top_encode_decode(DayOfWeek::Saturday, &[5]);
    check_top_encode_decode(DayOfWeek::Sunday, &[6]);
    check_dep_encode_decode(DayOfWeek::Monday, &[0]);
    check_dep_encode_decode(DayOfWeek::Tuesday, &[1]);
    check_dep_encode_decode(DayOfWeek::Wednesday, &[2]);
    check_dep_encode_decode(DayOfWeek::Thursday, &[3]);
    check_dep_encode_decode(DayOfWeek::Friday, &[4]);
    check_dep_encode_decode(DayOfWeek::Saturday, &[5]);
    check_dep_encode_decode(DayOfWeek::Sunday, &[6]);
    match (&DayOfWeek::Monday, &check_top_decode(&[0])) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&DayOfWeek::Monday, &check_top_decode(&[0, 0])) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
enum EnumWithEverything {
    Quit,
    Today(DayOfWeek),
    Write(Vec<u8>, u16),
    Struct { int: u16, seq: Vec<u8>, another_byte: u8, uint_32: u32, uint_64: u64 },
}
impl elrond_codec::NestedEncode for EnumWithEverything {
    fn dep_encode_or_handle_err<O, H>(
        &self,
        dest: &mut O,
        h: H,
    ) -> core::result::Result<(), H::HandledErr>
    where
        O: elrond_codec::NestedEncodeOutput,
        H: elrond_codec::EncodeErrorHandler,
    {
        match self {
            EnumWithEverything::Quit => {
                elrond_codec::NestedEncode::dep_encode_or_handle_err(&0u8, dest, h)?;
            }
            EnumWithEverything::Today(unnamed_0) => {
                elrond_codec::NestedEncode::dep_encode_or_handle_err(&1u8, dest, h)?;
                elrond_codec::NestedEncode::dep_encode_or_handle_err(
                    &unnamed_0,
                    dest,
                    h,
                )?;
            }
            EnumWithEverything::Write(unnamed_0, unnamed_1) => {
                elrond_codec::NestedEncode::dep_encode_or_handle_err(&2u8, dest, h)?;
                elrond_codec::NestedEncode::dep_encode_or_handle_err(
                    &unnamed_0,
                    dest,
                    h,
                )?;
                elrond_codec::NestedEncode::dep_encode_or_handle_err(
                    &unnamed_1,
                    dest,
                    h,
                )?;
            }
            EnumWithEverything::Struct { int, seq, another_byte, uint_32, uint_64 } => {
                elrond_codec::NestedEncode::dep_encode_or_handle_err(&3u8, dest, h)?;
                elrond_codec::NestedEncode::dep_encode_or_handle_err(&int, dest, h)?;
                elrond_codec::NestedEncode::dep_encode_or_handle_err(&seq, dest, h)?;
                elrond_codec::NestedEncode::dep_encode_or_handle_err(
                    &another_byte,
                    dest,
                    h,
                )?;
                elrond_codec::NestedEncode::dep_encode_or_handle_err(&uint_32, dest, h)?;
                elrond_codec::NestedEncode::dep_encode_or_handle_err(&uint_64, dest, h)?;
            }
        };
        core::result::Result::Ok(())
    }
}
impl elrond_codec::NestedDecode for EnumWithEverything {
    fn dep_decode_or_handle_err<I, H>(
        input: &mut I,
        h: H,
    ) -> core::result::Result<Self, H::HandledErr>
    where
        I: elrond_codec::NestedDecodeInput,
        H: elrond_codec::DecodeErrorHandler,
    {
        match <u8 as elrond_codec::NestedDecode>::dep_decode_or_handle_err(input, h)? {
            0u8 => core::result::Result::Ok(EnumWithEverything::Quit),
            1u8 => {
                core::result::Result::Ok(
                    EnumWithEverything::Today(
                        <DayOfWeek as elrond_codec::NestedDecode>::dep_decode_or_handle_err(
                            input,
                            h,
                        )?,
                    ),
                )
            }
            2u8 => {
                core::result::Result::Ok(
                    EnumWithEverything::Write(
                        <Vec<
                            u8,
                        > as elrond_codec::NestedDecode>::dep_decode_or_handle_err(
                            input,
                            h,
                        )?,
                        <u16 as elrond_codec::NestedDecode>::dep_decode_or_handle_err(
                            input,
                            h,
                        )?,
                    ),
                )
            }
            3u8 => {
                core::result::Result::Ok(EnumWithEverything::Struct {
                    int: <u16 as elrond_codec::NestedDecode>::dep_decode_or_handle_err(
                        input,
                        h,
                    )?,
                    seq: <Vec<
                        u8,
                    > as elrond_codec::NestedDecode>::dep_decode_or_handle_err(
                        input,
                        h,
                    )?,
                    another_byte: <u8 as elrond_codec::NestedDecode>::dep_decode_or_handle_err(
                        input,
                        h,
                    )?,
                    uint_32: <u32 as elrond_codec::NestedDecode>::dep_decode_or_handle_err(
                        input,
                        h,
                    )?,
                    uint_64: <u64 as elrond_codec::NestedDecode>::dep_decode_or_handle_err(
                        input,
                        h,
                    )?,
                })
            }
            _ => {
                core::result::Result::Err(
                    h.handle_error(elrond_codec::DecodeError::INVALID_VALUE),
                )
            }
        }
    }
}
impl elrond_codec::TopEncode for EnumWithEverything {
    fn top_encode_or_handle_err<O, H>(
        &self,
        output: O,
        h: H,
    ) -> core::result::Result<(), H::HandledErr>
    where
        O: elrond_codec::TopEncodeOutput,
        H: elrond_codec::EncodeErrorHandler,
    {
        match self {
            EnumWithEverything::Quit => {
                elrond_codec::TopEncode::top_encode_or_handle_err(&0u8, output, h)
            }
            EnumWithEverything::Today(unnamed_0) => {
                let mut buffer = output.start_nested_encode();
                let dest = &mut buffer;
                elrond_codec::NestedEncode::dep_encode_or_handle_err(&1u8, dest, h)?;
                elrond_codec::NestedEncode::dep_encode_or_handle_err(
                    &unnamed_0,
                    dest,
                    h,
                )?;
                output.finalize_nested_encode(buffer);
                core::result::Result::Ok(())
            }
            EnumWithEverything::Write(unnamed_0, unnamed_1) => {
                let mut buffer = output.start_nested_encode();
                let dest = &mut buffer;
                elrond_codec::NestedEncode::dep_encode_or_handle_err(&2u8, dest, h)?;
                elrond_codec::NestedEncode::dep_encode_or_handle_err(
                    &unnamed_0,
                    dest,
                    h,
                )?;
                elrond_codec::NestedEncode::dep_encode_or_handle_err(
                    &unnamed_1,
                    dest,
                    h,
                )?;
                output.finalize_nested_encode(buffer);
                core::result::Result::Ok(())
            }
            EnumWithEverything::Struct { int, seq, another_byte, uint_32, uint_64 } => {
                let mut buffer = output.start_nested_encode();
                let dest = &mut buffer;
                elrond_codec::NestedEncode::dep_encode_or_handle_err(&3u8, dest, h)?;
                elrond_codec::NestedEncode::dep_encode_or_handle_err(&int, dest, h)?;
                elrond_codec::NestedEncode::dep_encode_or_handle_err(&seq, dest, h)?;
                elrond_codec::NestedEncode::dep_encode_or_handle_err(
                    &another_byte,
                    dest,
                    h,
                )?;
                elrond_codec::NestedEncode::dep_encode_or_handle_err(&uint_32, dest, h)?;
                elrond_codec::NestedEncode::dep_encode_or_handle_err(&uint_64, dest, h)?;
                output.finalize_nested_encode(buffer);
                core::result::Result::Ok(())
            }
        }
    }
}
impl elrond_codec::TopDecode for EnumWithEverything {
    fn top_decode_or_handle_err<I, H>(
        top_input: I,
        h: H,
    ) -> core::result::Result<Self, H::HandledErr>
    where
        I: elrond_codec::TopDecodeInput,
        H: elrond_codec::DecodeErrorHandler,
    {
        if top_input.byte_len() == 0 {
            return core::result::Result::Ok(EnumWithEverything::Quit);
        }
        let mut nested_buffer = top_input.into_nested_buffer();
        let result = match <u8 as elrond_codec::NestedDecode>::dep_decode_or_handle_err(
            &mut nested_buffer,
            h,
        )? {
            0u8 => core::result::Result::Ok(EnumWithEverything::Quit),
            1u8 => {
                core::result::Result::Ok(
                    EnumWithEverything::Today(
                        <DayOfWeek as elrond_codec::NestedDecode>::dep_decode_or_handle_err(
                            &mut nested_buffer,
                            h,
                        )?,
                    ),
                )
            }
            2u8 => {
                core::result::Result::Ok(
                    EnumWithEverything::Write(
                        <Vec<
                            u8,
                        > as elrond_codec::NestedDecode>::dep_decode_or_handle_err(
                            &mut nested_buffer,
                            h,
                        )?,
                        <u16 as elrond_codec::NestedDecode>::dep_decode_or_handle_err(
                            &mut nested_buffer,
                            h,
                        )?,
                    ),
                )
            }
            3u8 => {
                core::result::Result::Ok(EnumWithEverything::Struct {
                    int: <u16 as elrond_codec::NestedDecode>::dep_decode_or_handle_err(
                        &mut nested_buffer,
                        h,
                    )?,
                    seq: <Vec<
                        u8,
                    > as elrond_codec::NestedDecode>::dep_decode_or_handle_err(
                        &mut nested_buffer,
                        h,
                    )?,
                    another_byte: <u8 as elrond_codec::NestedDecode>::dep_decode_or_handle_err(
                        &mut nested_buffer,
                        h,
                    )?,
                    uint_32: <u32 as elrond_codec::NestedDecode>::dep_decode_or_handle_err(
                        &mut nested_buffer,
                        h,
                    )?,
                    uint_64: <u64 as elrond_codec::NestedDecode>::dep_decode_or_handle_err(
                        &mut nested_buffer,
                        h,
                    )?,
                })
            }
            _ => {
                core::result::Result::Err(
                    h.handle_error(elrond_codec::DecodeError::INVALID_VALUE),
                )
            }
        };
        if !elrond_codec::NestedDecodeInput::is_depleted(&nested_buffer) {
            return core::result::Result::Err(
                h.handle_error(elrond_codec::DecodeError::INPUT_TOO_LONG),
            );
        }
        result
    }
}
impl ::core::marker::StructuralPartialEq for EnumWithEverything {}
#[automatically_derived]
impl ::core::cmp::PartialEq for EnumWithEverything {
    #[inline]
    fn eq(&self, other: &EnumWithEverything) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag
            && match (self, other) {
                (
                    EnumWithEverything::Today(__self_0),
                    EnumWithEverything::Today(__arg1_0),
                ) => *__self_0 == *__arg1_0,
                (
                    EnumWithEverything::Write(__self_0, __self_1),
                    EnumWithEverything::Write(__arg1_0, __arg1_1),
                ) => *__self_0 == *__arg1_0 && *__self_1 == *__arg1_1,
                (
                    EnumWithEverything::Struct {
                        int: __self_0,
                        seq: __self_1,
                        another_byte: __self_2,
                        uint_32: __self_3,
                        uint_64: __self_4,
                    },
                    EnumWithEverything::Struct {
                        int: __arg1_0,
                        seq: __arg1_1,
                        another_byte: __arg1_2,
                        uint_32: __arg1_3,
                        uint_64: __arg1_4,
                    },
                ) => {
                    *__self_0 == *__arg1_0 && *__self_1 == *__arg1_1
                        && *__self_2 == *__arg1_2 && *__self_3 == *__arg1_3
                        && *__self_4 == *__arg1_4
                }
                _ => true,
            }
    }
}
impl ::core::marker::StructuralEq for EnumWithEverything {}
#[automatically_derived]
impl ::core::cmp::Eq for EnumWithEverything {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<DayOfWeek>;
        let _: ::core::cmp::AssertParamIsEq<Vec<u8>>;
        let _: ::core::cmp::AssertParamIsEq<u16>;
        let _: ::core::cmp::AssertParamIsEq<u8>;
        let _: ::core::cmp::AssertParamIsEq<u32>;
        let _: ::core::cmp::AssertParamIsEq<u64>;
    }
}
#[automatically_derived]
impl ::core::clone::Clone for EnumWithEverything {
    #[inline]
    fn clone(&self) -> EnumWithEverything {
        match self {
            EnumWithEverything::Quit => EnumWithEverything::Quit,
            EnumWithEverything::Today(__self_0) => {
                EnumWithEverything::Today(::core::clone::Clone::clone(__self_0))
            }
            EnumWithEverything::Write(__self_0, __self_1) => {
                EnumWithEverything::Write(
                    ::core::clone::Clone::clone(__self_0),
                    ::core::clone::Clone::clone(__self_1),
                )
            }
            EnumWithEverything::Struct {
                int: __self_0,
                seq: __self_1,
                another_byte: __self_2,
                uint_32: __self_3,
                uint_64: __self_4,
            } => {
                EnumWithEverything::Struct {
                    int: ::core::clone::Clone::clone(__self_0),
                    seq: ::core::clone::Clone::clone(__self_1),
                    another_byte: ::core::clone::Clone::clone(__self_2),
                    uint_32: ::core::clone::Clone::clone(__self_3),
                    uint_64: ::core::clone::Clone::clone(__self_4),
                }
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for EnumWithEverything {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            EnumWithEverything::Quit => ::core::fmt::Formatter::write_str(f, "Quit"),
            EnumWithEverything::Today(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Today", &__self_0)
            }
            EnumWithEverything::Write(__self_0, __self_1) => {
                ::core::fmt::Formatter::debug_tuple_field2_finish(
                    f,
                    "Write",
                    &__self_0,
                    &__self_1,
                )
            }
            EnumWithEverything::Struct {
                int: __self_0,
                seq: __self_1,
                another_byte: __self_2,
                uint_32: __self_3,
                uint_64: __self_4,
            } => {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "Struct",
                    "int",
                    &__self_0,
                    "seq",
                    &__self_1,
                    "another_byte",
                    &__self_2,
                    "uint_32",
                    &__self_3,
                    "uint_64",
                    &__self_4,
                )
            }
        }
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker]
pub const field_enum_zero_value: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("field_enum_zero_value"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(field_enum_zero_value())),
};
fn field_enum_zero_value() {
    check_top_encode_decode(EnumWithEverything::Quit, &[]);
    check_dep_encode_decode(EnumWithEverything::Quit, &[0]);
    match (&EnumWithEverything::Quit, &check_top_decode(&[0])) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker]
pub const field_enum_variant_with_value: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("field_enum_variant_with_value"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(
        field_enum_variant_with_value(),
    )),
};
fn field_enum_variant_with_value() {
    check_top_encode_decode(EnumWithEverything::Today(DayOfWeek::Friday), &[1, 4]);
    check_dep_encode_decode(EnumWithEverything::Today(DayOfWeek::Friday), &[1, 4]);
    let enum_tuple_0 = EnumWithEverything::Write(Vec::new(), 0);
    #[rustfmt::skip]
    let enum_tuple_0_bytes = &[2, 0, 0, 0, 0, 0, 0];
    check_top_encode_decode(enum_tuple_0.clone(), enum_tuple_0_bytes);
    check_dep_encode_decode(enum_tuple_0, enum_tuple_0_bytes);
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker]
pub const field_enum_variant_with_tuple: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("field_enum_variant_with_tuple"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(
        field_enum_variant_with_tuple(),
    )),
};
fn field_enum_variant_with_tuple() {
    let enum_tuple_1 = EnumWithEverything::Write([1, 2, 3].to_vec(), 4);
    #[rustfmt::skip]
    let enum_tuple_1_bytes = &[2, 0, 0, 0, 3, 1, 2, 3, 0, 4];
    check_top_encode_decode(enum_tuple_1.clone(), enum_tuple_1_bytes);
    check_dep_encode_decode(enum_tuple_1, enum_tuple_1_bytes);
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker]
pub const field_enum_struct_variant: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("field_enum_struct_variant"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(field_enum_struct_variant())),
};
fn field_enum_struct_variant() {
    let enum_struct = EnumWithEverything::Struct {
        int: 0x42,
        seq: <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([0x1, 0x2, 0x3, 0x4, 0x5]),
        ),
        another_byte: 0x6,
        uint_32: 0x12345,
        uint_64: 0x123456789,
    };
    #[rustfmt::skip]
    let enum_struct_bytes = &[
        3,
        0,
        0x42,
        0,
        0,
        0,
        5,
        1,
        2,
        3,
        4,
        5,
        6,
        0x00,
        0x01,
        0x23,
        0x45,
        0x00,
        0x00,
        0x00,
        0x01,
        0x23,
        0x45,
        0x67,
        0x89,
    ];
    check_top_encode_decode(enum_struct.clone(), enum_struct_bytes);
    check_dep_encode_decode(enum_struct, enum_struct_bytes);
}
#[rustc_main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(
        &[
            &fieldless_enum,
            &field_enum_zero_value,
            &field_enum_variant_with_value,
            &field_enum_variant_with_tuple,
            &field_enum_struct_variant,
        ],
    )
}
