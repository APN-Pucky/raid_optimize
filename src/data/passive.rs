
#[derive(Deserialize,Debug, Clone, Copy, PartialEq)]
pub enum Passive {
    None,
    Generic,
    //Natalie
    BloodthirstyDesire,
    //Maya
    ForceOfMercy,
    //Seth
    DeepSeaBloodline,
    //Space
    Resplendence {
        turn_meter_ratio: f32,
    },
    //Tifya
    SharpInstinct,
    //Hazier
    BloodlustStrike {
        leech : f32,
        damage_reduction_buffs : f32,
        damage_reduction_nobuffs : f32,
    },
    IncessantChatter, // TODO
    //Margarita
    CounterattackCommand {
        blades : u32,
        crit_damage_turns : u32,
        attack_damage_ratio : f32,
    },
    //Alahan
    Commendation {
        max_hp_ratio: f32,
        attack_up_turns : u32,
    }
    //...
}