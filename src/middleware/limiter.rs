// Copyright (c) 2021 feldim2425
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::{sync::{Arc, Mutex}};

/**
 * Limiter Middleware
 * Time based branch limiter, for WASM modules. 
 */

use wasmer::{FunctionMiddleware, LocalFunctionIndex, MiddlewareError, MiddlewareReaderState, ModuleMiddleware};
use wasmer_vm::ModuleInfo;
use wasmer_types::{FunctionIndex, ImportIndex};
use wasmer::wasmparser::{Operator, Type, TypeOrFuncType};

use crate::constants;

//use crate::transformer::import::transform_add_import;

type RefCheckfun = Arc<Mutex<Option<FunctionIndex>>>;

/**
    The Limiter is a middleware for Wasmer.
    It injects a call to the timeout check function on critical branch instructions.
    This way any function call can be forcefully stopped if the execution takes to long because of large loops.
    The middleware only works if the import for the limiter function was added to the module beforehand.
*/
#[derive(Debug)]
pub struct Limiter {
    checkfun_index: RefCheckfun
}

impl Limiter {
    pub fn new() -> Self {
        return Limiter {
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
        for f in info.imports.iter() {
            if let ImportIndex::Function(index) = f.1 {
                if constants::INTERNAL_WASM_MODULE.eq(&f.0.0) && constants::FUNCTION_CHECKTIMEOUT.eq(&f.0.1) {
                    *self.checkfun_index.lock().unwrap() = Option::Some(*index);
                    println!("{:?}", index);
                }
            }
        }

        if let Option::None = *self.checkfun_index.lock().unwrap() {
            println!("Error")
        }
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
                Operator::End // block ends are branch targets
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