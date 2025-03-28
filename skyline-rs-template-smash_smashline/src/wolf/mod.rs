use {
    smash::{
        app::{lua_bind::*, sv_animcmd::*, *},
        hash40,
        lib::lua_const::*,
        lua2cpp::*,
        phx::*,
    },
    smash_script::*,
    smashline::*,
};

use smashline::Priority::*;
//Fighter Frame

pub static mut FIGHTER_WOLF_GENERATE_ARTICLE_PLASMAPULSE: i32 = 5; // Needs to be equal to your character's number of articles
const WEAPON_WOLF_PLASMAPULSE_STATUS_KIND_SHOOT: i32 = 0x0;

pub static mut FIGHTER_WOLF_GENERATE_ARTICLE_PLASMAPULSEFULLCHARGE: i32 = 5; // Needs to be equal to your character's number of articles
const WEAPON_WOLF_PLASMAPULSEFULLCHARGE_STATUS_KIND_SHOOT: i32 = 0x0;

pub static mut FIGHTER_WOLF_GENERATE_ARTICLE_BLACKSHIELD: i32 = 5; // Needs to be equal to your character's number of articles
const WEAPON_WOLF_BLACKSHIELD_STATUS_KIND_SHOOT: i32 = 0x0;

unsafe extern "C" fn wolf_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let color =
            WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);

        if crate::MARKED_COLORS[color as usize] {}
    }
}

// Status Scripts
unsafe extern "C" fn shoot_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CORRECT_KIND_NONE),
        false,
        0,
        0,
        0,
        0,
    );
    0.into()
}
unsafe extern "C" fn shoot_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(
        weapon.module_accessor,
        *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER,
    );
    let owner_boma = sv_battle_object::module_accessor(owner_id as u32);
    let owner_lr = PostureModule::lr(owner_boma);

    PostureModule::set_lr(weapon.module_accessor, owner_lr);

    MotionModule::change_motion(
        weapon.module_accessor,
        Hash40::new("shoot"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false,
    );
    // This is just for positioning the article, you can do whatever you want
    PostureModule::add_pos_2d(weapon.module_accessor, &Vector2f { x: 5.0, y: 5.0 });
    // Speed stuff
    // In case you're cloning an article and it doesn't move,
    // you may delete/comment the lines below, EXCEPT the weapon.fastshift.
    // Then do what the comment says in shoot_main_loop

    let lr = PostureModule::lr(weapon.module_accessor);
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        2.0 * lr
    );
    weapon.fastshift(L2CValue::Ptr(shoot_main_loop as *const () as _))
}
unsafe extern "C" fn shoot_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if AttackModule::is_infliction_status(
        weapon.module_accessor,
        *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD,
    ) {
        AttackModule::set_add_reaction_frame(weapon.module_accessor, 0, 5.0, false);
    }
    0.into()
}
unsafe extern "C" fn shoot_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn shoot_pre_fullcharge(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CORRECT_KIND_NONE),
        false,
        0,
        0,
        0,
        0,
    );
    0.into()
}
unsafe extern "C" fn shoot_main_fullcharge(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(
        weapon.module_accessor,
        *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER,
    );
    let owner_boma = sv_battle_object::module_accessor(owner_id as u32);
    let owner_lr = PostureModule::lr(owner_boma);

    PostureModule::set_lr(weapon.module_accessor, owner_lr);

    MotionModule::change_motion(
        weapon.module_accessor,
        Hash40::new("shoot"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false,
    );
    // This is just for positioning the article, you can do whatever you want
    PostureModule::add_pos_2d(weapon.module_accessor, &Vector2f { x: 5.0, y: 5.0 });

    weapon.fastshift(L2CValue::Ptr(shoot_main_loop_fullcharge as *const () as _))
}
unsafe extern "C" fn shoot_main_loop_fullcharge(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if AttackModule::is_infliction_status(
        weapon.module_accessor,
        *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD,
    ) {
        AttackModule::set_add_reaction_frame(weapon.module_accessor, 0, 12.0, false);
    }
    0.into()
}
unsafe extern "C" fn shoot_end_fullcharge(weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

//Basic Attacks

///Attack11
unsafe extern "C" fn game_attack11(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 3.0, 3.0);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(
            agent,
            0,
            0,
            Hash40::new("top"),
            2.0,
            361,
            20,
            0,
            20,
            1.4,
            0.0,
            7.0,
            6.7,
            None,
            None,
            None,
            1.6,
            1.0,
            *ATTACK_SETOFF_KIND_ON,
            *ATTACK_LR_CHECK_F,
            false,
            0,
            0.0,
            0,
            false,
            false,
            false,
            false,
            true,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_normal"),
            *ATTACK_SOUND_LEVEL_S,
            *COLLISION_SOUND_ATTR_PUNCH,
            *ATTACK_REGION_PUNCH,
        );
        macros::ATTACK(
            agent,
            1,
            0,
            Hash40::new("top"),
            2.0,
            361,
            20,
            0,
            20,
            1.4,
            0.0,
            7.0,
            9.2,
            None,
            None,
            None,
            1.6,
            1.0,
            *ATTACK_SETOFF_KIND_ON,
            *ATTACK_LR_CHECK_F,
            false,
            0,
            0.0,
            0,
            false,
            false,
            false,
            false,
            true,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_normal"),
            *ATTACK_SOUND_LEVEL_S,
            *COLLISION_SOUND_ATTR_PUNCH,
            *ATTACK_REGION_PUNCH,
        );
        macros::ATTACK(
            agent,
            2,
            0,
            Hash40::new("top"),
            2.0,
            180,
            15,
            0,
            20,
            1.6,
            0.0,
            7.0,
            12.0,
            None,
            None,
            None,
            1.6,
            1.0,
            *ATTACK_SETOFF_KIND_ON,
            *ATTACK_LR_CHECK_F,
            false,
            0,
            0.0,
            0,
            false,
            false,
            false,
            false,
            true,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_FIGHTER,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_normal"),
            *ATTACK_SOUND_LEVEL_S,
            *COLLISION_SOUND_ATTR_PUNCH,
            *ATTACK_REGION_PUNCH,
        );
        macros::ATTACK(
            agent,
            3,
            0,
            Hash40::new("top"),
            2.0,
            361,
            15,
            0,
            20,
            1.6,
            0.0,
            7.0,
            12.0,
            None,
            None,
            None,
            1.6,
            1.0,
            *ATTACK_SETOFF_KIND_ON,
            *ATTACK_LR_CHECK_F,
            false,
            0,
            0.0,
            0,
            false,
            false,
            false,
            false,
            true,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_normal"),
            *ATTACK_SOUND_LEVEL_S,
            *COLLISION_SOUND_ATTR_PUNCH,
            *ATTACK_REGION_PUNCH,
        );
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 8.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 8.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 2, 8.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 3, 8.0, false);
        AttackModule::set_attack_height_all(
            agent.module_accessor,
            AttackHeight(*ATTACK_HEIGHT_MIDDLE),
            false,
        );
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(
            agent.module_accessor,
            *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO,
        );
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(
            agent.module_accessor,
            *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART,
        );
    }
}

