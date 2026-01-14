use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EconomicEntity {
    pub id: EntityId,
    pub name: i32,
    pub value: i32,
}

impl EconomicEntity {
    fn save_to_file() -> Self {
        let json = serde_json::to_string(&Self)?;
    }
}


#[derive(Serialize, Deserialize)]
pub struct EntityId(Uuid);
impl EntityId {
    pub fn new() -> Self {
        EntityId(Uuid::new_v4())
    }
}



