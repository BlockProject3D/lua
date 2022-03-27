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

use rlua::{Context, Number};
use crate::LuaEngine;
use crate::number::{Int, Num};

fn math_clamp(_: Context, (x, min, max): (Num, Num, Num)) -> rlua::Result<Num> {
    if x.0 > max.0 {
        Ok(max)
    } else if x.0 < min.0 {
        Ok(min)
    } else {
        Ok(x)
    }
}

fn math_round(_: Context, (x, decimals): (Num, Int)) -> rlua::Result<Number> {
    let power = u32::pow(10, decimals.0 as _) as Number;
    Ok((x.0 * power).round() / power)
}

fn math_gaussian(_: Context, (sigma, x): (Num, Num)) -> rlua::Result<Number> {
    let term1 = 1.0 / 2.0 * std::f64::consts::PI * (sigma.0 * sigma.0);
    let term2 = (-(x.0 / (2.0 * sigma.0 * sigma.0))).exp();
    Ok(term1 * term2)
}

// The reason why we provide a custom math lib is to have on par implementation with rust
// and nalgebra, required for accurate rendering with the engine.
pub trait LibMath {
    fn load_math(&self) -> rlua::Result<()>;
}

impl LibMath for LuaEngine {
    fn load_math(&self) -> rlua::Result<()> {
        self.create_library("math", false, |ctx| {
            ctx.constant("PI", std::f64::consts::PI)?;
            ctx.constant("E", std::f64::consts::E)?;
            ctx.function("cos", |_, x: Num| Ok(x.0.cos()))?;
            ctx.function("sin", |_, x: Num| Ok(x.0.sin()))?;
            ctx.function("tan", |_, x: Num| Ok(x.0.tan()))?;
            ctx.function("acos", |_, x: Num| Ok(x.0.acos()))?;
            ctx.function("asin", |_, x: Num| Ok(x.0.asin()))?;
            ctx.function("atan", |_, x: Num| Ok(x.0.atan()))?;
            ctx.function("cosh", |_, x: Num| Ok(x.0.cosh()))?;
            ctx.function("sinh", |_, x: Num| Ok(x.0.sinh()))?;
            ctx.function("tanh", |_, x: Num| Ok(x.0.tanh()))?;
            ctx.function("acosh", |_, x: Num| Ok(x.0.acosh()))?;
            ctx.function("asinh", |_, x: Num| Ok(x.0.asinh()))?;
            ctx.function("atanh", |_, x: Num| Ok(x.0.atanh()))?;
            ctx.function("atan2", |_, (x, y): (Num, Num)| Ok(x.0.atan2(y.0)))?;
            ctx.function("degrees", |_, x: Num| Ok(x.0.to_degrees()))?;
            ctx.function("radians", |_, x: Num| Ok(x.0.to_radians()))?;
            ctx.function("floor", |_, x: Num| Ok(x.0.floor()))?;
            ctx.function("ceil", |_, x: Num| Ok(x.0.ceil()))?;
            ctx.function("round", |_, x: Num| Ok(x.0.round()))?;
            ctx.function("round2", math_round)?;
            ctx.function("pow", |_, (x, n): (Num, Num)| Ok(x.0.powf(n.0)))?;
            ctx.function("clamp", math_clamp)?;
            ctx.function("gaussian2d", math_gaussian)?;
            Ok(())
        })
    }
}
