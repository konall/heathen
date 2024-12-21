pub(crate) mod gen {
    pub use crate::instance::InstanceX;
    pub use crate::element::ElementX;
    pub use crate::value::Value;

    wit_bindgen::generate!({
        path: "wit",
        exports: {
            "konall:heathen/instance/instance": InstanceX,
            "konall:heathen/element/element": ElementX,
            "konall:heathen/value/value": Value
        }
    });
}

pub(crate) mod types {
    pub(crate) use super::gen::konall::heathen::types::*;
    pub(crate) use super::gen::exports::konall::heathen::instance::OwnInstance as Instance;
    pub(crate) use super::gen::exports::konall::heathen::element::OwnElement as Element;
    pub(crate) use super::gen::exports::konall::heathen::value::OwnValue as Value;
}

pub(crate) mod traits {
    pub(crate) use super::gen::exports::konall::heathen::element::GuestElement;
    pub(crate) use super::gen::exports::konall::heathen::instance::GuestInstance;
    pub(crate) use super::gen::exports::konall::heathen::value::GuestValue;
}
