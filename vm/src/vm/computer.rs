// Copyright (c) 2021 feldim2425
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::{borrow::{Borrow, BorrowMut}, error::Error, ops::DerefMut, sync::{Arc, Mutex, RwLock, Weak}};

use crate::{
    constants,
    errors::{VmStateError, WasmVmError},
    middleware::limiter::Limiter,
    transformer,
};

use lazy_static::lazy_static;
use wasmer::{
    imports, BaseTunables, CompilerConfig, Function, Instance, Module, NativeFunc, Pages,
    RuntimeError, Store, Target, WasmerEnv, JIT,
};
use wasmer_compiler_singlepass::Singlepass;

use super::{FnTimeoutCheck, config::VmConfig, limittubable::LimitingTunables};


/**
    A VmInstance represents one VM including, the compiler and runtime.
    Memory can be limited on a per VM basis.
    The instance handles module loading, module communication, state saving/loading and native functionality
*/
pub struct VmInstance {
    store: Store,
    instances: Vec<Box<Instance>>,
    config: VmConfig,
    fenv: FnEnv,
}

#[derive(WasmerEnv, Clone, Copy)]
struct FnEnv {
    check_fun: FnTimeoutCheck,
}

impl VmInstance {
    pub fn new(config: VmConfig) -> Self {
        let mut compiler = Singlepass::new();
        compiler.push_middleware(Arc::new(Limiter::new()));
        let engine = JIT::new(compiler).engine();

        let base_tunable = BaseTunables::for_target(&Target::default());
        let tunables = LimitingTunables::new(base_tunable, Pages(config.max_pages));

        let store = Store::new_with_tunables(&engine, tunables);

        return Self {
            store: store,
            instances: Vec::new(),
            config: config,
            fenv: FnEnv {
                check_fun: config.check_fun
            }
        };
    }

    fn new_with_state(config: VmConfig) -> Self {
        return Self::new(config);
    }

    fn state_load(&mut self) {}

    fn state_save(&self) {}

    fn tick() {}

    fn event_dispatch() {}

    pub fn load_module(&mut self, data: impl AsRef<[u8]>) -> Result<(), Box<dyn Error>> {
        let tresult = transformer::transform_add_runlimit(data)?;

        let module = Module::new(&self.store, tresult.0)?;
        let import_obj = imports! {
            constants::INTERNAL_WASM_MODULE => {
                constants::FUNCTION_CHECKTIMEOUT => Function::new_native_with_env(&self.store, self.fenv, checkfun_wrapper)
            }
        };
        self.instances
            .push(Box::new(Instance::new(&module, &import_obj)?));
        let instance = &self.instances[self.instances.len() - 1];

        let run_func: NativeFunc<(), ()> = instance.exports.get_native_function("run")?;

        let result = run_func.call();
        if let Some(err) = result.err() {
            match &err.downcast::<WasmVmError>() {
                Ok(werr) => {
                    if let WasmVmError::YieldTimeoutReached = werr {
                        println!("Too long without yielding") //TODO: Debug message
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

fn checkfun_wrapper(env: &FnEnv) {
    (env.check_fun)();
}
