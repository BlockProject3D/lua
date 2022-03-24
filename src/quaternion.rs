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

use nalgebra::{Quaternion, UnitQuaternion};
use rlua::{Context, FromLua, Function, Number, ToLua, UserData, Value};
use crate::{LuaEngine, ValueExt};
use crate::number::{NumFromLua, NumToLua, Num};
use crate::macros::auto_lib;
use crate::macros::vec_wrapper_2;
use crate::macros::vec_wrapper_2_uniform;
use crate::macros::vec_wrapper_1;
use crate::macros::vec_wrapper_3;

pub trait Lib {
    fn load_quat(&self) -> rlua::Result<()>;
}

const QUAT_LIB: &str = "quat";

const QUAT_NEW: &str = "Quat";

pub struct LuaQuat<T>(Quaternion<T>);

impl<T> From<Quaternion<T>> for LuaQuat<T> {
    fn from(v: Quaternion<T>) -> Self {
        Self(v)
    }
}

impl<T> From<LuaQuat<T>> for Quaternion<T> {
    fn from(v: LuaQuat<T>) -> Self {
        v.0
    }
}

impl<T> LuaQuat<T> {
    pub fn new(v: Quaternion<T>) -> LuaQuat<T> {
        Self(v)
    }

    pub fn into_inner(self) -> Quaternion<T> {
        self.0
    }
}

