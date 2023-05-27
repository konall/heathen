#[macro_use]
mod engine;
mod selectors;
mod animations;
mod events;
mod style;
mod node;
mod document;
mod element;

pub(crate) type Xid = u64;
pub(crate) use serde_json::Value;
pub(crate) use events::Event;

pub use document::*;
pub use serde_json::json as value;
