#[allow(non_snake_case)]
mod helpers;
mod iappdomain;
mod iassembly;
mod iclrmetahost;
mod iclrruntimeinfo;
mod icorruntimehost;
mod ienumunknown;
mod imethodinfo;
mod istringwriter;
mod itype;
mod iunknown;
mod types;

pub use helpers::*;
pub use iappdomain::*;
pub use iassembly::*;
pub use iclrmetahost::*;
pub use iclrruntimeinfo::*;
pub use icorruntimehost::*;
pub use ienumunknown::*;
pub use imethodinfo::*;
pub use istringwriter::*;
pub use itype::*;
pub use iunknown::*;
pub use types::*;

pub trait Interface: Sized {
    const IID: GUID;

    fn vtable(&self) -> *const std::ffi::c_void;
}

pub trait Class: Sized {
    const CLSID: GUID;
}
