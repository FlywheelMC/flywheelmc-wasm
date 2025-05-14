use crate::state;
use flywheelmc_common::prelude::*;
use wasmtime as wt;


pub const MODULE : &'static str = "env";


pub struct ImportFuncs {
    funcs : HashMap<&'static str, ImportFunc>
}

impl ImportFuncs {
    pub(crate) fn register(&self, linker : &mut wt::Linker<state::InstanceState>) {
        for (name, func,) in &self.funcs {
            (func.register)(linker, name);
        }
    }
}

struct ImportFunc {
    register : Box<dyn Fn(&mut wt::Linker<state::InstanceState>, &'static str) + Send + Sync>
}
