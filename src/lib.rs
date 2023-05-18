#[macro_use]
mod engine;
mod selectors;
mod animations;
mod events;
mod style;
mod node;
mod document;
mod element;
mod rpc;
mod bindings;

pub(crate) type Xid = u64;

pub use document::*;
pub use events::Event;
pub use serde_json::Value;
pub use serde_json::json as value;
