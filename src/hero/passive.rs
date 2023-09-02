
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
    //...
}