use crate::runner::WasmRunnerInstance;
use flywheelmc_common::prelude::*;


#[derive(Event)]
pub struct WasmTriggerEvent {
    pub runner : Entity,
    pub event  : WasmEvent
}

pub enum WasmEvent {

    PlayerJoined {
        session_id : u64
    },
    PlayerLeft {
        session_id : u64
    }

}


pub(crate) fn trigger_events(
        q_runners  : Query<(&WasmRunnerInstance,)>,
    mut er_trigger : EventReader<WasmTriggerEvent>
) {
    for WasmTriggerEvent { runner, event } in er_trigger.read() {
        if let Ok((runner,)) = q_runners.get(*runner) {
            match (event) {

                WasmEvent::PlayerJoined { session_id } => {
                    debug!("Triggered event \"player_joined\" in WASM runner {}", runner.id.0);
                    let _ = runner.event_sender.send(("flywheel_player_joined", session_id.to_le_bytes().to_vec()));
                },

                WasmEvent::PlayerLeft { session_id } => {
                    debug!("Triggered event \"player_left\" in WASM runner {}", runner.id.0);
                    let _ = runner.event_sender.send(("flywheel_player_left", session_id.to_le_bytes().to_vec()));
                }

            }
        }
    }
}