///Attack11 Effect
unsafe extern "C" fn effect_attack11_msonic(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(
            agent,
            Hash40::new("null"),
            Hash40::new("top"),
            0,
            0,
            0,
            0,
            0,
            0,
            0.8,
            0,
            0,
            0,
            0,
            0,
            0,
            false,
        );
        macros::EFFECT_FOLLOW_FLIP(
            agent,
            Hash40::new("sys_attack_line"),
            Hash40::new("sys_attack_line"),
            Hash40::new("top"),
            1,
            6.8,
            -2,
            0,
            0,
            0,
            0.9,
            true,
            *EF_FLIP_YZ,
        );
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT(
            agent,
            Hash40::new("sys_attack_impact"),
            Hash40::new("top"),
            13,
            6.5,
            0,
            0,
            0,
            0,
            0.9,
            0,
            0,
            0,
            0,
            0,
            360,
            false,
        );
        macros::LAST_EFFECT_SET_ALPHA(agent, 0.7);
    }
}

///Attack13
unsafe extern "C" fn game_attack13(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::ATTACK(
            agent,
            0,
            0,
            Hash40::new("top"),
            4.0,
            361,
            110,
            0,
            50,
            2.6,
            0.0,
            7.2,
            2.2,
            None,
            None,
            None,
            2.0,
            1.0,
            *ATTACK_SETOFF_KIND_ON,
            *ATTACK_LR_CHECK_F,
            false,
            0,
            0.0,
            0,
            false,
            false,
            false,
            false,
            true,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_normal"),
            *ATTACK_SOUND_LEVEL_M,
            *COLLISION_SOUND_ATTR_KICK,
            *ATTACK_REGION_KICK,
        );
        macros::ATTACK(
            agent,
            1,
            0,
            Hash40::new("top"),
            4.0,
            361,
            110,
            0,
            50,
            2.6,
            0.0,
            8.0,
            6.8,
            None,
            None,
            None,
            2.0,
            1.0,
            *ATTACK_SETOFF_KIND_ON,
            *ATTACK_LR_CHECK_F,
            false,
            0,
            0.0,
            0,
            false,
            false,
            false,
            false,
            true,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_normal"),
            *ATTACK_SOUND_LEVEL_M,
            *COLLISION_SOUND_ATTR_KICK,
            *ATTACK_REGION_KICK,
        );
        macros::ATTACK(
            agent,
            2,
            0,
            Hash40::new("top"),
            4.0,
            361,
            110,
            0,
            50,
            3.6,
            0.0,
            8.5,
            13.0,
            Some(0.0),
            Some(7.5),
            Some(13.0),
            2.0,
            1.0,
            *ATTACK_SETOFF_KIND_ON,
            *ATTACK_LR_CHECK_F,
            false,
            0,
            0.0,
            0,
            false,
            false,
            false,
            false,
            true,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_normal"),
            *ATTACK_SOUND_LEVEL_M,
            *COLLISION_SOUND_ATTR_KICK,
            *ATTACK_REGION_KICK,
        );
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

///Attack13Effect
unsafe extern "C" fn effect_attack13_msonic(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(
            agent,
            Hash40::new("sys_attack_line"),
            Hash40::new("sys_attack_line"),
            Hash40::new("top"),
            -1.7,
            6.5,
            -2,
            -10,
            7,
            0,
            1,
            true,
            *EF_FLIP_YZ,
        );
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(
            agent,
            Hash40::new("sys_run_smoke"),
            Hash40::new("top"),
            0,
            0,
            0,
            0,
            0,
            0,
            1,
            0,
            0,
            0,
            0,
            0,
            0,
            false,
        );
        macros::EFFECT(
            agent,
            Hash40::new("sys_attack_impact"),
            Hash40::new("top"),
            13,
            8.5,
            0,
            0,
            0,
            0,
            1,
            0,
            0,
            0,
            0,
            0,
            360,
            false,
        );
        macros::LAST_EFFECT_SET_ALPHA(agent, 0.7);
    }
}

