use crate::sig::ImportFuncs;
use crate::runner::WasmCallCtx;
use crate::types::{ WasmAnyPtr, WasmResult };
use super::define;
use flywheelmc_common::prelude::*;
use flywheelmc_players::player::comms::{ PlayerCommsActionEvent, PlayerCommsAction };
use protocol::value::{ Identifier, Text };
use protocol::packet::s2c::play::SoundCategory;


pub fn define_all(import_funcs : &mut ImportFuncs) {
    //define!(import_funcs, flywheel_player_get_pos,);
    //define!(import_funcs, flywheel_player_set_pos,);
    //define!(import_funcs, flywheel_player_get_rot,);
    //define!(import_funcs, flywheel_player_set_rot,);
    //define!(import_funcs, flywheel_player_get_vel,);
    //define!(import_funcs, flywheel_player_set_vel,);
    define!(import_funcs, flywheel_player_send_chat,);
    define!(import_funcs, flywheel_player_send_actionbar,);
    define!(import_funcs, flywheel_player_send_title,);
    define!(import_funcs, flywheel_player_send_sound,);
}


// async fn flywheel_player_get_pos(_ctx : WasmCallCtx<'_>, _session_id : u64, _out_x : WasmPtr<f64>, _out_y : WasmPtr<f64>, _out_z : WasmPtr<f64>) -> WasmResult<()> {
//     todo!()
// }

// async fn flywheel_player_set_pos(_ctx : WasmCallCtx<'_>, _session_id : u64, _x : f64, _y : f64, _z : f64) -> WasmResult<()> {
//     todo!()
// }


// /// Radians
// async fn flywheel_player_get_rot(_ctx : WasmCallCtx<'_>, _session_id : u64, _out_yaw : WasmPtr<f64>, _out_pitch : WasmPtr<f64>) -> WasmResult<()> {
//     todo!()
// }

// /// Radians
// async fn flywheel_player_set_rot(_ctx : WasmCallCtx<'_>, _session_id : u64, _yaw : f64, _pitch : f64) -> WasmResult<()> {
//     todo!()
// }


// async fn flywheel_player_get_vel(_ctx : WasmCallCtx<'_>, _session_id : u64, _out_x : WasmPtr<f64>, _out_y : WasmPtr<f64>, _out_z : WasmPtr<f64>) -> WasmResult<()> {
//     todo!()
// }

// async fn flywheel_player_set_vel(_ctx : WasmCallCtx<'_>, _session_id : u64, _x : f64, _y : f64, _z : f64) -> WasmResult<()> {
//     todo!()
// }


/// XML text
async fn flywheel_player_send_chat(
    ctx        : WasmCallCtx<'_>,
    session_id : u64,
    in_msg     : WasmAnyPtr,
    msg_len    : u32
) -> WasmResult<()> {
    if let Some(entity) = ctx.player_session_to_entity(session_id).await {
        let msg = ctx.mem_read_str(in_msg, msg_len)?;
        let _ = AsyncWorld.send_event(PlayerCommsActionEvent {
            entity,
            action : PlayerCommsAction::Chat { message : Text::from_xml(msg, true, true) }
        });
    }
    Ok(())
}

/// XML text
async fn flywheel_player_send_actionbar(
    ctx        : WasmCallCtx<'_>,
    session_id : u64,
    in_msg     : WasmAnyPtr,
    msg_len    : u32
) -> WasmResult<()> {
    if let Some(entity) = ctx.player_session_to_entity(session_id).await {
        let msg = ctx.mem_read_str(in_msg, msg_len)?;
        let _ = AsyncWorld.send_event(PlayerCommsActionEvent {
            entity,
            action : PlayerCommsAction::Actionbar { message : Text::from_xml(msg, true, true) }
        });
    }
    Ok(())
}

/// XML text
#[expect(clippy::too_many_arguments)]
async fn flywheel_player_send_title(
    ctx          : WasmCallCtx<'_>,
    session_id   : u64,
    in_title     : WasmAnyPtr,
    title_len    : u32,
    in_subtitle  : WasmAnyPtr,
    subtitle_len : u32,
    fade_in      : u32,
    stay         : u32,
    fade_out     : u32
) -> WasmResult<()> {
    if let Some(entity) = ctx.player_session_to_entity(session_id).await {
        let title    = ctx.mem_read_str(in_title, title_len)?;
        let subtitle = ctx.mem_read_str(in_subtitle, subtitle_len)?;
        let _ = AsyncWorld.send_event(PlayerCommsActionEvent {
            entity,
            action : PlayerCommsAction::Title {
                title    : Text::from_xml(title, false, false),
                subtitle : Text::from_xml(subtitle, false, false),
                fade_in, stay, fade_out
            }
        });
    }
    Ok(())
}

#[expect(clippy::too_many_arguments)]
async fn flywheel_player_send_sound(
    ctx          : WasmCallCtx<'_>,
    session_id : u64,
    in_id      : WasmAnyPtr,
    id_len     : u32,
    category   : u32,
    volume     : f32,
    pitch      : f32,
    seed       : u64
) -> WasmResult<()> {
    if let Some(entity) = ctx.player_session_to_entity(session_id).await {
        let category = match (category) {
            0 => SoundCategory::Master,
            1 => SoundCategory::Music,
            2 => SoundCategory::Records,
            3 => SoundCategory::Weather,
            4 => SoundCategory::Blocks,
            5 => SoundCategory::Hostile,
            6 => SoundCategory::Neutral,
            7 => SoundCategory::Player,
            8 => SoundCategory::Ambient,
            9 => SoundCategory::Voice,
            _ => { return Ok(()); }
        };
        let id = ctx.mem_read_str(in_id, id_len)?;
        let _ = AsyncWorld.send_event(PlayerCommsActionEvent {
            entity,
            action : PlayerCommsAction::Sound {
                id       : Identifier::from(id),
                category,
                volume, pitch, seed
            }
        });
    }
    Ok(())
}
