pub struct Value(serde_json::Value);

use crate::wit::gen::exports::konall::heathen::value::{OwnValue, ValueTy};

impl crate::wit::gen::exports::konall::heathen::value::GuestValue for Value {
    fn ty(&self) -> ValueTy {
        todo!()
    }

    fn null() -> OwnValue {
        todo!()
    }

    fn bool(v:bool,) -> OwnValue {
        todo!()
    }

    fn string(v:wit_bindgen::rt::string::String,) -> OwnValue {
        todo!()
    }

    fn number(v:f64,) -> OwnValue {
        todo!()
    }

    fn array(v:wit_bindgen::rt::vec::Vec::<OwnValue>,) -> OwnValue {
        todo!()
    }

    fn object(v:wit_bindgen::rt::vec::Vec::<(wit_bindgen::rt::string::String,OwnValue,)>,) -> OwnValue {
        todo!()
    }

    fn as_null(&self,) -> Option<Result<(),()>> {
        todo!()
    }

    fn as_bool(&self,) -> Option<bool> {
        todo!()
    }

    fn as_string(&self,) -> Option<wit_bindgen::rt::string::String> {
        todo!()
    }

    fn as_number(&self,) -> Option<f64> {
        todo!()
    }

    fn as_array(&self,) -> Option<wit_bindgen::rt::vec::Vec::<OwnValue>> {
        todo!()
    }

    fn as_object(&self,) -> Option<wit_bindgen::rt::vec::Vec::<(wit_bindgen::rt::string::String,OwnValue,)>> {
        todo!()
    }
}