//Tilt Mid
unsafe extern "C" fn game_attacks3(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(
            agent,
            2,
            0,
            Hash40::new("kneel"),
            4.0,
            28,
            3,
            0,
            80,
            5.0,
            5.3,
            1.0,
            -0.7,
            None,
            None,
            None,
            1.0,
            1.0,
            *ATTACK_SETOFF_KIND_ON,
            *ATTACK_LR_CHECK_F,
            false,
            0,
            0.0,
            0,
            false,
            false,
            false,
            false,
            true,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_normal"),
            *ATTACK_SOUND_LEVEL_M,
            *COLLISION_SOUND_ATTR_KICK,
            *ATTACK_REGION_KICK,
        );
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::ATTACK(
            agent,
            0,
            0,
            Hash40::new("kneel"),
            7.0,
            361,
            115,
            0,
            30,
            5.0,
            6.0,
            0.0,
            -1.6,
            None,
            None,
            None,
            1.0,
            1.0,
            *ATTACK_SETOFF_KIND_ON,
            *ATTACK_LR_CHECK_F,
            false,
            0,
            0.0,
            0,
            false,
            false,
            false,
            false,
            true,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_normal"),
            *ATTACK_SOUND_LEVEL_L,
            *COLLISION_SOUND_ATTR_KICK,
            *ATTACK_REGION_KICK,
        );
        macros::ATTACK(
            agent,
            1,
            0,
            Hash40::new("kneel"),
            5.0,
            361,
            115,
            0,
            30,
            3.5,
            0.0,
            0.0,
            -1.0,
            None,
            None,
            None,
            1.0,
            1.0,
            *ATTACK_SETOFF_KIND_ON,
            *ATTACK_LR_CHECK_F,
            false,
            0,
            0.0,
            0,
            false,
            false,
            false,
            false,
            true,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_normal"),
            *ATTACK_SOUND_LEVEL_M,
            *COLLISION_SOUND_ATTR_KICK,
            *ATTACK_REGION_KICK,
        );
        macros::ATTACK(
            agent,
            2,
            0,
            Hash40::new("legl"),
            5.0,
            361,
            115,
            0,
            30,
            3.5,
            -1.0,
            0.0,
            0.0,
            None,
            None,
            None,
            1.0,
            1.0,
            *ATTACK_SETOFF_KIND_ON,
            *ATTACK_LR_CHECK_F,
            false,
            0,
            0.0,
            0,
            false,
            false,
            false,
            false,
            true,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_normal"),
            *ATTACK_SOUND_LEVEL_M,
            *COLLISION_SOUND_ATTR_KICK,
            *ATTACK_REGION_KICK,
        );
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, true);
    }
}

///Tilt Mid Effect

unsafe extern "C" fn effect_attacks3_msonic(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(
            agent,
            Hash40::new("sys_atk_smoke"),
            Hash40::new("top"),
            0,
            0,
            0,
            0,
            0,
            0,
            0.5,
            0,
            0,
            0,
            0,
            0,
            0,
            false,
        );
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(
            agent,
            Hash40::new("sys_attack_speedline"),
            Hash40::new("sys_attack_speedline"),
            Hash40::new("top"),
            0,
            1,
            -3,
            -22,
            0,
            0,
            1.1,
            true,
            *EF_FLIP_YZ,
        );
        macros::LAST_EFFECT_SET_COLOR(agent, 3, 0.7, 0.2);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT(
            agent,
            Hash40::new("sys_attack_impact"),
            Hash40::new("top"),
            0,
            9,
            15.5,
            0,
            0,
            0,
            1.1,
            0,
            0,
            0,
            0,
            0,
            360,
            true,
        );
    }
}

