// Copyright (c) 2021 feldim2425
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use wasmer::wat2wasm;

use crate::vm::computer::VmInstance;

const wasm1 : &'static [u8] = br#"
(module
    (type $run_t (func (param) (result)))
    (func $run (type $run_t)
       (loop (br 0))
    )
    (export "run" (func $run)) 
)
"#;


#[test]
fn test_vm() -> Result<(), Box<dyn std::error::Error>> {
    let mut instance = VmInstance::new();
    instance.load_module(wat2wasm(wasm1)?)?;

    return Ok(());
}