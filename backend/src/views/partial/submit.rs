use crate::entities::Submit as SubmitEntity;

pub struct Submit {
    pub id: String,
    pub user_id: String,
    pub asem: String,
    pub time: String,
}

impl From<SubmitEntity> for Submit {
    fn from(e: SubmitEntity) -> Self {
        Submit {
            id: e.id().to_string(),
            user_id: e.user_id.to_string(),
            asem: e.asem,
            time: e.time.format("%Y/%m/%d %H:%M").to_string(),
        }
    }
}

