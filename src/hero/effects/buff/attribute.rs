use enum_map::Enum;
#[derive(Debug,Enum, PartialEq, Eq ,strum_macros::Display,Copy,Clone,Deserialize, )]
pub enum AttributeBuff {
    RippleI,
    RippleII,
    AttackUpI,
    AttackUpII,
    TenacityUpI,
    TenacityUpII,
    SpeedUpI,
    SpeedUpII,
}