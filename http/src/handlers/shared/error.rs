use crate::util::Error;

#[derive(serde::Serialize)]
pub(in super::super) struct SingleErr {
    code: String,
    message: String,
}
impl From<Error> for SingleErr {
    fn from(err: Error) -> Self {
        Self {
            code: err.get_code(),
            message: err.to_string(),
        }
    }
}
