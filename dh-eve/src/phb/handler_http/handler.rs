use std::fmt;

use reqwest::{IntoUrl, Url, blocking::Client};

use crate::{
    phb::{
        data::{
            PAttr, PBuff, PEffect, PFighterAbil, PItem, PItemAttrs, PItemEffects, PItemFighterAbils, PItemGroup,
            PItemList, PItemSkillMap, PItemSpaceComp, PMutaAttrMods, PMutaItemConvs,
        },
        fsd,
    },
    util::Error,
};

use super::error::FromSuffix;

/// Data handler which fetches [Phobos](https://github.com/pyfa-org/Phobos) JSON dump via HTTP
pub struct PhbHttpEdh {
    base_url: Url,
    data_version: String,
    client: Client,
}
impl PhbHttpEdh {
    /// Constructs HTTP EVE data handler using provided base URL and data version.
    ///
    /// URL should end with a trailing slash, and should point to the top-level directory of
    /// a data dump, e.g. `/phobos_en-us/` and not `/phobos_en-us/fsd_binary/`.
    ///
    /// This data handler assumes that data version is known before its construction.
    pub fn new<U: IntoUrl + Copy + Into<String>>(base_url: U, data_version: String) -> Result<Self, Error> {
        let base_url_conv = base_url
            .into_url()
            .map_err(|e| Error::PhbHttpInvalidBaseUrl(base_url.into(), format!("failed to interpret: {e}")))?;
        match base_url_conv.cannot_be_a_base() {
            true => Err(Error::PhbHttpInvalidBaseUrl(
                base_url.into(),
                "cannot be used as base".to_string(),
            )),
            false => Ok(Self {
                base_url: base_url_conv,
                data_version,
                client: Client::new(),
            }),
        }
    }
    fn fetch_data(&self, suffix: &str) -> Result<serde_json::Value, Error> {
        let full_url = self.base_url.join(suffix).map_err(|e| Error::from_suffix(e, suffix))?;
        let data = self
            .client
            .get(full_url)
            .send()
            .map_err(|e| Error::from_suffix(e, suffix))?
            .error_for_status()
            .map_err(|e| Error::from_suffix(e, suffix))?
            .json()
            .map_err(|e| Error::from_suffix(e, suffix))?;
        Ok(data)
    }
    fn process_fsd<T, U>(&self, folder: &'static str, file: &'static str) -> rc::ed::EResult<rc::ed::EDataCont<U>>
    where
        T: serde::de::DeserializeOwned + fsd::FsdMerge<U>,
    {
        let suffix = format!("{folder}/{file}.json");
        let json = self.fetch_data(&suffix)?;
        fsd::handle::<T, U>(json, &suffix)
    }
}
impl fmt::Debug for PhbHttpEdh {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PhbHttpEdh(\"{}\")", self.base_url)
    }
}
impl rc::ed::EveDataHandler for PhbHttpEdh {
    fn get_items(&self) -> rc::ed::EResult<rc::ed::EDataCont<rc::ed::EItem>> {
        self.process_fsd::<PItem, rc::ed::EItem>("fsd_binary", "types")
    }
    fn get_item_groups(&self) -> rc::ed::EResult<rc::ed::EDataCont<rc::ed::EItemGroup>> {
        self.process_fsd::<PItemGroup, rc::ed::EItemGroup>("fsd_binary", "groups")
    }
    fn get_item_lists(&self) -> rc::ed::EResult<rc::ed::EDataCont<rc::ed::EItemList>> {
        self.process_fsd::<PItemList, rc::ed::EItemList>("fsd_binary", "typelist")
    }
    fn get_attrs(&self) -> rc::ed::EResult<rc::ed::EDataCont<rc::ed::EAttr>> {
        self.process_fsd::<PAttr, rc::ed::EAttr>("fsd_binary", "dogmaattributes")
    }
    fn get_item_attrs(&self) -> rc::ed::EResult<rc::ed::EDataCont<rc::ed::EItemAttr>> {
        self.process_fsd::<PItemAttrs, rc::ed::EItemAttr>("fsd_binary", "typedogma")
    }
    fn get_effects(&self) -> rc::ed::EResult<rc::ed::EDataCont<rc::ed::EEffect>> {
        self.process_fsd::<PEffect, rc::ed::EEffect>("fsd_binary", "dogmaeffects")
    }
    fn get_item_effects(&self) -> rc::ed::EResult<rc::ed::EDataCont<rc::ed::EItemEffect>> {
        self.process_fsd::<PItemEffects, rc::ed::EItemEffect>("fsd_binary", "typedogma")
    }
    fn get_fighter_abils(&self) -> rc::ed::EResult<rc::ed::EDataCont<rc::ed::EFighterAbil>> {
        self.process_fsd::<PFighterAbil, rc::ed::EFighterAbil>("fsd_lite", "fighterabilities")
    }
    fn get_item_fighter_abils(&self) -> rc::ed::EResult<rc::ed::EDataCont<rc::ed::EItemFighterAbil>> {
        self.process_fsd::<PItemFighterAbils, rc::ed::EItemFighterAbil>("fsd_lite", "fighterabilitiesbytype")
    }
    fn get_buffs(&self) -> rc::ed::EResult<rc::ed::EDataCont<rc::ed::EBuff>> {
        self.process_fsd::<PBuff, rc::ed::EBuff>("fsd_lite", "dbuffcollections")
    }
    fn get_space_comps(&self) -> rc::ed::EResult<rc::ed::EDataCont<rc::ed::EItemSpaceComp>> {
        self.process_fsd::<PItemSpaceComp, rc::ed::EItemSpaceComp>("fsd_binary", "spacecomponentsbytype")
    }
    fn get_item_skill_reqs(&self) -> rc::ed::EResult<rc::ed::EDataCont<rc::ed::EItemSkillReq>> {
        self.process_fsd::<PItemSkillMap, rc::ed::EItemSkillReq>("fsd_binary", "requiredskillsfortypes")
    }
    fn get_muta_item_convs(&self) -> rc::ed::EResult<rc::ed::EDataCont<rc::ed::EMutaItemConv>> {
        self.process_fsd::<PMutaItemConvs, rc::ed::EMutaItemConv>("fsd_binary", "dynamicitemattributes")
    }
    fn get_muta_attr_mods(&self) -> rc::ed::EResult<rc::ed::EDataCont<rc::ed::EMutaAttrMod>> {
        self.process_fsd::<PMutaAttrMods, rc::ed::EMutaAttrMod>("fsd_binary", "dynamicitemattributes")
    }
    fn get_data_version(&self) -> rc::ed::EResult<String> {
        Ok(self.data_version.clone())
    }
}
