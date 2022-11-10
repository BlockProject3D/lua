// Copyright (c) 2022, BlockProject 3D
//
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:
//
//     * Redistributions of source code must retain the above copyright notice,
//       this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above copyright notice,
//       this list of conditions and the following disclaimer in the documentation
//       and/or other materials provided with the distribution.
//     * Neither the name of BlockProject 3D nor the names of its contributors
//       may be used to endorse or promote products derived from this software
//       without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
// EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
// PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
// PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
// LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
// NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

//! Fast implementation of numbers and integers.

use crate::ValueExt;
use num_traits::{cast, NumCast};
use rlua::Error;
use rlua::{Context, FromLua, Integer, Number, ToLua, Value};

/// Fast encodable/decodable float type with no coercion and no range checking.
#[derive(Copy, Clone, Debug, Default)]
pub struct Num(pub Number);

// Well I've looked at the implementation and extra cpu time pumped at useless ifs do not
// look good in 3D engines!
// So here we go let's get rid of these useless ifs...
// The reason why they're useless is because of the fact we're directly reading lua numbers (f64).
impl<'lua> FromLua<'lua> for Num {
    fn from_lua(lua_value: Value<'lua>, _: Context<'lua>) -> rlua::Result<Self> {
        lua_value.check_number().map(Num)
    }
}

// Here the implementation is the same. This means that for sending to lua using with Number
// or FastNumber is fine.
impl<'lua> ToLua<'lua> for Num {
    fn to_lua(self, _: Context<'lua>) -> rlua::Result<Value<'lua>> {
        Ok(Value::Number(self.0))
    }
}

/// Fast encodable/decodable integer type with no coercion and no range checking.
#[derive(Copy, Clone, Debug, Default)]
pub struct Int(pub Integer);

// Well I've looked at the implementation and extra cpu time pumped at useless ifs do not
// look good in 3D engines!
// So here we go let's get rid of these useless ifs...
// The reason why they're useless is because of the fact we're directly reading lua numbers (f64).
impl<'lua> FromLua<'lua> for Int {
    fn from_lua(lua_value: Value<'lua>, _: Context<'lua>) -> rlua::Result<Self> {
        lua_value.check_integer().map(Int)
    }
}

// Again here we're skipping quite a lot of if blocks, which are not needed again because we're
// using native lua types.
impl<'lua> ToLua<'lua> for Int {
    fn to_lua(self, _: Context<'lua>) -> rlua::Result<Value<'lua>> {
        Ok(Value::Integer(self.0))
    }
}

// These implementation are possibly truncating numbers; the use case being rendering engines,
// precision doesn't matter that much.

pub trait NumToLua {
    fn num_to_lua<'a>(self) -> Value<'a>;
}

pub trait NumFromLua
where
    Self: Sized,
{
    fn type_name() -> &'static str;
    fn num_from_lua(val: Value) -> rlua::Result<Self>;
}

macro_rules! impl_num_float {
    ($($target: ty)*) => {
        $(
        impl NumToLua for $target {
            fn num_to_lua<'a>(self) -> Value<'a> {
                Value::Number(self as Number)
            }
        }

        impl NumFromLua for $target {
            fn num_from_lua(val: Value) -> rlua::Result<Self> {
                val.check_number().map(|v| v as $target)
            }

            fn type_name() -> &'static str {
                stringify!($target)
            }
        }
        )*
    };
}

macro_rules! impl_num_int {
    ($($target: ty)*) => {
        $(
        impl NumToLua for $target {
            fn num_to_lua<'a>(self) -> Value<'a> {
                Value::Integer(self as Integer)
            }
        }

        impl NumFromLua for $target {
            fn num_from_lua(val: Value) -> rlua::Result<Self> {
                val.check_integer().map(|v| v as $target)
            }

            fn type_name() -> &'static str {
                stringify!($target)
            }
        }
        )*
    };
}

impl_num_float!(f32 f64);

impl_num_int!(
    i8 i16 i32 i64
    u8 u16 u32 u64
);

/// A range checked number which does not implement number coercion for better performance.
pub struct Checked<T>(pub T);

impl<'lua, T: NumFromLua + NumCast> FromLua<'lua> for Checked<T> {
    fn from_lua(lua_value: Value<'lua>, _: Context<'lua>) -> rlua::Result<Self> {
        match lua_value {
            Value::Integer(v) => {
                cast(v)
                    .map(Checked)
                    .ok_or_else(|| Error::FromLuaConversionError {
                        from: lua_value.type_name(),
                        to: T::type_name(),
                        message: Some("out of range".to_string()),
                    })
            }
            _ => Err(Error::FromLuaConversionError {
                from: lua_value.type_name(),
                to: T::type_name(),
                message: Some("expected integer".to_string()),
            }),
        }
    }
}

impl<'lua, T: NumToLua> ToLua<'lua> for Checked<T> {
    fn to_lua(self, _: Context<'lua>) -> rlua::Result<Value<'lua>> {
        Ok(self.0.num_to_lua())
    }
}
