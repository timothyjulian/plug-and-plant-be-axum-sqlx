#[derive(Debug)]
pub enum HttpScenario {
    Index
}

impl HttpScenario {
    pub fn get_code(&self) -> String {
        match self {
            HttpScenario::Index => String::from("00")
        }
    }
}