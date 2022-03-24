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

macro_rules! auto_lib {
    ($lua: ident ($lib: ident, $self_callable: expr) { $($name: ident: $fn: ident,)* }) => {
        $lua.create_library($lib, $self_callable, |ctx| {
            $(
                ctx.function(stringify!($name), $fn)?;
            )*
            Ok(())
        })
    };
}

macro_rules! vec_wrapper_2 {
    ($name: ident ($this: ident: $in: ty, $other: ident: $in2: ty) => $out: ty { $code: expr }) => {
        fn $name<'a>(_: Context<'a>, (this, $other): ($in, $in2)) -> rlua::Result<$out> {
            let $this = this.into_inner();
            Ok($code)
        }
    };
}

macro_rules! vec_wrapper_3 {
    ($name: ident ($this: ident: $in: ty, $other: ident: $in2: ty, $other2: ident: $in3: ty) => $out: ty { $code: expr }) => {
        fn $name<'a>(_: Context<'a>, (this, $other, $other2): ($in, $in2, $in3)) -> rlua::Result<$out> {
            let $this = this.into_inner();
            Ok($code)
        }
    };
}

macro_rules! vec_wrapper_4 {
    ($name: ident ($this: ident: $in: ty, $other: ident: $in2: ty, $other2: ident: $in3: ty, $other3: ident: $in4: ty) => $out: ty { $code: expr }) => {
        fn $name<'a>(_: Context<'a>, (this, $other, $other2, $other3): ($in, $in2, $in3, $in4)) -> rlua::Result<$out> {
            let $this = this.into_inner();
            Ok($code)
        }
    };
}

macro_rules! vec_wrapper_1 {
    ($name: ident ($this: ident: $in: ty) => $out: ty { $code: expr }) => {
        fn $name<'a>(_: Context<'a>, this: $in) -> rlua::Result<$out> {
            let $this = this.into_inner();
            Ok($code)
        }
    };
}

macro_rules! vec_wrapper_2_uniform {
    ($name: ident ($this: ident, $other: ident): $in: ty => $out: ty { $code: expr }) => {
        fn $name<'a>(_: Context<'a>, (this, other): ($in, $in)) -> rlua::Result<$out> {
            let $this = this.into_inner();
            let $other = other.into_inner();
            Ok($code)
        }
    };
}

pub(crate) use auto_lib;
pub(crate) use vec_wrapper_1;
pub(crate) use vec_wrapper_2;
pub(crate) use vec_wrapper_3;
pub(crate) use vec_wrapper_4;
pub(crate) use vec_wrapper_2_uniform;
