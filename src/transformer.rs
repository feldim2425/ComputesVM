// Copyright (c) 2021 feldim2425
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::error::Error;

use walrus::{Module, ValType};

use crate::constants;

pub(crate) fn transform_add_runlimit(data: impl AsRef<[u8]>) -> Result<(Vec<u8>, u32), Box<dyn Error>> {
    let mut premod = Module::from_buffer(data.as_ref())?;
    
    let valtype = premod.types.add(&[], &[ValType::I32]);
    let import_check = premod.add_import_func(constants::FUNCTION_CHECKTIMEOUT.0, constants::FUNCTION_CHECKTIMEOUT.1, valtype);

    return Ok((premod.emit_wasm(), import_check.0.index() as u32));
}