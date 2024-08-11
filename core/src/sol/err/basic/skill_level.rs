use crate::SkillLevel;

#[derive(Debug)]
pub struct SkillLevelError {
    pub level: SkillLevel,
}
impl SkillLevelError {
    pub(crate) fn new(level: SkillLevel) -> Self {
        Self { level }
    }
}
impl std::error::Error for SkillLevelError {}
impl std::fmt::Display for SkillLevelError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "skill level {} is out of allowed range [0, 5]", self.level)
    }
}
