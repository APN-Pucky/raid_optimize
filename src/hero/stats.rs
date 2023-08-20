use enum_map::Enum;

#[derive(Debug,Enum, PartialEq, Eq)]
pub enum Stats {
    DamageDone,
    DamageTaken,
}