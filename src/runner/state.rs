use flywheelmc_common::prelude::*;


pub(crate) struct InstanceState {
    pub(crate) runner         : Entity,
    pub(crate) memory         : Option<wt::Memory>,
    pub(crate) fn_alloc       : Option<wt::TypedFunc<(u32, u32,), u32>>,
    pub(crate) event_receiver : channel::Receiver<(&'static str, Vec<u8>,)>
}
