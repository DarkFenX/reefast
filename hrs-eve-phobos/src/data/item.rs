use crate::fsd::FsdMerge;

#[derive(Debug, serde::Deserialize)]
pub(crate) struct Item {
    #[serde(rename = "groupID")]
    pub(crate) group_id: rc::ReeInt,
}
impl FsdMerge<rc::edt::EItem> for Item {
    fn fsd_merge(self, id: rc::ReeInt) -> Vec<rc::edt::EItem> {
        vec![rc::edt::EItem::new(id, self.group_id)]
    }
}
