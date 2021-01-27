// Copyright (c) 2021 feldim2425
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::{error::Error, sync::{Arc, Mutex}};

use crate::{constants, errors::WasmVmError, middleware::limiter::Limiter, transformer};

use wasmer::{BaseTunables, CompilerConfig, Function, Instance, JIT, Module, NativeFunc, Pages, RuntimeError, Store, Target, imports};
use wasmer_compiler_singlepass::Singlepass;
use lazy_static::lazy_static;

use super::limittubable::LimitingTunables;


//TODO: Remove this it is just for testing
lazy_static!{
    static ref c: Mutex<u8> = Mutex::new(0);
}

/**
    A VmInstance represents one VM including, the compiler and runtime. 
    Memory can be limited on a per VM basis.
    The instance handles module loading, module communication, state saving/loading and native functionality
*/
pub struct VmInstance {
    store: Store,
    
}

impl VmInstance {
    pub fn new() -> Self {
        let mut compiler = Singlepass::new();
        compiler.push_middleware(Arc::new(Limiter::new()));
        let engine = JIT::new(compiler).engine();

        let base_tunable = BaseTunables::for_target(&Target::default());
        let tunables = LimitingTunables::new(base_tunable, Pages(24)); // TODO: Dynamic Limit

        let store = Store::new_with_tunables(&engine, tunables);

        return Self { store: store };
    }

    fn new_with_state() -> Self {
        return Self::new();
    }

    fn state_load(&mut self) {}

    fn state_save(&self) {}

    fn tick() {}

    fn event_dispatch() {}

    fn _check_timeout() {
        println!("Check timeout!");
        let mut count = *c.lock().unwrap();
        count += 1;
        *c.lock().unwrap() = count;
        if count > 10 {
            RuntimeError::raise(Box::new(WasmVmError::YieldTimeoutReached));
        }
    }

    pub fn load_module(&mut self, data: impl AsRef<[u8]>) -> Result<(), Box<dyn Error>> {

        let tresult = transformer::transform_add_runlimit(data)?;

        let module = Module::new(&self.store, tresult.0)?;
        let import_obj = imports! {
            constants::INTERNAL_WASM_MODULE => {
                constants::FUNCTION_CHECKTIMEOUT => Function::new_native(&self.store, Self::_check_timeout)
            }
        };
        let instance = Instance::new(&module, &import_obj)?;

        let run_func: NativeFunc<(), ()> = instance.exports.get_native_function("run")?;

        let result = run_func.call();
        if let Some(err) = result.err() {
            match &err.downcast::<WasmVmError>() {
                Ok(werr) => {
                    if let WasmVmError::YieldTimeoutReached = werr {
                        println!("Too long without yielding")
                    }
                }

                Err(err) => {
                    return Err(Box::new(err.clone()));
                }
            }
        }

        return Ok(());
    }
}
