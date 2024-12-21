mod wit;
mod engine;
mod selectors;
mod animations;
mod element;
mod events;
mod instance;
#[macro_use]
mod macros;
mod node;
mod style;
mod text;
mod value;

pub(crate) type Xid = u64;
pub(crate) use serde_json::Value as Json;
pub(crate) use events::Event;

use engine::Engine;
use instance::{Instance, InstanceId};

use std::sync::OnceLock;

use dashmap::DashMap;

pub(crate) static ENGINES: OnceLock<DashMap<InstanceId, Engine>> = OnceLock::new();

// pub fn new(w: f32, h: f32) -> Instance {
//     Instance(Engine::new_instance(w, h))
// }
