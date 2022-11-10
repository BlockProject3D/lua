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

use crate::number::{Checked, Int, Num};
use crate::LuaEngine;
use noise::Billow;
use noise::NoiseFn;
use noise::Perlin;
use noise::Seedable;
use rlua::UserData;
use rlua::UserDataMethods;

pub trait LibNoise {
    fn load_noise(&self) -> rlua::Result<()>;
}

struct LuaPerlin(Perlin);

impl UserData for LuaPerlin {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_method("getSeed", |_, this, ()| Ok(Checked(this.0.seed())));
        methods.add_method("sample2d", |_, this, (x, y): (Num, Num)| {
            Ok(this.0.get([x.0, y.0]))
        });
        methods.add_method("sample3d", |_, this, (x, y, z): (Num, Num, Num)| {
            Ok(this.0.get([x.0, y.0, z.0]))
        });
        methods.add_method("sample4d", |_, this, (x, y, z, w): (Num, Num, Num, Num)| {
            Ok(this.0.get([x.0, y.0, z.0, w.0]))
        });
    }
}

struct LuaBillow(Billow);

impl UserData for LuaBillow {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_method("getSeed", |_, this, ()| Ok(Checked(this.0.seed())));
        methods.add_method("sample2d", |_, this, (x, y): (Num, Num)| {
            Ok(this.0.get([x.0, y.0]))
        });
        methods.add_method("sample3d", |_, this, (x, y, z): (Num, Num, Num)| {
            Ok(this.0.get([x.0, y.0, z.0]))
        });
        methods.add_method("sample4d", |_, this, (x, y, z, w): (Num, Num, Num, Num)| {
            Ok(this.0.get([x.0, y.0, z.0, w.0]))
        });
        methods.add_method("getOctaves", |_, this, ()| Ok(Int(this.0.octaves as _)));
        methods.add_method("getFrequency", |_, this, ()| Ok(this.0.frequency));
        methods.add_method("getLacunarity", |_, this, ()| Ok(this.0.lacunarity));
        methods.add_method("getPersistence", |_, this, ()| Ok(this.0.persistence));
        methods.add_method_mut("setOctaves", |_, this, x: Int| {
            this.0.octaves = x.0 as _;
            Ok(())
        });
        methods.add_method_mut("setFrequency", |_, this, x: Num| {
            this.0.frequency = x.0;
            Ok(())
        });
        methods.add_method_mut("setLacunarity", |_, this, x: Num| {
            this.0.lacunarity = x.0;
            Ok(())
        });
        methods.add_method_mut("setPersistence", |_, this, x: Num| {
            this.0.persistence = x.0;
            Ok(())
        });
    }
}

impl LibNoise for LuaEngine {
    fn load_noise(&self) -> rlua::Result<()> {
        self.create_library("noise", false, |ctx| {
            ctx.function("perlin", |_, seed: Option<Checked<u32>>| {
                Ok(LuaPerlin(match seed {
                    None => Perlin::new(),
                    Some(seed) => Perlin::new().set_seed(seed.0),
                }))
            })?;
            ctx.function("billow", |_, seed: Option<Checked<u32>>| {
                Ok(LuaBillow(match seed {
                    None => Billow::new(),
                    Some(seed) => Billow::new().set_seed(seed.0),
                }))
            })?;
            Ok(())
        })
    }
}
