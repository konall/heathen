use crate::{
    animations::Timing,
    text::Text
};

pub type HexColour = String;
pub type Attribute = String;
pub type Duration = f32;

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Background {
    Fill(HexColour),
    PixMap(Vec<u8>),
    B64PixMap(String),
    File(String),
    Url(String),
}
impl Default for Background {
    fn default() -> Self {
        Background::Fill("#000000".into())
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BorderRadii {
    pub top_left: f32,
    pub bottom_left: f32,
    pub bottom_right: f32,
    pub top_right: f32
}

#[derive(Debug, Default, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BorderColours {
    pub top: HexColour,
    pub bottom: HexColour,
    pub left: HexColour,
    pub right: HexColour
}

#[derive(Debug, Clone, Default, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Transform {
    pub translate: (f32, f32),
    pub scale: (f32, f32),
    pub rotate: f32
}

#[derive(Debug, Clone, Default, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Style {
    pub z: f32,
    pub opacity: f32,
    pub background: Background,
    pub text: Text,
    pub transform: Transform,
    pub transitions: std::collections::HashMap<Attribute, (Duration, Timing)>,
    
    pub border_radii: BorderRadii,
    pub border_colours: BorderColours,
    
    #[serde(flatten)]
    pub layout: taffy::prelude::Style
}

impl Style {
    fn new() {
        // cosmic_text::Align::
    }
    // pub(crate) fn merge(&mut self, newer_style: &miniserde_json::Value) {
    //     let mut current_style = miniserde_json::to_value(&self).unwrap();
        
    //     for (att, val) in newer_style.as_object().unwrap() {
    //         current_style[att] = val.clone();
    //     }
        
    //     *self = miniserde_json::from_value(current_style).unwrap();
    // }
    
    // text
    fn text(&self) {
        
    }
    
    pub(crate) fn merge_with_style(&mut self, other: &Style) {
        
    }
    
    pub(crate) fn merge_with_selector(&mut self, selector: &str) {
        
    }
}
