// Copyright (c) 2021 feldim2425
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::{error::Error, sync::Arc};

use wasmer::{BaseTunables, CompilerConfig, Function, Instance, JIT, Module, NativeFunc, Pages, Store, Target, imports};
use wasmer_compiler_singlepass::{Singlepass, SinglepassCompiler};
use crate::middleware::limiter::Limiter;

use super::config::VmConfig;

use super::limittubable::LimitingTunables;

pub struct VmInstance {
    store: Store
}


impl VmInstance {

    pub fn new() -> Self {
        let mut compiler = Singlepass::new();
        compiler.push_middleware(Arc::new(Limiter::new(&("computes_internal","_check_timeout"))));
        let engine = JIT::new(compiler).engine();


        let base_tunable = BaseTunables::for_target(&Target::default());
        let tunables = LimitingTunables::new(base_tunable, Pages(24)); // TODO: Dynamic Limit

        let store = Store::new_with_tunables(&engine, tunables);

        return Self {
            store: store
        }
    }
    
    fn new_with_state() -> Self {
        return Self::new();
    }

    fn state_load(&mut self, ) {

    }

    fn state_save(&self, ){

    }

    fn tick() {

    }

    fn event_dispatch() {

    }

    fn _check_timeout() -> i32 {
        println!("Check timeout!");
        return 0;
    }

    pub fn load_module(&mut self, data: impl AsRef<[u8]>) -> Result<(), Box<dyn Error>>
    {
        let module = Module::new(&self.store, data)?;
        let import_obj = imports!{
            "computes_internal" => {
                "_check_timeout" => Function::new_native(&self.store, Self::_check_timeout)
            }
        };

        let instance = Instance::new(&module, &import_obj)?;

        let run_func: NativeFunc<(),()> = instance.exports.get_native_function("run")?;

        run_func.call()?;

        return Ok(());
    }


}