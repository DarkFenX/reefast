//! Data handler implementation which uses JSON produced by
//! [Phobos](https://github.com/pyfa-org/Phobos) as a data source.

pub use handler::PhbFileDHandler;

mod address;
mod aux;
mod data;
mod fsd;
mod handler;
