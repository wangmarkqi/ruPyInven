use crate::common::tools::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Inventory {
    #[serde(default = "default_string")]
    pub product: String,
    #[serde(default = "default_string")]
    pub date: String,
    #[serde(default = "default_f32")]
    pub qty: f32,
}

impl Default for Inventory {
    fn default() -> Inventory {
        Inventory {
            product: "".to_string(),
            date: "".to_string(),
            qty: 0.0,
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InvenRes{
    #[serde(default = "default_string")]
    pub product: String,
    #[serde(default = "default_string")]
    pub date: String,
    #[serde(default = "default_f32")]
    pub accumulate: f32,
    #[serde(default = "default_f32")]
    pub value: f32,
}

impl Default for InvenRes{
    fn default() -> InvenRes{
        InvenRes {
            product: "".to_string(),
            date: "".to_string(),
            accumulate: 0.0,
            value: 0.0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct  Price{
    #[serde(default = "default_string")]
    pub product: String,
    #[serde(default = "default_string")]
    pub date: String,
    #[serde(default = "default_f32")]
    pub price: f32,
}

impl Default for Price{
    fn default() -> Price{
        Price{
            product: "".to_string(),
            date: "".to_string(),
            price: 0.0,
        }
    }
}
