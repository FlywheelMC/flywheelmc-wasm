use crate::runner::WasmRunnerInstance;
use crate::runner::event::{ WasmTriggerEvent, WasmEvent };
use flywheelmc_common::prelude::*;
use flywheelmc_players::player::Player;
use flywheelmc_players::world::{ World, PlayerInWorld };


#[derive(Component)]
pub struct PlayerWasmBinding {
    /// Query<(&WasmRunnerInstance,)>
    pub runner : Entity
}


#[derive(Event)]
pub struct PlayerBindWasm {
    /// Query<(&Player,)>
    pub player : Entity,
    /// Query<(&WasmRunnerInstance,)>
    pub runner : Entity
}

pub(crate) fn bind_players(
    mut cmds       : Commands,
    mut q_players  : Query<(&Player, &mut World,)>,
    mut q_runners  : Query<(&mut WasmRunnerInstance,)>,
    mut er_bind    : EventReader<PlayerBindWasm>,
    mut ew_trigger : EventWriter<WasmTriggerEvent>
) {
    for PlayerBindWasm { player : player_entity, runner : runner_entity } in er_bind.read() {
        if let Ok((player, mut world,)) = q_players.get_mut(*player_entity)
            && let Ok((mut runner,)) = q_runners.get_mut(*runner_entity)
        {
            info!("Bound player {} ({}) to WASM runner {}", player.username(), player.uuid(), runner.id.0);
            cmds.entity(*player_entity).insert((
                PlayerWasmBinding { runner : *runner_entity },
                PlayerInWorld,
            ));
            let session_id = runner.players.left_values().max().map_or(0, |v| v + 1);
            runner.players.insert(session_id, *player_entity);
            ew_trigger.write(WasmTriggerEvent { runner : *runner_entity, event : WasmEvent::PlayerJoined {
                session_id
            } });
        }
    }
}

// TODO: Unbind players
