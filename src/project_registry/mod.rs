pub mod cloud;

pub use cerberus::{
    project::ProjectData,
    registry::{
        RegistryClient as ProjectRegistry,
        RegistryError as Error,
        RegistryResult as Result,
    },
};
