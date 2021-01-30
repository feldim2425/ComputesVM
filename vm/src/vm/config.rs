// Copyright (c) 2021 feldim2425
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use super::FnTimeoutCheck;

#[derive(Debug,Copy,Clone)]
pub struct VmConfig {
    pub max_pages: u32,
    pub check_fun: FnTimeoutCheck
}