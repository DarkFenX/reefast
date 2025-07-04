use crate::{
    def::{AttrVal, OF},
    err::basic::BreacherDmgError,
};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BreacherInfo {
    absolute_max: AttrVal,
    relative_max: AttrVal,
}
impl BreacherInfo {
    pub fn try_new(absolute_max: AttrVal, relative_max: AttrVal) -> Result<Self, BreacherInfoError> {
        if absolute_max < OF(0.0) {
            return Err(BreacherDmgError::Absolute(absolute_max).into());
        }
        if relative_max < OF(0.0) || relative_max > OF(1.0) {
            return Err(BreacherDmgError::Absolute(relative_max).into());
        }
        Ok(Self {
            absolute_max,
            relative_max,
        })
    }
    pub fn get_absolute_max(&self) -> AttrVal {
        self.absolute_max
    }
    pub fn get_relative_max(&self) -> AttrVal {
        self.relative_max
    }
}

#[derive(thiserror::Error, Debug)]
pub enum BreacherInfoError {
    #[error("{0}")]
    InvalidValue(#[from] BreacherDmgError),
}
