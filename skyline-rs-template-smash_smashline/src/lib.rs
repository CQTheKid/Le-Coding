#![feature(concat_idents, proc_macro_hygiene)]
#![allow(unused_macros)]

use csk::*;
use param_config::*;
use smash::hash40;
use smash::lib::lua_const::FIGHTER_KIND_WOLF;
use smash::lib::lua_const::WEAPON_KIND_FALCO_BLASTER_BULLET;
use std::collections::HashMap;

mod wolf;

pub static mut MARKED_COLORS: [bool; 256] = [false; 256];
pub static mut LAST_COLOR: i32 = -1;
pub static mut FIGHTER_WOLF_GENERATE_ARTICLE_PLASMAPULSE: i32 = 6;

extern "C" fn mods_mounted(_ev: arcropolis_api::Event) {
    const FIGHTER_NAME: &str = "wolf";
    const MARKER_FILE: &str = "msonic.marker";
    let mut lowest_color: i32 = -1;
    let mut marked_slots: Vec<i32> = vec![];
    for x in 0..256 {
        if let Ok(_) = std::fs::read(format!(
            "mods:/fighter/{}/model/body/c{:02}/{}",
            FIGHTER_NAME, x, MARKER_FILE
        )) {
            unsafe {
                marked_slots.push(x as _);
                MARKED_COLORS[x as usize] = true;
                if lowest_color == -1 {
                    lowest_color = x as _;
                }
            }
        }
    }

    if lowest_color == -1 {
        // if no marker exist, leave
        return;
    }

    let color_num = {
        unsafe {
            let mut index = lowest_color;
            while index < 256 && MARKED_COLORS[index as usize] {
                index += 1;
            }
            LAST_COLOR = index - 1;
            index - lowest_color
        }
    };

    // Param Edits

    // Param changes template
    // General params
    update_float_2(
        *FIGHTER_KIND_WOLF,
        marked_slots.clone(),
        (hash40("weight"), 0, 78.0),
    );
    update_float_2(
        *FIGHTER_KIND_WOLF,
        marked_slots.clone(),
        (hash40("walk_accel_mul"), 0, 0.100),
    );
    update_float_2(
        *FIGHTER_KIND_WOLF,
        marked_slots.clone(),
        (hash40("walk_accel_add"), 0, 0.100),
    );
    update_float_2(
        *FIGHTER_KIND_WOLF,
        marked_slots.clone(),
        (hash40("walk_speed_max"), 0, 1.366),
    );
    update_float_2(
        *FIGHTER_KIND_WOLF,
        marked_slots.clone(),
        (hash40("dash_speed"), 0, 2.24),
    );
    update_float_2(
        *FIGHTER_KIND_WOLF,
        marked_slots.clone(),
        (hash40("run_accel_mul"), 0, 0.157),
    );
    update_float_2(
        *FIGHTER_KIND_WOLF,
        marked_slots.clone(),
        (hash40("run_accel_add"), 0, 0.042),
    );
    update_float_2(
        *FIGHTER_KIND_WOLF,
        marked_slots.clone(),
        (hash40("run_speed_max"), 0, 3.5),
    );
    // Jump params
    update_float_2(
        *FIGHTER_KIND_WOLF,
        marked_slots.clone(),
        (hash40("jump_speed_x"), 0, 0.9),
    );
    update_float_2(
        *FIGHTER_KIND_WOLF,
        marked_slots.clone(),
        (hash40("jump_speed_x_mul"), 0, 0.75),
    );
    update_float_2(
        *FIGHTER_KIND_WOLF,
        marked_slots.clone(),
        (hash40("jump_speed_x_max"), 0, 1.95),
    );
    update_float_2(
        *FIGHTER_KIND_WOLF,
        marked_slots.clone(),
        (hash40("jump_initial_y"), 0, 18.0),
    );
    update_float_2(
        *FIGHTER_KIND_WOLF,
        marked_slots.clone(),
        (hash40("jump_y"), 0, 32.0),
    );
    update_float_2(
        *FIGHTER_KIND_WOLF,
        marked_slots.clone(),
        (hash40("mini_jump_y"), 0, 15.0),
    );
    update_float_2(
        *FIGHTER_KIND_WOLF,
        marked_slots.clone(),
        (hash40("jump_aerial_y"), 0, 16.0),
    );
    // Air params
    update_float_2(
        *FIGHTER_KIND_WOLF,
        marked_slots.clone(),
        (hash40("air_accel_y"), 0, 0.12),
    );
    update_float_2(
        *FIGHTER_KIND_WOLF,
        marked_slots.clone(),
        (hash40("air_accel_x_mul"), 0, 0.065),
    );
    update_float_2(
        *FIGHTER_KIND_WOLF,
        marked_slots.clone(),
        (hash40("air_accel_x_add"), 0, 0.02),
    );
    update_float_2(
        *FIGHTER_KIND_WOLF,
        marked_slots.clone(),
        (hash40("air_speed_x_stable"), 0, 1.2),
    );
    update_float_2(
        *FIGHTER_KIND_WOLF,
        marked_slots.clone(),
        (hash40("air_brake_x"), 0, 0.015),
    );
    update_float_2(
        *FIGHTER_KIND_WOLF,
        marked_slots.clone(),
        (hash40("air_brake_y"), 0, 0.108),
    );
    update_float_2(
        *FIGHTER_KIND_WOLF,
        marked_slots.clone(),
        (hash40("fly_speed_y_mul"), 0, 1.5),
    );
    // Special Params
    update_float_2(
        *FIGHTER_KIND_WOLF,
        marked_slots.clone(),
        (
            hash40("param_special_s"),
            hash40("air_end_speed_x_stable_mul"),
            0.0,
        ),
    );
    //Up Special (Overdrive)
    update_float_2(
        *FIGHTER_KIND_WOLF,
        marked_slots.clone(),
        (hash40("param_special_hi"), hash40("fire_rush_speed"), 4.73),
    );
    update_float_2(
        *FIGHTER_KIND_WOLF,
        marked_slots.clone(),
        (
            hash40("param_special_hi"),
            hash40("fire_landing_frame"),
            25.0,
        ),
    );
    //PlasmaPulse
    update_float_2(
        *FIGHTER_KIND_WOLF,
        marked_slots.clone(),
        (hash40("param_blaster_bullet"), hash40("life"), 27.0),
    );
    // 3 Jabs params
    update_int_2(
        *FIGHTER_KIND_WOLF,
        marked_slots.clone(),
        (hash40("attack_combo_max"), 0, 3),
    );
    update_float_2(
        *FIGHTER_KIND_WOLF,
        marked_slots.clone(),
        (hash40("combo_attack_12_end"), 0, 25.0),
    );
    update_float_2(
        *FIGHTER_KIND_WOLF,
        marked_slots.clone(),
        (hash40("combo_attack_13_end"), 0, 32.0),
    );
    // Kirby_copy param
    update_int_2(
        *FIGHTER_KIND_WOLF,
        marked_slots.clone(),
        (hash40("kirby_cant_copy"), 0, 0),
    );
    // Villager_pocket params
    disable_villager_pocket(*FIGHTER_KIND_WOLF, marked_slots.clone(), 0);

    // CSK Stuff

    add_narration_characall_entry("vc_narration_characall_msonic");

    add_chara_db_entry_info(CharacterDatabaseEntry {
        ui_chara_id: hash40("ui_chara_msonic"),
        clone_from_ui_chara_id: Some(hash40("ui_chara_wolf")),
        name_id: StringType::Overwrite(CStrCSK::new("msonic")),
        ui_series_id: Hash40Type::Overwrite(hash40("ui_series_sonic")),
        disp_order: SignedByteType::Overwrite(40),
        is_dlc: BoolType::Overwrite(false),
        is_patch: BoolType::Overwrite(false),
        color_num: UnsignedByteType::Overwrite(color_num as u8),
        extra_index_maps: UnsignedByteMap::Overwrite(HashMap::from([(
            hash40("color_start_index"),
            UnsignedByteType::Overwrite(lowest_color as u8),
        )])),
        extra_hash_maps: Hash40Map::Overwrite(HashMap::from([
            (
                hash40("characall_label_c00"),
                Hash40Type::Overwrite(hash40("vc_narration_characall_msonic")),
            ),
            (
                hash40("original_ui_chara_hash"),
                Hash40Type::Overwrite(hash40("ui_chara_sonic")),
            ),
        ])),
        ..Default::default()
    });

    add_chara_layout_db_entry_info(CharacterLayoutDatabaseEntry {
        ui_layout_id: hash40("ui_chara_msonic_00"),
        clone_from_ui_layout_id: Some(hash40("ui_chara_wolf_00")),
        ui_chara_id: Hash40Type::Overwrite(hash40("ui_chara_msonic")),
        ..Default::default()
    });

    add_series_db_entry_info(SeriesDatabaseEntry {
        ui_series_id: hash40("ui_series_customseries"),
        name_id: StringType::Overwrite(CStrCSK::new("customseries")),
        disp_order: SignedByteType::Overwrite(0),
        disp_order_sound: SignedByteType::Overwrite(0),
        save_no: SignedByteType::Overwrite(0),
        shown_as_series_in_directory: BoolType::Overwrite(false),
        is_dlc: BoolType::Overwrite(false),
        is_patch: BoolType::Overwrite(false),
        dlc_chara_id: Hash40Type::Overwrite(0),
        is_use_amiibo_bg: BoolType::Overwrite(false),
        ..Default::default()
    });
}

#[skyline::main(name = "smashline_test")]
pub fn main() {
    unsafe {
        extern "C" {
            fn arcrop_register_event_callback(
                ty: arcropolis_api::Event,
                callback: arcropolis_api::EventCallbackFn,
            );
        }
        arcrop_register_event_callback(arcropolis_api::Event::ModFilesystemMounted, mods_mounted);
    }
    wolf::install();
    unsafe {
        FIGHTER_WOLF_GENERATE_ARTICLE_PLASMAPULSE += smashline::clone_weapon(
            "falco",
            *WEAPON_KIND_FALCO_BLASTER_BULLET,
            "wolf",
            "plasmapulse",
            true,
        )
    }
}
