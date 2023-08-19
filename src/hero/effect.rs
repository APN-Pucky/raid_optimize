use crate::wave::InstanceRef;

#[derive(Debug, Eq, Hash, PartialEq,strum_macros::Display)]
pub enum Effect {
    WetI,
    WetII,
    ColdI,
    ColdII,
    ColdIII,
    Bleed,
    HPBurning,
    Suffocated
}

