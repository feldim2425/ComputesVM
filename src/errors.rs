// Copyright (c) 2021 feldim2425
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT



use custom_error::custom_error;

custom_error!{pub WasmVmError
    YieldTimeoutReached = "module ran too long without yielding",
    Unknown = "unkown error"
}