//Overdrive

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(
            agent,
            0,
            0,
            Hash40::new("hip"),
            16.0,
            60,
            60,
            0,
            70,
            7.2,
            2.5,
            -1.5,
            0.0,
            None,
            None,
            None,
            1.0,
            1.0,
            *ATTACK_SETOFF_KIND_ON,
            *ATTACK_LR_CHECK_F,
            false,
            5,
            0.0,
            0,
            false,
            false,
            false,
            false,
            true,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_fire"),
            *ATTACK_SOUND_LEVEL_L,
            *COLLISION_SOUND_ATTR_FIRE,
            *ATTACK_REGION_BODY,
        );
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::ATTACK(
            agent,
            0,
            0,
            Hash40::new("hip"),
            10.0,
            60,
            50,
            0,
            85,
            5.0,
            2.5,
            -1.5,
            0.0,
            None,
            None,
            None,
            1.0,
            1.0,
            *ATTACK_SETOFF_KIND_ON,
            *ATTACK_LR_CHECK_F,
            false,
            5,
            0.0,
            0,
            false,
            false,
            false,
            false,
            true,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_fire"),
            *ATTACK_SOUND_LEVEL_L,
            *COLLISION_SOUND_ATTR_FIRE,
            *ATTACK_REGION_BODY,
        );
    }
}
unsafe extern "C" fn game_specialhilanding(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(
            agent,
            0,
            0,
            Hash40::new("haver"),
            3.0,
            40,
            185,
            0,
            40,
            6.0,
            0.0,
            10.0,
            0.0,
            None,
            None,
            None,
            3.0,
            1.0,
            *ATTACK_SETOFF_KIND_OFF,
            *ATTACK_LR_CHECK_F,
            false,
            0,
            0.0,
            0,
            false,
            false,
            false,
            false,
            true,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_sting"),
            *ATTACK_SOUND_LEVEL_L,
            *COLLISION_SOUND_ATTR_ELEC,
            *ATTACK_REGION_BODY,
        );
        macros::ATTACK(
            agent,
            1,
            0,
            Hash40::new("haver"),
            3.0,
            40,
            185,
            0,
            40,
            6.0,
            0.0,
            4.0,
            0.0,
            None,
            None,
            None,
            3.0,
            1.0,
            *ATTACK_SETOFF_KIND_OFF,
            *ATTACK_LR_CHECK_F,
            false,
            0,
            0.0,
            0,
            false,
            false,
            false,
            false,
            true,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_sting"),
            *ATTACK_SOUND_LEVEL_L,
            *COLLISION_SOUND_ATTR_ELEC,
            *ATTACK_REGION_BODY,
        );
        macros::ATTACK(
            agent,
            2,
            0,
            Hash40::new("top"),
            3.0,
            40,
            185,
            0,
            40,
            8.0,
            0.0,
            7.0,
            0.0,
            None,
            None,
            None,
            3.0,
            1.0,
            *ATTACK_SETOFF_KIND_OFF,
            *ATTACK_LR_CHECK_F,
            false,
            0,
            0.0,
            0,
            false,
            false,
            false,
            false,
            true,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_sting"),
            *ATTACK_SOUND_LEVEL_L,
            *COLLISION_SOUND_ATTR_ELEC,
            *ATTACK_REGION_BODY,
        );
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

////Effect Test
unsafe extern "C" fn effect_specialhi_msonic(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT(
            agent,
            Hash40::new("falco_firebird_start"),
            Hash40::new("waist"),
            -8,
            0,
            0,
            0,
            90,
            10,
            1,
            0,
            0,
            0,
            0,
            0,
            0,
            true,
        );
        macros::EFFECT_FOLLOW_NO_STOP(
            agent,
            Hash40::new("falco_firebird"),
            Hash40::new("rot"),
            0.75,
            -1.5,
            1.5,
            90,
            0,
            0,
            0.85,
            true,
        );
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    for _ in 0..4 {
        if macros::is_excute(agent) {
            macros::BURN_COLOR(agent, 2, 0.1, 0, 0.5);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::BURN_COLOR_FRAME(agent, 2, 1, 0.2, 0.1, 0);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::BURN_COLOR_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 1.0);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::EFFECT_ALPHA(
            agent,
            Hash40::new("sys_attack_impact"),
            Hash40::new("rot"),
            0,
            -3,
            7,
            0,
            0,
            0,
            2.7,
            0,
            0,
            0,
            0,
            0,
            0,
            true,
            0.9,
        );
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("falco_firebird"), false, false);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    if macros::is_excute(agent) {
        macros::BURN_COLOR(agent, 2, 0.1, 0, 0.5);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::BURN_COLOR_FRAME(agent, 2, 1, 0.2, 0.1, 0);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::BURN_COLOR_NORMAL(agent);
    }
    wait(agent.lua_state_agent, 1.0);
}

//Sensory Overload

//-Animation Code

//Plasma Pulse

///Projectile

// Projectile Scripts

///No Charge Plasma Pulse
unsafe extern "C" fn game_shoot(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(
            agent,
            0,
            0,
            Hash40::new("top"),
            7.0,
            361,
            20,
            0,
            35,
            2.4,
            0.0,
            0.0,
            0.0,
            None,
            None,
            None,
            0.6,
            1.0,
            *ATTACK_SETOFF_KIND_ON,
            *ATTACK_LR_CHECK_SPEED,
            false,
            -2.5,
            0.0,
            0,
            true,
            true,
            false,
            false,
            false,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_aura"),
            *ATTACK_SOUND_LEVEL_S,
            *COLLISION_SOUND_ATTR_FIRE,
            *ATTACK_REGION_NONE,
        );
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        // This destroys the article
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
    }
}
unsafe extern "C" fn effect_shoot(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(
            agent,
            Hash40::new("sys_vector"),
            Hash40::new("top"),
            0,
            0,
            0,
            180,
            0,
            0,
            0.8,
            false,
        );
        macros::LAST_EFFECT_SET_COLOR(agent, 57, 109, 124);
        macros::LAST_EFFECT_SET_ALPHA(agent, 9.0);
    }
}
unsafe extern "C" fn sound_shoot(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {}
}
///Full Power Plasma Pulse
unsafe extern "C" fn game_fullchargeshot(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(
            agent,
            0,
            0,
            Hash40::new("top"),
            7.0,
            361,
            20,
            0,
            35,
            2.4,
            0.0,
            0.0,
            0.0,
            None,
            None,
            None,
            0.6,
            1.0,
            *ATTACK_SETOFF_KIND_ON,
            *ATTACK_LR_CHECK_SPEED,
            false,
            -2.5,
            0.0,
            0,
            true,
            true,
            false,
            false,
            false,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_aura"),
            *ATTACK_SOUND_LEVEL_S,
            *COLLISION_SOUND_ATTR_FIRE,
            *ATTACK_REGION_NONE,
        );
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        // This destroys the article
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
    }
}
unsafe extern "C" fn effect_fullchargeshot(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(
            agent,
            Hash40::new("sys_vector"),
            Hash40::new("top"),
            0,
            0,
            0,
            180,
            0,
            0,
            3.0,
            false,
        );
        macros::LAST_EFFECT_SET_COLOR(agent, 57, 109, 124);
        macros::LAST_EFFECT_SET_ALPHA(agent, 9.0);
    }
}
unsafe extern "C" fn sound_fullchargeshot(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {}
}
//unsafe extern "C" fn game_specialn(agent: &mut L2CAgentBase) {
//    macros::FT_MOTION_RATE(agent, 1.15);
//    frame(agent.lua_state_agent, 14.0);
//    if macros::is_excute(agent) {
//        ArticleModule::generate_article(
//            agent.module_accessor,
//            FIGHTER_WOLF_GENERATE_ARTICLE_PLASMAPULSE,
//            false,
//            -1,
//        );
//    }
//}

