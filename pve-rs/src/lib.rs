//! Rust library for the Proxmox VE code base.

#[path = "../common/src/mod.rs"]
pub mod common;

pub mod apt;
pub mod notify_context;
pub mod openid;
pub mod resource_scheduling;
pub mod tfa;

#[perlmod::package(name = "Proxmox::Lib::PVE", lib = "pve_rs")]
mod export {
    use crate::{common, notify_context};

    #[export]
    pub fn init() {
        common::logger::init("PVE_LOG", "info");
        notify_context::init();
    }
}
