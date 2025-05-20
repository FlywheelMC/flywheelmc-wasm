use crate::runner::WasmRunnerInstance;
use crate::runner::player::PlayerWasmBinding;
use flywheelmc_common::prelude::*;
use flywheelmc_players::player::Player;
use flywheelmc_players::world::WorldChunkLoading;


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
    },

    WorldChunkLoading {
        session_id : u64,
        pos        : Vec2<i32>
    },
    WorldChunkUnloaded {
        session_id : u64,
        pos        : Vec2<i32>
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
                    trace!("Triggered event \"player_joined\" in WASM runner {}", runner.id.0);
                    let _ = runner.event_sender.force_send(("flywheel_player_joined", session_id.to_le_bytes().to_vec()));
                },
                WasmEvent::PlayerLeft { session_id } => {
                    trace!("Triggered event \"player_left\" in WASM runner {}", runner.id.0);
                    let _ = runner.event_sender.force_send(("flywheel_player_left", session_id.to_le_bytes().to_vec()));
                },

                WasmEvent::WorldChunkLoading { session_id, pos } => {
                    trace!("Triggered event \"world_chunk_loading\" in WASM runner {}", runner.id.0);
                    let mut args = Vec::with_capacity(8 + 4 + 4);
                    args.extend(session_id.to_le_bytes());
                    args.extend(pos.x.to_le_bytes());
                    args.extend(pos.y.to_le_bytes());
                    let _ = runner.event_sender.force_send(("flywheel_world_chunk_loading", args));
                },
                WasmEvent::WorldChunkUnloaded { session_id, pos } => {
                    trace!("Triggered event \"world_chunk_unloaded\" in WASM runner {}", runner.id.0);
                    let mut args = Vec::with_capacity(8 + 4 + 4);
                    args.extend(session_id.to_le_bytes());
                    args.extend(pos.x.to_le_bytes());
                    args.extend(pos.y.to_le_bytes());
                    let _ = runner.event_sender.force_send(("flywheel_world_chunk_unloaded", args));
                }

            }
        }
    }
}


pub(crate) fn load_world_chunks(
        q_players  : Query<(&Player, &PlayerWasmBinding)>,
        q_runners  : Query<(&WasmRunnerInstance,)>,
    mut er_load    : EventReader<WorldChunkLoading>,
    mut ew_trigger : EventWriter<WasmTriggerEvent>
) {
    for WorldChunkLoading { entity : player_entity, pos, .. } in er_load.read() {
        if let Ok((player, PlayerWasmBinding { runner : runner_entity },)) = q_players.get(*player_entity)
            && let Ok((runner,)) = q_runners.get(*runner_entity)
            && let Some(session_id) = runner.players.get_by_right(player_entity)
        {
            ew_trigger.write(WasmTriggerEvent { runner : *runner_entity, event : WasmEvent::WorldChunkLoading {
                session_id : *session_id, pos : *pos
            } });
        }
    }
}
