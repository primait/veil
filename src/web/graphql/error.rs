use strum_macros::{AsRefStr, EnumIter};

#[derive(AsRefStr, EnumIter)]
pub enum ErrorCode {
    #[strum(serialize = "UNAUTHORIZED")]
    Unauthorized,
}
