use wasm_bindgen::prelude::*;

pub mod types;
pub mod sph;
pub mod physics;
pub mod simulation;
pub mod optimization;
pub mod api;

#[cfg(feature = "native")]
pub mod visualization;

pub use crate::types::*;
pub use crate::sph::*;
pub use crate::physics::*;
pub use crate::simulation::*;
pub use crate::optimization::*;
pub use api::*;

#[cfg(feature = "native")]
pub use crate::visualization::*;
