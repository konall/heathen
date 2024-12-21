#[derive(Debug, Default, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum TextAlign {
    #[default]
    Left,
    Centre,
    Right,
    Justified
}

#[derive(Debug, Default, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum TextDecoration {
    #[default]
    Normal,
    Underline,
    Strikethrough
}

#[derive(Debug, Default, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum TextWeight {
    #[default]
    Normal,
    Bold,
    Light
}

#[derive(Debug, Default, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum TextStyle {
    #[default]
    Normal,
    Italic,
    Oblique
}

#[derive(Debug, Clone, Default, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Text {
    pub family: String,
    pub size: f32,
    pub align: TextAlign,
    pub decoration: TextDecoration,
    pub style: TextStyle,
    pub weight: TextWeight,
    pub colour: String
}

pub(crate) struct TextResources {
    pub(crate) font_system: cosmic_text::FontSystem,
    pub(crate) swash_cache: cosmic_text::SwashCache
}
impl Default for TextResources {
    fn default() -> Self {
        Self {
            font_system: cosmic_text::FontSystem::new(),
            swash_cache: cosmic_text::SwashCache::new()
        }
    }
}
