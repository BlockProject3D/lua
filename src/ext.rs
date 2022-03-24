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

// This module contains lua engine extensions, some of which existed in the C API.

// These functions are non standard but used to optimize 3D engines where access to numbers
// MUST be VERY fast.

use rlua::{Integer, Number, Table, Value, Result, Error};

pub trait ValueExt<'a> {
    fn check_number(self) -> Result<Number>;
    fn check_integer(self) -> Result<Integer>;
    fn check_table(self) -> Result<Table<'a>>;
    fn check_bool(self) -> Result<bool>;
    fn check_string(&self) -> Result<&str>;
}

impl<'a> ValueExt<'a> for Value<'a> {
    fn check_number(self) -> Result<Number> {
        match self {
            Value::Number(v) => Ok(v),
            Value::Integer(v) => Ok(v as Number),
            _ => Err(Error::FromLuaConversionError {
                from: self.type_name(),
                to: "Number",
                message: Some("expected number".to_string()),
            })
        }
    }

    fn check_integer(self) -> Result<Integer> {
        match self {
            Value::Integer(v) => Ok(v),
            _ => Err(Error::FromLuaConversionError {
                from: self.type_name(),
                to: "Integer",
                message: Some("expected integer".to_string()),
            })
        }
    }

    fn check_table(self) -> Result<Table<'a>> {
        match self {
            Value::Table(v) => Ok(v),
            _ => Err(Error::FromLuaConversionError {
                from: self.type_name(),
                to: "Table",
                message: Some("expected table".to_string()),
            })
        }
    }

    fn check_bool(self) -> Result<bool> {
        match self {
            Value::Boolean(v) => Ok(v),
            _ => Err(Error::FromLuaConversionError {
                from: self.type_name(),
                to: "Boolean",
                message: Some("expected boolean".to_string()),
            })
        }
    }

    fn check_string(&self) -> Result<&str> {
        match self {
            Value::String(v) => v.to_str(),
            _ => Err(Error::FromLuaConversionError {
                from: self.type_name(),
                to: "String",
                message: Some("expected string".to_string()),
            })
        }
    }
}

pub trait TableExt {
    fn enable_self_callable(&self) -> rlua::Result<()>;
}

impl<'a> TableExt for Table<'a> {
    fn enable_self_callable(&self) -> rlua::Result<()> {
        let selfcopy = self.clone();
        self.raw_set("__index", selfcopy)
    }
}
