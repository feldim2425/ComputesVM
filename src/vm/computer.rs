// Copyright (c) 2021 feldim2425
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use wasmer::{BaseTunables, Instance, JIT, Pages, Store, Target};
use wasmer_compiler_singlepass::{Singlepass, SinglepassCompiler};
use super::config::VmConfig;

use super::limittubable::LimitingTunables;

struct VmInstance {
    store: Store
}


impl VmInstance {

    fn new(config: VmConfig) -> Self {
        let compiler = Singlepass::new();
        let engine = JIT::new(compiler).engine();


        let base_tunable = BaseTunables::for_target(&Target::default());
        let tubables = LimitingTunables::new(base_tunable, Pages(24)); // TODO: Dynamic Limit

        let store = Store::new_with_tunables(&engine, tubables);

        return Self {
            store: store
        }
    }
    
    fn new_with_state(config: VmConfig) -> Self {
        return Self::new(config);
    }

    fn state_load(&mut self, ) {

    }

    fn state_save(&self, ){

    }

    fn tick() {

    }

    fn event_dispatch() {

    }


}