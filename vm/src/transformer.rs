// Copyright (c) 2021 feldim2425
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::{error::Error};

use walrus::Module;

use crate::constants;

/**
    This function takes in a array representing a WASM bytecode.
    A import is added to the WASM module for the runtime limiter function.
    The new bytecode is returned as well as the function index of the import.

    This may not be the best method to do this. It requires parsing the bytecode, modifying the data and reassembling the bytecode. 
    Ideally reassembling the bytecode would not be necessary, especially since it requires another parsing step in Wasmer afterwards.
    A method contained inside Wasmer where the modified code can be used directly in execution would be ideal. 
*/
pub(crate) fn transform_add_runlimit(data: impl AsRef<[u8]>) -> Result<(Vec<u8>, u32), Box<dyn Error>> {
    let mut premod = Module::from_buffer(data.as_ref())?;
    
    let valtype = premod.types.add(&[], &[]);
    let import_check = premod.add_import_func(constants::INTERNAL_WASM_MODULE, constants::FUNCTION_CHECKTIMEOUT, valtype);

    return Ok((premod.emit_wasm(), import_check.0.index() as u32));
}