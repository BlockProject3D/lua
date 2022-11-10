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

use crate::TableExt;
use rlua::{Context, FromLuaMulti, Lua, StdLib, Table, ToLua, ToLuaMulti, Value};

pub struct LibContext<'a> {
    ctx: Context<'a>,
    table: Table<'a>,
}

impl<'a> LibContext<'a> {
    pub fn function<
        A: FromLuaMulti<'a>,
        R: ToLuaMulti<'a>,
        F: 'static + Send + Fn(Context, A) -> rlua::Result<R>,
    >(
        &self,
        name: &str,
        function: F,
    ) -> rlua::Result<()> {
        let func = self.ctx.create_function(function)?;
        self.table.raw_set(name, func)?;
        Ok(())
    }

    pub fn constant<T: ToLua<'a>>(&self, name: &str, val: T) -> rlua::Result<()> {
        self.table.raw_set(name, val)
    }
}

pub struct LuaEngine {
    state: Lua,
}

fn strip_potentially_dangerous(state: &Lua) -> rlua::Result<()> {
    state.context(|ctx| {
        let globals = ctx.globals();
        //Basically all these functions allows arbitrary lua code injection without source checking.
        globals.raw_set("dofile", Value::Nil)?;
        globals.raw_set("load", Value::Nil)?;
        globals.raw_set("loadfile", Value::Nil)?;
        globals.raw_set("loadstring", Value::Nil)?;
        Ok(())
    })
}

impl LuaEngine {
    pub fn new() -> rlua::Result<LuaEngine> {
        let state = Lua::new_with(
            StdLib::BASE | StdLib::UTF8 | StdLib::STRING | StdLib::TABLE | StdLib::COROUTINE,
        );
        strip_potentially_dangerous(&state)?;
        Ok(LuaEngine { state })
    }

    pub fn create_library<F: FnOnce(&LibContext) -> rlua::Result<()>>(
        &self,
        name: &str,
        self_callable: bool,
        function: F,
    ) -> rlua::Result<()> {
        self.state.context(|ctx| {
            let table = ctx.create_table()?;
            let libctx = LibContext { ctx, table };
            function(&libctx)?;
            if self_callable {
                libctx.table.enable_self_callable()?;
            }
            let globals = libctx.ctx.globals();
            globals.raw_set(name, libctx.table)?;
            Ok(())
        })
    }

    pub fn context<R, F: FnOnce(Context) -> rlua::Result<R>>(
        &self,
        function: F,
    ) -> rlua::Result<R> {
        self.state.context(function)
    }
}
