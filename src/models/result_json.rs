use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ResultJson {
    pub time: f64,
    pub size: f64,
}

impl ResultJson {

    pub fn new(time: f64, size: f64) -> Self{
        ResultJson{time: time, size: size}
    }

}