unsafe extern "C" fn game_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        macros::FT_MOTION_RATE(agent, 5.0);
        frame(agent.lua_state_agent, 30.0);
        if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            macros::FT_MOTION_RATE(agent, 1.15);
            DamageModule::add_damage(agent.module_accessor, 12.5, 0);
            StatusModule::change_status_request_from_script(
                agent.module_accessor,
                *FIGHTER_STATUS_KIND_FURAFURA,
                false,
            );
        } else {
            frame(agent.lua_state_agent, 30.0);
            macros::FT_MOTION_RATE(agent, 1.15);
            if macros::is_excute(agent) {
                ArticleModule::generate_article(
                    agent.module_accessor,
                    FIGHTER_WOLF_GENERATE_ARTICLE_PLASMAPULSEFULLCHARGE,
                    false,
                    -1,
                );
            }
        }
    } else {
        frame(agent.lua_state_agent, 14.0);
        macros::FT_MOTION_RATE(agent, 1.15);
        if macros::is_excute(agent) {
            ArticleModule::generate_article(
                agent.module_accessor,
                FIGHTER_WOLF_GENERATE_ARTICLE_PLASMAPULSEFULLCHARGE,
                false,
                -1,
            );
        }
    }
}

unsafe extern "C" fn effect_specialn_msonic(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(
            agent,
            Hash40::new("wolf_bayonet"),
            Hash40::new("haver"),
            0,
            0,
            0,
            0,
            0,
            0,
            1,
            true,
        );
        macros::AFTER_IMAGE4_ON_arg29(
            agent,
            Hash40::new("tex_wolf_bayonet1"),
            Hash40::new("tex_wolf_bayonet2"),
            3,
            Hash40::new("haver"),
            0,
            -0.3,
            3,
            Hash40::new("haver"),
            0,
            0.77,
            6.2,
            true,
            Hash40::new("null"),
            Hash40::new("haver"),
            0,
            0,
            0,
            0,
            0,
            0,
            1,
            0,
            *EFFECT_AXIS_X,
            0,
            *TRAIL_BLEND_ALPHA,
            101,
            *TRAIL_CULL_NONE,
            1.3,
            0.1,
        );
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 4);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("wolf_bayonet"), false, false);
        macros::EFFECT(
            agent,
            Hash40::new("wolf_blaster_shot"),
            Hash40::new("top"),
            0,
            9.8,
            13.2,
            0,
            0,
            0,
            1,
            0,
            0,
            0,
            0,
            0,
            0,
            true,
        );
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(
            agent,
            Hash40::new("sys_dash_smoke"),
            Hash40::new("top"),
            -8,
            0,
            0,
            0,
            0,
            0,
            1,
            0,
            0,
            0,
            0,
            0,
            0,
            false,
        );
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(
            agent,
            Hash40::new("wolf_bayonet"),
            Hash40::new("haver"),
            0,
            0,
            0,
            0,
            0,
            0,
            1,
            true,
        );
        macros::AFTER_IMAGE4_ON_arg29(
            agent,
            Hash40::new("tex_wolf_bayonet1"),
            Hash40::new("tex_wolf_bayonet2"),
            4,
            Hash40::new("haver"),
            0,
            -0.3,
            2.5,
            Hash40::new("haver"),
            0,
            0.77,
            6.3,
            true,
            Hash40::new("null"),
            Hash40::new("haver"),
            0,
            0,
            0,
            0,
            0,
            0,
            1,
            0,
            *EFFECT_AXIS_X,
            0,
            *TRAIL_BLEND_ALPHA,
            101,
            *TRAIL_CULL_NONE,
            1.3,
            0.1,
        );
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 3);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("wolf_bayonet"), false, false);
    }
}

