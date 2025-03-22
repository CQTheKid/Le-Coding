use {
    crate::FIGHTER_WOLF_GENERATE_ARTICLE_PLASMAPULSE,
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

unsafe extern "C" fn wolf_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let color =
            WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);

        if crate::MARKED_COLORS[color as usize] {}
    }
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
unsafe extern "C" fn game_attackairn(agent: &mut L2CAgentBase) {
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

///Sensory Overload Effects
unsafe extern "C" fn effect_attackairn_msonic(agent: &mut L2CAgentBase) {
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

//Electric Surge

////Electric Surge Attack (Grounded)

unsafe extern "C" fn game_specialn(agent: &mut L2CAgentBase) {
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

////Electric Surge Attack (Aerial)

unsafe extern "C" fn game_specialairn(agent: &mut L2CAgentBase) {
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
    {
        frame(agent.lua_state_agent, 5.0);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_L);
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
unsafe extern "C" fn effect_specialn_msonic(agent: &mut L2CAgentBase) {
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

unsafe extern "C" fn effect_specialairn_msonic(agent: &mut L2CAgentBase) {
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
pub fn install() {
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
        //Tilts
        .effect_acmd("effect_attacks3_msonic", effect_attacks3_msonic, High)
        //Smashes
        //Aerials
        .game_acmd("game_attackairn_msonic", game_attackairn, High)
        .effect_acmd("effect_attackairn_msonic", effect_attackairn_msonic, High)
        //Specials
        .game_acmd("game_specialn_msonic", game_specialn, High)
        .game_acmd("game_specialairn_msonic", game_specialn, High)
        .effect_acmd("effect_specialn_msonic", effect_specialn_msonic, High)
        .effect_acmd("effect_specialairn_msonic", effect_specialairn_msonic, High)
        .game_acmd("game_specialhi_msonic", game_specialhi, High)
        .effect_acmd("effect_specialhi_msonic", effect_specialhi_msonic, High)
        .game_acmd("game_specialhilanding_msonic", game_specialhilanding, High)
        .install();
}

// ,
//         "fighter/wolf/cmn": [
//             "effect/fighter/wolf/transplant/falco/ef_falco.eff",
//             "effect/fighter/wolf/transplant/pikachu/ef_pikachu.eff",
//             "effect/fighter/wolf/transplant/zelda/ef_zelda.eff"
//         ]
