#[macro_use]
mod engine;
mod parsing;
mod animations;
mod events;
mod style;
mod node;
mod document;
mod element;

pub(crate) type Xid = usize;

pub use document::Document;
pub use element::Element;
pub use events::Event;
pub use miniserde::json::Value;