impl<'lua, T> ToLua<'lua> for LuaQuat<T>
    where T: NumToLua<'lua>
{
    fn to_lua(self, lua: Context<'lua>) -> rlua::Result<Value<'lua>> {
        let func: Function = lua.globals().raw_get(QUAT_NEW)?;
        let [[x, y, z, w]] = self.0.coords.data.0;
        func.call((x.num_to_lua(), y.num_to_lua(), z.num_to_lua(), w.num_to_lua()))
    }
}

impl<'lua, T> FromLua<'lua> for LuaQuat<T>
    where T: NumFromLua<'lua>
{
    fn from_lua(lua_value: Value<'lua>, _: Context<'lua>) -> rlua::Result<Self> {
        let table = lua_value.check_table()?;
        Ok(LuaQuat(Quaternion::new(
            T::num_from_lua(table.raw_get("w")?)?,
            T::num_from_lua(table.raw_get("x")?)?,
            T::num_from_lua(table.raw_get("y")?)?,
            T::num_from_lua(table.raw_get("z")?)?
        )))
    }
}

pub struct LuaUnitQuat<T>(UnitQuaternion<T>);

pub(crate) type Quat = LuaQuat<Number>;

vec_wrapper_2_uniform!(quat_eq (a, b): Quat => bool {a == b});
vec_wrapper_2_uniform!(quat_add (a, b): Quat => Quat {(a + b).into()});
vec_wrapper_2_uniform!(quat_sub (a, b): Quat => Quat {(a - b).into()});
vec_wrapper_2_uniform!(quat_mul (a, b): Quat => Quat {(a * b).into()});
vec_wrapper_2_uniform!(quat_dot (a, b): Quat => Number {a.dot(&b)});
vec_wrapper_2_uniform!(quat_inner (a, b): Quat => Quat {a.inner(&b).into()});
vec_wrapper_2_uniform!(quat_outer (a, b): Quat => Quat {a.outer(&b).into()});
vec_wrapper_2_uniform!(quat_project (a, b): Quat => Option<Quat> {a.project(&b).map(|v| v.into())});
vec_wrapper_2_uniform!(quat_reject (a, b): Quat => Option<Quat> {a.reject(&b).map(|v| v.into())});
vec_wrapper_1!(quat_conjugate (a: Quat) => Quat {a.conjugate().into()});
vec_wrapper_1!(quat_normalize (a: Quat) => Quat {a.normalize().into()});
vec_wrapper_1!(quat_ln (a: Quat) => Quat {a.ln().into()});
vec_wrapper_1!(quat_exp (a: Quat) => Quat {a.exp().into()});
vec_wrapper_1!(quat_squared (a: Quat) => Quat {a.squared().into()});
vec_wrapper_1!(quat_half (a: Quat) => Quat {a.half().into()});
vec_wrapper_1!(quat_sqrt (a: Quat) => Quat {a.sqrt().into()});
vec_wrapper_1!(quat_ispure (a: Quat) => bool {a.is_pure()});
vec_wrapper_1!(quat_cos (a: Quat) => Quat {a.cos().into()});
vec_wrapper_1!(quat_acos (a: Quat) => Quat {a.acos().into()});
vec_wrapper_1!(quat_sin (a: Quat) => Quat {a.sin().into()});
vec_wrapper_1!(quat_asin (a: Quat) => Quat {a.asin().into()});
vec_wrapper_1!(quat_tan (a: Quat) => Quat {a.tan().into()});
vec_wrapper_1!(quat_atan (a: Quat) => Quat {a.atan().into()});
vec_wrapper_1!(quat_sinh (a: Quat) => Quat {a.sinh().into()});
vec_wrapper_1!(quat_asinh (a: Quat) => Quat {a.asinh().into()});
vec_wrapper_1!(quat_cosh (a: Quat) => Quat {a.cosh().into()});
vec_wrapper_1!(quat_acosh (a: Quat) => Quat {a.acosh().into()});
vec_wrapper_1!(quat_tanh (a: Quat) => Quat {a.tanh().into()});
vec_wrapper_1!(quat_atanh (a: Quat) => Quat {a.atanh().into()});
vec_wrapper_2!(quat_pow (a: Quat, n: Num) => Quat {a.powf(n.0).into()});
vec_wrapper_3!(quat_lerp (a: Quat, b: Quat, f: Num) => Quat {a.lerp(&b.into_inner(), f.0).into()});
vec_wrapper_1!(quat_imag (a: Quat) => crate::vector::Vec3 {a.imag().into()});
vec_wrapper_1!(quat_scalar (a: Quat) => Number {a.scalar()});
vec_wrapper_1!(quat_norm (a: Quat) => Number {a.norm()});
vec_wrapper_1!(quat_norm_squared (a: Quat) => Number {a.norm_squared()});
vec_wrapper_1!(quat_inverse (a: Quat) => Option<Quat> {a.try_inverse().map(|v| v.into())});

impl Lib for LuaEngine {
    fn load_quat(&self) -> rlua::Result<()> {
        auto_lib!(self (QUAT_LIB, true) {
            __add: quat_add, __sub: quat_sub, __mul: quat_mul, __eq: quat_eq,
            dot: quat_dot, inner: quat_inner, outer: quat_outer, project: quat_project,
            reject: quat_reject, conjugate: quat_conjugate, normalize: quat_normalize, ln: quat_ln,
            exp: quat_exp, squared: quat_squared, half: quat_half, sqrt: quat_sqrt,
            ispure: quat_ispure, cos: quat_cos, acos: quat_acos, sin: quat_sin, asin: quat_asin,
            tan: quat_tan, atan: quat_atan, sinh: quat_sinh, asinh: quat_asinh,
            cosh: quat_cosh, acosh: quat_acosh, tanh: quat_tanh, atanh: quat_atanh,
            pow: quat_pow, lerp: quat_lerp, imag: quat_imag, scalar: quat_scalar,
            norm: quat_norm, normSquared: quat_norm_squared, inverse: quat_inverse,
        })?;
        //Create constructor function.
        self.context(|ctx| {
            let function = ctx.create_function(|ctx, (x, y, z, w): (Num, Num, Num, Num)| {
                let globals = ctx.globals();
                let table = ctx.create_table()?;
                table.raw_set("x", x)?;
                table.raw_set("y", y)?;
                table.raw_set("z", z)?;
                table.raw_set("w", w)?;
                table.set_metatable(globals.raw_get(QUAT_LIB)?);
                Ok(table)
            })?;
            let globals = ctx.globals();
            globals.raw_set(QUAT_NEW, function)?;
            Ok(())
        })
    }
}