//Electric Surge

////Electric Surge Attack (Grounded)

unsafe extern "C" fn game_attackdash(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(
            agent.module_accessor,
            *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING,
        );
    }
    frame(agent.lua_state_agent, 7.0);
    for _ in 0..5 {
        if macros::is_excute(agent) {
            macros::ATTACK(
                agent,
                0,
                0,
                Hash40::new("top"),
                1.6,
                185,
                100,
                30,
                20,
                2.5,
                0.0,
                11.0,
                4.1,
                None,
                None,
                None,
                0.6,
                1.0,
                *ATTACK_SETOFF_KIND_ON,
                *ATTACK_LR_CHECK_POS,
                false,
                0,
                0.0,
                0,
                false,
                false,
                false,
                false,
                true,
                *COLLISION_SITUATION_MASK_GA,
                *COLLISION_CATEGORY_MASK_ALL,
                *COLLISION_PART_MASK_ALL,
                false,
                Hash40::new("collision_attr_elec"),
                *ATTACK_SOUND_LEVEL_M,
                *COLLISION_SOUND_ATTR_ELEC,
                *ATTACK_REGION_BODY,
            );
            macros::ATTACK(
                agent,
                1,
                0,
                Hash40::new("top"),
                1.6,
                185,
                100,
                30,
                20,
                2.5,
                0.0,
                11.0,
                -6.1,
                None,
                None,
                None,
                0.6,
                1.0,
                *ATTACK_SETOFF_KIND_ON,
                *ATTACK_LR_CHECK_POS,
                false,
                0,
                0.0,
                0,
                false,
                false,
                false,
                false,
                true,
                *COLLISION_SITUATION_MASK_GA,
                *COLLISION_CATEGORY_MASK_ALL,
                *COLLISION_PART_MASK_ALL,
                false,
                Hash40::new("collision_attr_elec"),
                *ATTACK_SOUND_LEVEL_M,
                *COLLISION_SOUND_ATTR_ELEC,
                *ATTACK_REGION_BODY,
            );
            macros::ATTACK(
                agent,
                2,
                0,
                Hash40::new("top"),
                1.6,
                367,
                100,
                60,
                20,
                2.5,
                0.0,
                2.0,
                4.1,
                None,
                None,
                None,
                0.6,
                1.0,
                *ATTACK_SETOFF_KIND_ON,
                *ATTACK_LR_CHECK_POS,
                false,
                0,
                0.0,
                0,
                false,
                false,
                false,
                false,
                true,
                *COLLISION_SITUATION_MASK_GA,
                *COLLISION_CATEGORY_MASK_ALL,
                *COLLISION_PART_MASK_ALL,
                false,
                Hash40::new("collision_attr_elec"),
                *ATTACK_SOUND_LEVEL_M,
                *COLLISION_SOUND_ATTR_ELEC,
                *ATTACK_REGION_BODY,
            );
            macros::ATTACK(
                agent,
                3,
                0,
                Hash40::new("top"),
                1.6,
                367,
                100,
                60,
                20,
                2.5,
                0.0,
                2.0,
                -6.1,
                None,
                None,
                None,
                0.6,
                1.0,
                *ATTACK_SETOFF_KIND_ON,
                *ATTACK_LR_CHECK_POS,
                false,
                0,
                0.0,
                0,
                false,
                false,
                false,
                false,
                true,
                *COLLISION_SITUATION_MASK_GA,
                *COLLISION_CATEGORY_MASK_ALL,
                *COLLISION_PART_MASK_ALL,
                false,
                Hash40::new("collision_attr_elec"),
                *ATTACK_SOUND_LEVEL_M,
                *COLLISION_SOUND_ATTR_ELEC,
                *ATTACK_REGION_BODY,
            );
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        wait(agent.lua_state_agent, 2.0);
    }
    if macros::is_excute(agent) {
        macros::ATTACK(
            agent,
            0,
            0,
            Hash40::new("top"),
            4.0,
            361,
            110,
            0,
            40,
            13.5,
            0.0,
            7.0,
            -2.0,
            None,
            None,
            None,
            1.2,
            1.0,
            *ATTACK_SETOFF_KIND_ON,
            *ATTACK_LR_CHECK_POS,
            false,
            0,
            0.0,
            0,
            false,
            false,
            false,
            false,
            true,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_elec"),
            *ATTACK_SOUND_LEVEL_L,
            *COLLISION_SOUND_ATTR_ELEC,
            *ATTACK_REGION_BODY,
        );
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 46.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(
            agent.module_accessor,
            *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING,
        );
    }
}

