use crate::database::schemas::uploads;
use diesel::{Queryable, Selectable};
use uuid::Uuid;

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = uploads)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Uploads {
    pub uuid: Uuid,
    pub expiration: i64,
    pub getted: bool,
}
