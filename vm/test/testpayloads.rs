// Copyright (c) 2021 feldim2425
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::borrow::BorrowMut;

use wasmer::{RuntimeError, wat2wasm};

use computes_vm::{errors::WasmVmError, vm::{computer::VmInstance, config::VmConfig}};


fn test_check_fn() {
    RuntimeError::raise(Box::new(WasmVmError::YieldTimeoutReached));
}


#[test]
fn test_vm() -> Result<(), Box<dyn std::error::Error>> {
    let code = wat2wasm(br#"
    (module
        (type $run_t (func (param) (result)))
        (func $run (type $run_t)
            (loop (br 1))
        )
        (export "run" (func $run)) 
    )
    "#)?;

    let config = VmConfig{
        max_pages: 1,
        check_fun: test_check_fn
    };


    let mut instance = VmInstance::new(config);
    instance.load_module(wat2wasm(&code)?)?;

    return Ok(());
}