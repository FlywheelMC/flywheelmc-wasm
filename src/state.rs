use flywheelmc_common::prelude::*;


pub(crate) struct InstanceState {
    pub(crate) memory      : Option<wt::Memory>,
    pub(crate) fn_alloc    : Option<wt::TypedFunc<(u32, u32,), u32>>,
    pub(crate) event_queue : VecDeque<(String, Vec<u8>,)>
}