////Electric Surge Effects
unsafe extern "C" fn effect_attackdash_msonic(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(
            agent,
            Hash40::new("pikachu_elec_spark"),
            Hash40::new("top"),
            0,
            2,
            0,
            0,
            0,
            0,
            1,
            true,
        );
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    for _ in 0..4 {
        if macros::is_excute(agent) {
            macros::BURN_COLOR(agent, 255, 69, 0, 1.0);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::BURN_COLOR_FRAME(agent, 2, 255, 69, 0, 0);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::BURN_COLOR_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 2.0);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pikachu_elec_spark"), false, false);
    }
}
//Emerald Dive

unsafe extern "C" fn game_specialsstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        frame(agent.lua_state_agent, 15.0);
        if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            macros::FT_MOTION_RATE(agent, 5.0);
            DamageModule::add_damage(agent.module_accessor, 12.5, 0);
        } else {
            macros::FT_MOTION_RATE(agent, 3.0);
            DamageModule::add_damage(agent.module_accessor, 7.0, 0);
        }
    } else {
        macros::FT_MOTION_RATE(agent, 1.0);
        DamageModule::add_damage(agent.module_accessor, 1.0, 0);
    }
}
unsafe extern "C" fn game_specials(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(
            agent,
            *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW,
            0,
            12.0,
            361,
            90,
            0,
            60,
            0.0,
            1.0,
            *ATTACK_LR_CHECK_F,
            0.0,
            true,
            Hash40::new("collision_attr_purple"),
            *ATTACK_SOUND_LEVEL_L,
            *COLLISION_SOUND_ATTR_BOMB,
            *ATTACK_REGION_NONE,
        );
        macros::ATTACK_ABS(
            agent,
            *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH,
            0,
            4.0,
            0,
            10,
            0,
            100,
            0.0,
            1.0,
            *ATTACK_LR_CHECK_F,
            0.0,
            true,
            Hash40::new("collision_attr_normal"),
            *ATTACK_SOUND_LEVEL_S,
            *COLLISION_SOUND_ATTR_KICK,
            *ATTACK_REGION_NONE,
        );
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        damage!(
            agent,
            *MA_MSC_DAMAGE_DAMAGE_NO_REACTION,
            *DAMAGE_NO_REACTION_MODE_ALWAYS,
            0
        );
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(
            agent.module_accessor,
            *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT,
        );
        let target_group = WorkModule::get_int64(
            agent.module_accessor,
            *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP,
        );
        let target_no = WorkModule::get_int64(
            agent.module_accessor,
            *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO,
        );
        macros::ATK_HIT_ABS(
            agent,
            *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW,
            Hash40::new("throw"),
            target,
            target_group,
            target_no,
        );
    }
    frame(agent.lua_state_agent, 39.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(
            agent.module_accessor,
            *FIGHTER_GANON_STATUS_WORK_ID_FLAG_EXPLOSION_SET_FALL,
        );
    }
    frame(agent.lua_state_agent, 41.0);
    if macros::is_excute(agent) {
        damage!(
            agent,
            *MA_MSC_DAMAGE_DAMAGE_NO_REACTION,
            *DAMAGE_NO_REACTION_MODE_NORMAL,
            0
        );
    }
}
unsafe extern "C" fn effect_specials_msonic(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_POS(
            agent,
            Hash40::new("sys_catch"),
            Hash40::new("throw"),
            0,
            0,
            0,
            0,
            0,
            0,
            1,
            true,
        );
        macros::EFFECT_FOLLOW(
            agent,
            Hash40::new("pikachu_elec2"),
            Hash40::new("havel"),
            -1,
            0,
            0.5,
            0,
            0,
            0,
            1,
            true,
        );
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        macros::EFFECT(
            agent,
            Hash40::new("pikachu_elec2"),
            Hash40::new("throw"),
            0,
            0,
            0,
            0,
            0,
            0,
            0.8,
            0,
            0,
            0,
            0,
            0,
            0,
            true,
        );
    }
}
pub fn install() {
    unsafe {
        FIGHTER_WOLF_GENERATE_ARTICLE_PLASMAPULSE += smashline::clone_weapon(
            "miiswordsman",
            *WEAPON_KIND_MIISWORDSMAN_WAVE,
            "wolf",
            "plasmapulse",
            false,
        );
        FIGHTER_WOLF_GENERATE_ARTICLE_PLASMAPULSEFULLCHARGE += smashline::clone_weapon(
            "miiswordsman",
            *WEAPON_KIND_MIISWORDSMAN_WAVE,
            "wolf",
            "plasmapulsefullcharge",
            false,
        );
        Agent::new("wolf")
            //fighter frame
            .on_line(Main, wolf_frame)
            //Misc.
            //Jabs
            .game_acmd("game_attack11_msonic", game_attack11, High)
            .effect_acmd("effect_attack11_msonic", effect_attack11_msonic, High)
            .game_acmd("game_attack13_msonic", game_attack13, High)
            .effect_acmd("effect_attack13_msonic", effect_attack13_msonic, High)
            //Dash Attack
            .game_acmd("game_attackdash_msonic", game_attackdash, High)
            .effect_acmd("effect_attackdash_msonic", effect_attackdash_msonic, High)
            //Tilts
            .effect_acmd("effect_attacks3_msonic", effect_attacks3_msonic, High)
            //Smashes
            //Aerials
            //Specials
            .game_acmd("game_specialn_msonic", game_specialn, High)
            .effect_acmd("effect_specialn_msonic", effect_specialn_msonic, High)
            .game_acmd("game_specialhi_msonic", game_specialhi, High)
            .effect_acmd("effect_specialhi_msonic", effect_specialhi_msonic, High)
            .game_acmd("game_specialhilanding_msonic", game_specialhilanding, High)
            .game_acmd("game_specialsstart_msonic", game_specialsstart, High)
            .game_acmd("game_specials_msonic", game_specials, High)
            .effect_acmd("effect_specials_msonic", effect_specials_msonic, High)
            .install();
        //Normal Shot
        Agent::new("wolf_plasmapulse")
            .game_acmd("game_shoot", game_shoot, Priority::Default)
            .effect_acmd("effect_shoot", effect_shoot, Priority::Default)
            .status(Pre, WEAPON_WOLF_PLASMAPULSE_STATUS_KIND_SHOOT, shoot_pre)
            .status(Main, WEAPON_WOLF_PLASMAPULSE_STATUS_KIND_SHOOT, shoot_main)
            .status(End, WEAPON_WOLF_PLASMAPULSE_STATUS_KIND_SHOOT, shoot_end)
            .install();
        //Big Laser
        Agent::new("wolf_plasmapulsefullcharge")
            .game_acmd(
                "game_fullchargeshot",
                game_fullchargeshot,
                Priority::Default,
            )
            .effect_acmd(
                "effect_fullchargeshot",
                effect_fullchargeshot,
                Priority::Default,
            )
            .status(
                Pre,
                WEAPON_WOLF_PLASMAPULSEFULLCHARGE_STATUS_KIND_SHOOT,
                shoot_pre_fullcharge,
            )
            .status(
                Main,
                WEAPON_WOLF_PLASMAPULSEFULLCHARGE_STATUS_KIND_SHOOT,
                shoot_main_fullcharge,
            )
            .status(
                End,
                WEAPON_WOLF_PLASMAPULSEFULLCHARGE_STATUS_KIND_SHOOT,
                shoot_end_fullcharge,
            )
            .install();
    }
}

// ,
//         "fighter/wolf/cmn": [
//             "effect/fighter/wolf/transplant/falco/ef_falco.eff",
//             "effect/fighter/wolf/transplant/pikachu/ef_pikachu.eff",
//             "effect/fighter/wolf/transplant/zelda/ef_zelda.eff"
//         ]
