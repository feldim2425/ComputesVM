// Copyright (c) 2021 feldim2425
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use wasmer::FunctionType;
use wasmer_types::{FunctionIndex, ImportIndex};
use wasmer_vm::ModuleInfo;


pub fn transform_add_import(module: &mut ModuleInfo, name: (String, String), signature: FunctionType) -> FunctionIndex {
    let index = module.imports.len() as u32;
    let sig_index= module.signatures.push(signature);
    let func_index = module.functions.push(sig_index);
    module.num_imported_functions += 1;
    module.imports.insert((name.0, name.1, index), ImportIndex::Function(func_index));
    return func_index;
}