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

use nalgebra::{Vector2, Vector3, Vector4};
use rlua::{Context, FromLua, Function, Number, ToLua, Value};
use crate::{LuaEngine, ValueExt};
use crate::number::{Num, Int, NumFromLua, NumToLua};
use crate::macros::vec_wrapper_3;
use crate::macros::vec_wrapper_2;
use crate::macros::vec_wrapper_1;
use crate::macros::vec_wrapper_2_uniform;
use crate::macros::auto_lib;

pub trait LibVector {
    fn load_vec2(&self) -> rlua::Result<()>;
    fn load_vec3(&self) -> rlua::Result<()>;
    fn load_vec4(&self) -> rlua::Result<()>;
}

const VEC2_LIB: &str = "vec2";
const VEC3_LIB: &str = "vec3";
const VEC4_LIB: &str = "vec4";

const VEC2_NEW: &str = "Vec2";
const VEC3_NEW: &str = "Vec3";
const VEC4_NEW: &str = "Vec4";

pub struct LuaVec2<T>(Vector2<T>);

impl<T> From<Vector2<T>> for LuaVec2<T> {
    fn from(v: Vector2<T>) -> Self {
        Self(v)
    }
}

impl<T> From<LuaVec2<T>> for Vector2<T> {
    fn from(v: LuaVec2<T>) -> Self {
        v.0
    }
}

impl<T> LuaVec2<T> {
    pub fn new(v: Vector2<T>) -> LuaVec2<T> {
        Self(v)
    }

    pub fn into_inner(self) -> Vector2<T> {
        self.0
    }
}

