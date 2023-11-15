use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct Schedule {
    pub id: Uuid,
    pub feature_flag_id: Uuid,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl Schedule {
    pub fn new(
        id: Uuid,
        feature_flag_id: Uuid,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Schedule {
        Schedule {
            id,
            feature_flag_id,
            start,
            end,
        }
    }
}
