use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

use log::info;
use serde_json::Value;

use crate::data_handler::common::{DataHandler, DataRow};

use super::address::PhobosAddress;

pub struct PhobosDataHandler {
    base_path: PathBuf,
}

impl PhobosDataHandler {
    pub fn new<P: Into<PathBuf>>(path: P) -> PhobosDataHandler {
        PhobosDataHandler {
            base_path: path.into(),
        }
    }
    fn _read_file(&self, addr: PhobosAddress) -> io::Result<Vec<u8>> {
        let full_path = self
            .base_path
            .join(addr.folder)
            .join(format!("{}.json", addr.file));
        let mut bytes: Vec<u8> = Vec::new();
        File::open(full_path)?.read_to_end(&mut bytes)?;
        Ok(bytes)
    }

    fn _read_json(&self, addr: PhobosAddress) -> Value {
        let bytes = self._read_file(addr).unwrap();
        serde_json::from_slice(&bytes).unwrap()
    }
}

impl DataHandler for PhobosDataHandler {
    fn get_evetypes(&self) -> Vec<DataRow> {
        info!("processing evetypes");
        let _json = self._read_json(PhobosAddress {
            folder: "fsd_lite",
            file: "evetypes",
        });
        let data = Vec::new();
        data
    }
}
