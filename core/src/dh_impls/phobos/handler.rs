use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;
use std::result;

use log;
use serde;
use serde_json;

use crate::defines::ReeInt;
use crate::dh;

use super::address::Address;
use super::data::{Assemble, EveGroup, EveType, FighterAbil, FsdItem, Metadata, TypeFighterAbil};
use super::error::{Error, FromPath};

type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Handler {
    base_path: PathBuf,
}
impl Handler {
    pub fn new<T: Into<PathBuf>>(path: T) -> Handler {
        Handler { base_path: path.into() }
    }
    fn read_file(&self, addr: &Address) -> io::Result<Vec<u8>> {
        let full_path = addr.get_full_path(&self.base_path);
        let mut bytes = Vec::new();
        File::open(full_path)?.read_to_end(&mut bytes)?;
        Ok(bytes)
    }
    fn read_json(&self, addr: &Address) -> Result<serde_json::Value> {
        let bytes = self
            .read_file(addr)
            .map_err(|e| Error::from_path(e, addr.get_part_str()))?;
        let data = serde_json::from_slice(&bytes).map_err(|e| Error::from_path(e, addr.get_part_str()))?;
        Ok(data)
    }
    // FSD Lite methods
    fn handle_fsdlite<T, U>(&self, addr: &Address) -> dh::Result<dh::Container<U>>
    where
        T: serde::de::DeserializeOwned + Assemble<U>,
    {
        let unprocessed = self.read_json(&addr)?;
        let decomposed = Handler::decompose_fsdlite(&addr, unprocessed)?;
        Handler::convert_fsdlite::<T, U>(decomposed)
    }
    fn decompose_fsdlite(addr: &Address, json: serde_json::Value) -> Result<Vec<FsdItem>> {
        match json {
            serde_json::Value::Object(map) => Ok(map.into_iter().map(|(k, v)| FsdItem::new(k, v)).collect()),
            _ => Err(Error::new(format!(
                "{} FSD Lite decomposition failed: highest-level structure is not a map",
                addr.get_part_str()
            ))),
        }
    }
    fn convert_fsdlite<T, U>(decomposed: Vec<FsdItem>) -> dh::Result<dh::Container<U>>
    where
        T: serde::de::DeserializeOwned + Assemble<U>,
    {
        let mut data = Vec::new();
        let mut errors: u32 = 0;
        for fsd_item in decomposed {
            match (
                fsd_item.id.parse::<ReeInt>(),
                serde_json::from_value::<T>(fsd_item.item),
            ) {
                (Ok(id), Ok(item)) => data.push(item.assemble(id)),
                _ => errors += 1,
            }
        }
        Ok(dh::Container::new(data, errors))
    }
}
impl dh::Handler for Handler {
    fn get_evetypes(&self) -> dh::Result<dh::Container<dh::EveType>> {
        let addr = Address::new("fsd_lite", "evetypes");
        log::info!("processing {}", addr.get_full_str(&self.base_path));
        self.handle_fsdlite::<EveType, dh::EveType>(&addr)
    }
    fn get_evegroups(&self) -> dh::Result<dh::Container<dh::EveGroup>> {
        let addr = Address::new("fsd_lite", "evegroups");
        log::info!("processing {}", addr.get_full_str(&self.base_path));
        self.handle_fsdlite::<EveGroup, dh::EveGroup>(&addr)
    }
    fn get_fighterabils(&self) -> dh::Result<dh::Container<dh::FighterAbil>> {
        let addr = Address::new("fsd_lite", "fighterabilities");
        log::info!("processing {}", addr.get_full_str(&self.base_path));
        self.handle_fsdlite::<FighterAbil, dh::FighterAbil>(&addr)
    }
    fn get_typefighterabils(&self) -> dh::Result<dh::Container<dh::TypeFighterAbil>> {
        let addr = Address::new("fsd_lite", "fighterabilitiesbytype");
        log::info!("processing {}", addr.get_full_str(&self.base_path));
        self.handle_fsdlite::<TypeFighterAbil, dh::TypeFighterAbil>(&addr)
    }
    fn get_version(&self) -> dh::Result<String> {
        let addr = Address::new("phobos", "metadata");
        log::info!("processing {}", addr.get_full_str(&self.base_path));
        let unprocessed = self.read_json(&addr)?;
        let metadatas: Vec<Metadata> =
            serde_json::from_value(unprocessed).map_err(|e| Error::from_path(e, addr.get_part_str()))?;
        let mut version = None;
        for metadata in metadatas {
            if metadata.field_name == "client_build" {
                version = Some(metadata.field_value);
                break;
            }
        }
        match version {
            Some(v) => Ok(v.to_string()),
            None => Err(Error::new("version fetch failed: unable to find client build").into()),
        }
    }
}
