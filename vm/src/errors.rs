// Copyright (c) 2021 feldim2425
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT



use custom_error::custom_error;

custom_error!{pub WasmVmError
    YieldTimeoutReached = "module ran too long without yielding",
    IllegalTimeoutState{desc: String} = "timeout check is in an illegal state: {desc}",
    Unknown = "unkown error"
}

custom_error!{pub VmStateError
    NoFunctionEnv = "vm function environment not initialized",
    Unknown = "unkown error"
}
