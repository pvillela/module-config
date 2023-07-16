use super::DbErr;

/// type of application errors.
#[derive(Debug)]
pub struct AppErr;

impl From<DbErr> for AppErr {
    fn from(_db_err: DbErr) -> Self {
        // TODO: properly implement this
        AppErr
    }
}
