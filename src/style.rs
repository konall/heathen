pub type HexColour = String;
pub type Attribute = String;
pub type Duration = f32;

#[derive(Debug, Default, Clone, PartialEq)]
pub enum TextAlign {
    #[default]
    Left,
    Centre,
    Right,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum TextMod {
    #[default]
    Regular,
    Bold,
    Italic,
    Underline,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Background {
    Fill(HexColour),
    PixMap(Vec<u8>),
    B64PixMap(String),
    File(String),
    Url(String),
}
impl Default for Background {
    fn default() -> Self {
        Background::Fill(String::from("#000000"))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Timing {
    Linear,
    EaseInOut,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct BorderRadii {
    pub top_left: f32,
    pub bottom_left: f32,
    pub bottom_right: f32,
    pub top_right: f32
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct BorderColours {
    pub top: HexColour,
    pub bottom: HexColour,
    pub left: HexColour,
    pub right: HexColour
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Text {
    pub font: String,
    pub size: f32,
    pub align: TextAlign,
    pub mods: TextMod,
    pub colour: HexColour
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Transform {
    pub translate: (f32, f32),
    pub scale: (f32, f32),
    pub rotate: f32
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Style {
    pub z: f32,
    pub opacity: f32,
    pub background: Background,
    pub text: Text,
    pub transform: Transform,
    pub transitions: std::collections::HashMap<Attribute, (Duration, Timing)>,
    
    pub border_radii: BorderRadii,
    pub border_colours: BorderColours,
    
    pub layout: taffy::prelude::Style
}

#[derive(Clone, Default)]
pub(crate) struct StyleNode {
    pub(crate) style: Style,
    
    pub(crate) layers: std::collections::BTreeMap<String, StyleNode>
}

impl Style {
    pub(crate) fn parse_sheet(src: &str) -> std::collections::BTreeMap<String, Style> {
        let mut stylesheet = std::collections::BTreeMap::new();
        
        // let style_tree: std::collections::BTreeMap<String, StyleNode> = miniserde::json::from_str(src).unwrap();
        
        // let mut current = style_tree
        //     .into_iter()
        //     .map(|(k, v)| (k, v))
        //     .collect::<Vec<(String, StyleNode)>>();
        // let mut nxt = vec![];
        
        // while !current.is_empty() {
        //     for (selector, node) in current {
        //         stylesheet.insert(selector.clone(), node.style);
                
        //         nxt.extend(
        //             node.layers
        //                 .into_iter()
        //                 .map(|(s, n)| (format!("{} {}", selector, s), n)),
        //         );
        //     }
            
        //     current = nxt;
        //     nxt = vec![];
        // }
        
        stylesheet
    }
    
    // pub(crate) fn merge(&mut self, newer_style: &miniserde_json::Value) {
    //     let mut current_style = miniserde_json::to_value(&self).unwrap();
        
    //     for (att, val) in newer_style.as_object().unwrap() {
    //         current_style[att] = val.clone();
    //     }
        
    //     *self = miniserde_json::from_value(current_style).unwrap();
    // }
}

pub(crate) struct StyleScore {
    
}