impl<'lua, T> ToLua<'lua> for LuaVec2<T>
    where T: NumToLua
{
    fn to_lua(self, lua: Context<'lua>) -> rlua::Result<Value<'lua>> {
        let func: Function = lua.globals().raw_get(VEC2_NEW)?;
        let [[x, y]] = self.0.data.0;
        func.call((x.num_to_lua(), y.num_to_lua()))
    }
}

impl<'lua, T> FromLua<'lua> for LuaVec2<T>
    where T: NumFromLua
{
    fn from_lua(lua_value: Value<'lua>, _: Context<'lua>) -> rlua::Result<Self> {
        let table = lua_value.check_table()?;
        Ok(LuaVec2(nalgebra::Vector2::new(
            T::num_from_lua(table.raw_get("x")?)?,
            T::num_from_lua(table.raw_get("y")?)?
        )))
    }
}

pub struct LuaVec3<T>(Vector3<T>);

impl<T> From<Vector3<T>> for LuaVec3<T> {
    fn from(v: Vector3<T>) -> Self {
        Self(v)
    }
}

impl<T> From<LuaVec3<T>> for Vector3<T> {
    fn from(v: LuaVec3<T>) -> Self {
        v.0
    }
}

impl<T> LuaVec3<T> {
    pub fn new(v: Vector3<T>) -> LuaVec3<T> {
        Self(v)
    }

    pub fn into_inner(self) -> Vector3<T> {
        self.0
    }
}

impl<'lua, T> ToLua<'lua> for LuaVec3<T>
    where T: NumToLua
{
    fn to_lua(self, lua: Context<'lua>) -> rlua::Result<Value<'lua>> {
        let func: Function = lua.globals().raw_get(VEC3_NEW)?;
        let [[x, y, z]] = self.0.data.0;
        func.call((x.num_to_lua(), y.num_to_lua(), z.num_to_lua()))
    }
}

impl<'lua, T> FromLua<'lua> for LuaVec3<T>
    where T: NumFromLua
{
    fn from_lua(lua_value: Value<'lua>, _: Context<'lua>) -> rlua::Result<Self> {
        let table = lua_value.check_table()?;
        Ok(LuaVec3(nalgebra::Vector3::new(
            T::num_from_lua(table.raw_get("x")?)?,
            T::num_from_lua(table.raw_get("y")?)?,
            T::num_from_lua(table.raw_get("z")?)?
        )))
    }
}

pub struct LuaVec4<T>(Vector4<T>);

impl<T> From<Vector4<T>> for LuaVec4<T> {
    fn from(v: Vector4<T>) -> Self {
        Self(v)
    }
}

impl<T> From<LuaVec4<T>> for Vector4<T> {
    fn from(v: LuaVec4<T>) -> Self {
        v.0
    }
}

impl<T> LuaVec4<T> {
    pub fn new(v: Vector4<T>) -> LuaVec4<T> {
        Self(v)
    }

    pub fn into_inner(self) -> Vector4<T> {
        self.0
    }
}

impl<'lua, T> ToLua<'lua> for LuaVec4<T>
    where T: NumToLua
{
    fn to_lua(self, lua: Context<'lua>) -> rlua::Result<Value<'lua>> {
        let func: Function = lua.globals().raw_get(VEC4_NEW)?;
        let [[x, y, z, w]] = self.0.data.0;
        func.call((x.num_to_lua(), y.num_to_lua(), z.num_to_lua(), w.num_to_lua()))
    }
}

impl<'lua, T> FromLua<'lua> for LuaVec4<T>
    where T: NumFromLua
{
    fn from_lua(lua_value: Value<'lua>, _: Context<'lua>) -> rlua::Result<Self> {
        let table = lua_value.check_table()?;
        Ok(LuaVec4(nalgebra::Vector4::new(
            T::num_from_lua(table.raw_get("x")?)?,
            T::num_from_lua(table.raw_get("y")?)?,
            T::num_from_lua(table.raw_get("z")?)?,
            T::num_from_lua(table.raw_get("w")?)?
        )))
    }
}

pub(crate) type Vec2 = LuaVec2<Number>;
pub(crate) type Vec3 = LuaVec3<Number>;
pub(crate) type Vec4 = LuaVec4<Number>;

fn argminmax_to_lua((id, val): (usize, Number)) -> (Int, Num) {
    (Int(id as _), Num(val))
}

vec_wrapper_2_uniform!(vec2_add (a, b): Vec2 => Vec2 {(a + b).into()});
vec_wrapper_2_uniform!(vec2_sub (a, b): Vec2 => Vec2 {(a - b).into()});
vec_wrapper_2_uniform!(vec2_mul (a, b): Vec2 => Vec2 {a.component_mul(&b).into()});
vec_wrapper_2_uniform!(vec2_div (a, b): Vec2 => Vec2 {a.component_div(&b).into()});
vec_wrapper_2_uniform!(vec2_le (a, b): Vec2 => bool {a <= b});
vec_wrapper_2_uniform!(vec2_lt (a, b): Vec2 => bool {a < b});
vec_wrapper_2_uniform!(vec2_eq (a, b): Vec2 => bool {a == b});
vec_wrapper_1!(vec2_unm (a: Vec2) => Vec2 {(-a).into()});
vec_wrapper_2_uniform!(vec2_dot (a, b): Vec2 => Number {a.dot(&b)});
vec_wrapper_2_uniform!(vec2_cross (a, b): Vec2 => Vec2 {a.cross(&b).into()});
vec_wrapper_1!(vec2_norm (a: Vec2) => Number {a.norm()});
vec_wrapper_1!(vec2_norm_squared (a: Vec2) => Number {a.norm_squared()});
vec_wrapper_1!(vec2_argmin (a: Vec2) => (Int, Num) {argminmax_to_lua(a.argmin())});
vec_wrapper_1!(vec2_argmax (a: Vec2) => (Int, Num) {argminmax_to_lua(a.argmax())});
vec_wrapper_1!(vec2_normalize (a: Vec2) => Vec2 {a.normalize().into()});
vec_wrapper_3!(vec2_lerp (a: Vec2, b: Vec2, f: Num) => Vec2 {a.lerp(&b.into_inner(), f.0).into()});
vec_wrapper_3!(vec2_slerp (a: Vec2, b: Vec2, f: Num) => Vec2 {a.slerp(&b.into_inner(), f.0).into()});
vec_wrapper_2!(vec2_push (a: Vec2, b: Num) => Vec3 {a.push(b.0).into()});

vec_wrapper_2_uniform!(vec3_add (a, b): Vec3 => Vec3 {(a + b).into()});
vec_wrapper_2_uniform!(vec3_sub (a, b): Vec3 => Vec3 {(a - b).into()});
vec_wrapper_2_uniform!(vec3_mul (a, b): Vec3 => Vec3 {a.component_mul(&b).into()});
vec_wrapper_2_uniform!(vec3_div (a, b): Vec3 => Vec3 {a.component_div(&b).into()});
vec_wrapper_2_uniform!(vec3_le (a, b): Vec3 => bool {a <= b});
vec_wrapper_2_uniform!(vec3_lt (a, b): Vec3 => bool {a < b});
vec_wrapper_2_uniform!(vec3_eq (a, b): Vec3 => bool {a == b});
vec_wrapper_1!(vec3_unm (a: Vec3) => Vec3 {(-a).into()});
vec_wrapper_2_uniform!(vec3_dot (a, b): Vec3 => Number {a.dot(&b)});
vec_wrapper_2_uniform!(vec3_cross (a, b): Vec3 => Vec3 {a.cross(&b).into()});
vec_wrapper_1!(vec3_norm (a: Vec3) => Number {a.norm()});
vec_wrapper_1!(vec3_norm_squared (a: Vec3) => Number {a.norm_squared()});
vec_wrapper_1!(vec3_argmin (a: Vec3) => (Int, Num) {argminmax_to_lua(a.argmin())});
vec_wrapper_1!(vec3_argmax (a: Vec3) => (Int, Num) {argminmax_to_lua(a.argmax())});
vec_wrapper_1!(vec3_normalize (a: Vec3) => Vec3 {a.normalize().into()});
#[cfg(feature = "quaternion")]
vec_wrapper_2!(vec3_rotate (a: Vec3, b: crate::quaternion::Quat) => Vec3 {
    (nalgebra::Unit::new_unchecked(b.into_inner()) * a).into()
});
vec_wrapper_3!(vec3_lerp (a: Vec3, b: Vec3, f: Num) => Vec3 {a.lerp(&b.into_inner(), f.0).into()});
vec_wrapper_3!(vec3_slerp (a: Vec3, b: Vec3, f: Num) => Vec3 {a.slerp(&b.into_inner(), f.0).into()});
vec_wrapper_2!(vec3_push (a: Vec3, b: Num) => Vec4 {a.push(b.0).into()});

vec_wrapper_2_uniform!(vec4_add (a, b): Vec4 => Vec4 {(a + b).into()});
vec_wrapper_2_uniform!(vec4_sub (a, b): Vec4 => Vec4 {(a - b).into()});
vec_wrapper_2_uniform!(vec4_mul (a, b): Vec4 => Vec4 {a.component_mul(&b).into()});
vec_wrapper_2_uniform!(vec4_div (a, b): Vec4 => Vec4 {a.component_div(&b).into()});
vec_wrapper_2_uniform!(vec4_le (a, b): Vec4 => bool {a <= b});
vec_wrapper_2_uniform!(vec4_lt (a, b): Vec4 => bool {a < b});
vec_wrapper_2_uniform!(vec4_eq (a, b): Vec4 => bool {a == b});
vec_wrapper_1!(vec4_unm (a: Vec4) => Vec4 {(-a).into()});
vec_wrapper_2_uniform!(vec4_dot (a, b): Vec4 => Number {a.dot(&b)});
vec_wrapper_2_uniform!(vec4_cross (a, b): Vec4 => Vec4 {a.cross(&b).into()});
vec_wrapper_1!(vec4_norm (a: Vec4) => Number {a.norm()});
vec_wrapper_1!(vec4_norm_squared (a: Vec4) => Number {a.norm_squared()});
vec_wrapper_1!(vec4_argmin (a: Vec4) => (Int, Num) {argminmax_to_lua(a.argmin())});
vec_wrapper_1!(vec4_argmax (a: Vec4) => (Int, Num) {argminmax_to_lua(a.argmax())});
vec_wrapper_1!(vec4_normalize (a: Vec4) => Vec4 {a.normalize().into()});
vec_wrapper_3!(vec4_lerp (a: Vec4, b: Vec4, f: Num) => Vec4 {a.lerp(&b.into_inner(), f.0).into()});
vec_wrapper_3!(vec4_slerp (a: Vec4, b: Vec4, f: Num) => Vec4 {a.slerp(&b.into_inner(), f.0).into()});

impl LibVector for LuaEngine {
    fn load_vec2(&self) -> rlua::Result<()> {
        //Create the metatable.
        auto_lib!(self (VEC2_LIB, true) {
            __add: vec2_add, __sub: vec2_sub, __mul: vec2_mul, __div: vec2_div,
            __le: vec2_le, __lt: vec2_lt, __eq: vec2_eq, __unm: vec2_unm,
            dot: vec2_dot, cross: vec2_cross, norm: vec2_norm, normSquared: vec2_norm_squared,
            argmin: vec2_argmin, argmax: vec2_argmax, lerp: vec2_lerp, slerp: vec2_slerp,
            normalize: vec2_normalize, push: vec2_push,
        })?;
        //Create constructor function.
        self.context(|ctx| {
            let function = ctx.create_function(|ctx, (x, y): (Num, Option<Num>)| {
                let val = match y {
                    Some(y) => Vector2::new(x.0, y.0),
                    None => Vector2::from_element(x.0)
                };
                let table = ctx.create_table()?;
                table.raw_set("x", val.x)?;
                table.raw_set("y", val.y)?;
                let globals = ctx.globals();
                table.set_metatable(globals.raw_get(VEC2_LIB)?);
                Ok(table)
            })?;
            let globals = ctx.globals();
            globals.raw_set(VEC2_NEW, function)?;
            Ok(())
        })
    }

    fn load_vec3(&self) -> rlua::Result<()> {
        //Create the metatable.
        auto_lib!(self (VEC3_LIB, true) {
            __add: vec3_add, __sub: vec3_sub, __mul: vec3_mul, __div: vec3_div,
            __le: vec3_le, __lt: vec3_lt, __eq: vec3_eq, __unm: vec3_unm,
            dot: vec3_dot, cross: vec3_cross, norm: vec3_norm, normSquared: vec3_norm_squared,
            argmin: vec3_argmin, argmax: vec3_argmax, lerp: vec3_lerp, slerp: vec3_slerp,
            normalize: vec3_normalize, push: vec3_push,
        })?;
        #[cfg(feature = "quaternion")]
        self.context(|ctx| {
            let tbl: rlua::Table = ctx.globals().raw_get(VEC3_LIB)?;
            tbl.raw_set("rotate", ctx.create_function(vec3_rotate)?)?;
            Ok(())
        })?;
        //Create constructor function.
        self.context(|ctx| {
            let function = ctx.create_function(|ctx, (x, y, z): (Num, Option<Num>, Option<Num>)| {
                let val = match (y, z) {
                    (Some(y), Some(z)) => Vector3::new(x.0, y.0, z.0),
                    _ => Vector3::from_element(x.0)
                };
                let table = ctx.create_table()?;
                table.raw_set("x", val.x)?;
                table.raw_set("y", val.y)?;
                table.raw_set("z", val.z)?;
                let globals = ctx.globals();
                table.set_metatable(globals.raw_get(VEC3_LIB)?);
                Ok(table)
            })?;
            let globals = ctx.globals();
            globals.raw_set(VEC3_NEW, function)?;
            Ok(())
        })
    }

    fn load_vec4(&self) -> rlua::Result<()> {
        //Create the metatable.
        auto_lib!(self (VEC4_LIB, true) {
            __add: vec4_add, __sub: vec4_sub, __mul: vec4_mul, __div: vec4_div,
            __le: vec4_le, __lt: vec4_lt, __eq: vec4_eq, __unm: vec4_unm,
            dot: vec4_dot, cross: vec4_cross, norm: vec4_norm, normSquared: vec4_norm_squared,
            argmin: vec4_argmin, argmax: vec4_argmax, lerp: vec4_lerp, slerp: vec4_slerp,
            normalize: vec4_normalize,
        })?;
        //Create constructor function.
        self.context(|ctx| {
            let function = ctx.create_function(|ctx, (x, y, z, w): (Num, Option<Num>, Option<Num>, Option<Num>)| {
                let val = match (y, z, w) {
                    (Some(y), Some(z), Some(w)) => Vector4::new(x.0, y.0, z.0, w.0),
                    _ => Vector4::from_element(x.0)
                };
                let table = ctx.create_table()?;
                table.raw_set("x", val.x)?;
                table.raw_set("y", val.y)?;
                table.raw_set("z", val.z)?;
                table.raw_set("w", val.w)?;
                let globals = ctx.globals();
                table.set_metatable(globals.raw_get(VEC4_LIB)?);
                Ok(table)
            })?;
            let globals = ctx.globals();
            globals.raw_set(VEC4_NEW, function)?;
            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::LuaEngine;
    use crate::vector::LibVector;

    #[test]
    fn basic() {
        let engine = LuaEngine::new().unwrap();
        engine.load_vec2().unwrap();
        engine.context(|ctx| {
            ctx.load(r#"
                local v = Vec2(0, 0)
                local v1 = Vec2(2.5, 2.5)
                local add = v + v1
                local mul = v * v1
                local sub = v - v1
                local div = v1 / v
                print(add.x, add.y)
                print(mul.x, mul.y)
                print(sub.x, sub.y)
                print(div.x, div.y)
                print((-div):norm())
            "#).exec()?;
            Ok(())
        }).unwrap();
    }

    #[test]
    fn mutate() {
        let engine = LuaEngine::new().unwrap();
        engine.load_vec2().unwrap();
        engine.context(|ctx| {
            ctx.load(r#"
                local v = Vec2(0.1, 0.1)
                v.x = 0
                v.y = 5
                print(v:argmax())
            "#).exec()?;
            Ok(())
        }).unwrap();
    }
}
