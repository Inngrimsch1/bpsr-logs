use log::info;
use crate::live::opcodes_models::{EncounterMutex};


pub fn is_boss(entity_id: i64) -> bool {
    return BOSS_IDS.contains(&entity_id)
}

const BOSS_IDS:[i64;15] = [
    783,    //Goblin
    425,    //Tina
    185,    //Tower
    38,     //Kanima
    103588, //Dark Mist Fortress
    1985,   //Dragon Claw Valley
    15179,  //Frost Ogre
    146,    //Arachnocrab
    15395,  //Tempest Ogre
    15323,  //Muku Chief
    15269,  //Brigand Leader
    15159,  //Golden Juggernaut
    15202,  //Inferno Ogre
    87,     //Lizardman King
    147,  //Venobzzar Incubator
];

pub fn reset_combat_data(state: tauri::State<'_, EncounterMutex>) {
    let mut encounter = state.lock().unwrap();
    encounter.fight_start = false;
    encounter.is_encounter_paused = false;
    encounter.time_fight_start_ms = 0;
    encounter.time_fight_start_ms_boss = 0;
    encounter.time_last_combat_packet_ms = 0;
    encounter.time_last_combat_packet_ms_boss = 0;
    encounter.total_dmg = 0;
    encounter.total_dmg_boss = 0;
    encounter.total_heal = 0;
    encounter.entity_uid_to_entity.clear();
}