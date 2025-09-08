#[derive(Debug)]
pub enum HttpErrorCase {
    ZeroZero,
    ZeroOne,
    ZeroThree,
    ZeroFour,
    ZeroSix,
}

impl HttpErrorCase {
    pub fn get_case(&self) -> String {
        match self {
            HttpErrorCase::ZeroZero => String::from("00"),
            HttpErrorCase::ZeroOne => String::from("01"),
            HttpErrorCase::ZeroThree => String::from("03"),
            HttpErrorCase::ZeroSix => String::from("06"),
            HttpErrorCase::ZeroFour => String::from("04"),
        }
    }
}
