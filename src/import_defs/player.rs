use super::*;
use flywheelmc_players::player::comms::{ PlayerCommsActionEvent, PlayerCommsAction };
use protocol::value::Text;


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
    //define!(import_funcs, flywheel_player_send_sound,);
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
async fn flywheel_player_send_chat(ctx : WasmCallCtx<'_>, session_id : u64, in_msg : WasmAnyPtr, msg_len : u32) -> WasmResult<()> {
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
async fn flywheel_player_send_actionbar(ctx : WasmCallCtx<'_>, session_id : u64, in_msg : WasmAnyPtr, msg_len : u32) -> WasmResult<()> {
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
#[allow(clippy::too_many_arguments)]
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

#[allow(clippy::too_many_arguments)]
async fn flywheel_player_send_sound(
    _ctx          : WasmCallCtx<'_>,
    _session_id : u64,
    _in_id      : WasmAnyPtr,
    _id_len     : u32,
    _category   : u32,
    _volume     : f32,
    _pitch      : f32,
    _seed       : u64
) -> WasmResult<()> {
    todo!()
}
