
#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Login{
    None,
    SubscribeStar,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserData{
    pub login : Login,
    pub name:String,
}