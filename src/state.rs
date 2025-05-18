use flywheelmc_common::prelude::*;


pub(crate) struct InstanceState {
    pub(crate) memory      : Option<wt::Memory>,
    pub(crate) event_queue : VecDeque<(String, Vec<u8>,)>
}
