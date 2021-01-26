// Copyright (c) 2021 feldim2425
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

pub(crate) const INTERNAL_WASM_MODULE: &'static str = "_computesvm_internal";

pub(crate) const FUNCTION_CHECKTIMEOUT: (&'static str, &'static str) = (INTERNAL_WASM_MODULE, "_check_timeout");