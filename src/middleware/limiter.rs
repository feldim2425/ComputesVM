// Copyright (c) 2021 feldim2425
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::{cell::{Cell, RefCell}, mem::MaybeUninit, rc::Rc, sync::{Arc, Mutex}};

/**
 * Limiter Middleware
 * Time based branch limiter, for WASM modules. 
 */

use wasmer::{FunctionMiddleware, FunctionType, LocalFunctionIndex, MiddlewareError, MiddlewareReaderState, ModuleMiddleware};
use wasmer_vm::ModuleInfo;
use wasmer_types::FunctionIndex;
use wasmer::wasmparser::{Operator, Type, TypeOrFuncType};

use crate::transformer::import::transform_add_import;

type RefCheckfun = Arc<Mutex<Option<FunctionIndex>>>;

#[derive(Debug)]
pub struct Limiter {
    checkfun: (String, String),
    checkfun_index: RefCheckfun
}

impl Limiter {
    pub fn new(check: &(String, String)) -> Self {
        return Limiter {
            checkfun: check.clone(),
            checkfun_index: Arc::new(Mutex::new(Option::None))
        };
    }
}

impl ModuleMiddleware for Limiter {
    fn generate_function_middleware(&self, _local_function_index: LocalFunctionIndex) -> Box<dyn FunctionMiddleware> {
        return Box::new(LimiterFunction{
            checkfun: self.checkfun_index.clone()
        })
    }

    fn transform_module_info(&self, info: &mut ModuleInfo) {
        let ftype = FunctionType::new(vec![], vec![wasmer::Type::I32]);
        *self.checkfun_index.lock().unwrap() = Option::Some(transform_add_import(info, self.checkfun.clone(), ftype));
        //info.imports.insert(key, value)
    }
}


#[derive(Debug)]
pub struct LimiterFunction {
    checkfun: RefCheckfun
}

impl FunctionMiddleware for LimiterFunction {
    fn feed<'a>(
        &mut self,
        operator: Operator<'a>,
        state: &mut MiddlewareReaderState<'a>,
    ) -> Result<(), MiddlewareError> {
        if let Some(val) = *self.checkfun.lock().unwrap() {
            match operator {
                Operator::Loop { .. } // loop headers are branch targets
                | Operator::End // block ends are branch targets
                | Operator::Else // "else" is the "end" of an if branch
                | Operator::Br { .. } // branch source
                | Operator::BrTable { .. } // branch source
                | Operator::BrIf { .. } // branch source
                | Operator::Call { .. } // function call - branch source
                | Operator::CallIndirect { .. } // function call - branch source
                | Operator::Return // end of function - branch source
                => {
                    state.extend(&[
                        Operator::Call { function_index: val.as_u32() },
                        Operator::If { ty: TypeOrFuncType::Type(Type::EmptyBlockType) },
                        Operator::Unreachable,
                        Operator::End
                    ])
                }

                _ => {}
            }
        }
        state.push_operator(operator);
        Ok(())
    }
}