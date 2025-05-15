use super::*;


pub fn define_all(import_funcs : &mut ImportFuncs) {
    define!(import_funcs, flywheel_player_get_pos,);
    define!(import_funcs, flywheel_player_set_pos,);
    define!(import_funcs, flywheel_player_get_rot,);
    define!(import_funcs, flywheel_player_set_rot,);
    define!(import_funcs, flywheel_player_get_vel,);
    define!(import_funcs, flywheel_player_set_vel,);
    define!(import_funcs, flywheel_player_send_chat,);
    define!(import_funcs, flywheel_player_send_actionbar,);
    define!(import_funcs, flywheel_player_send_title,);
    define!(import_funcs, flywheel_player_send_sound,);
}


async fn flywheel_player_get_pos(_session_id : u64, _out_x : WasmPtr<f64>, _out_y : WasmPtr<f64>, _out_z : WasmPtr<f64>) -> WasmResult<()> {
    todo!()
}

async fn flywheel_player_set_pos(_session_id : u64, _x : f64, _y : f64, _z : f64) -> WasmResult<()> {
    todo!()
}


/// Radians
async fn flywheel_player_get_rot(_session_id : u64, _out_yaw : WasmPtr<f64>, _out_pitch : WasmPtr<f64>) -> WasmResult<()> {
    todo!()
}

/// Radians
async fn flywheel_player_set_rot(_session_id : u64, _yaw : f64, _pitch : f64) -> WasmResult<()> {
    todo!()
}


async fn flywheel_player_get_vel(_session_id : u64, _out_x : WasmPtr<f64>, _out_y : WasmPtr<f64>, _out_z : WasmPtr<f64>) -> WasmResult<()> {
    todo!()
}

async fn flywheel_player_set_vel(_session_id : u64, _x : f64, _y : f64, _z : f64) -> WasmResult<()> {
    todo!()
}


/// XML text
async fn flywheel_player_send_chat(_session_id : u64, _in_msg : WasmAnyPtr, _msg_len : u32) -> WasmResult<()> {
    todo!();
}

/// XML text
async fn flywheel_player_send_actionbar(_session_id : u64, _in_msg : WasmAnyPtr, _msg_len : u32) -> WasmResult<()> {
    todo!();
}

/// XML text
#[allow(clippy::too_many_arguments)]
async fn flywheel_player_send_title(
    _session_id   : u64,
    _in_title     : WasmAnyPtr,
    _title_len    : u32,
    _in_subtitle  : WasmAnyPtr,
    _subtitle_len : u32,
    _fade_in      : u32,
    _stay         : u32,
    _fade_out     : u32
) -> WasmResult<()> {
    todo!();
}

async fn flywheel_player_send_sound(
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
