#[derive(Debug)]
pub enum HttpScenario {
    Index,
    Register,
    Login,
}

impl HttpScenario {
    pub fn get_code(&self) -> String {
        match self {
            HttpScenario::Index => String::from("00"),
            HttpScenario::Register => String::from("13"),
            HttpScenario::Login => String::from("14"),
        }
    }
}
