use image::{DynamicImage, GenericImageView};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug, Formatter};

#[derive(Serialize, Deserialize, Clone)]
pub struct GlobalSettings {
    pub scene: Scene,
    pub ascii_image: Option<DynamicImage>,
    pub braille_image: Option<DynamicImage>,
    pub emoji_image: Option<DynamicImage>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Scene {
    AsciiArt,
    BrailleArt,
    EmojiArt,
}

impl Debug for GlobalSettings {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("GlobalSettings")
            .field("scene", &self.scene)
            .field("ascii.image", &format_image_size(&self.ascii_image))
            .field("braille.image", &format_image_size(&self.braille_image))
            .field("emoji.image", &format_image_size(&self.emoji_image))
            .finish()
    }
}

impl Debug for Scene {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Scene::AsciiArt => f.debug_tuple("AsciiArt").finish(),
            Scene::BrailleArt => f.debug_tuple("BrailleArt").finish(),
            Scene::EmojiArt => f.debug_tuple("EmojiArt").finish(),
        }
    }
}

impl Default for GlobalSettings {
    fn default() -> Self {
        Self { scene: Scene::AsciiArt, ascii_image: None, braille_image: None, emoji_image: None }
    }
}

fn format_image_size(img: &Option<DynamicImage>) -> String {
    match img {
        Some(i) => format!("{}×{}", i.width(), i.height()),
        None => String::from("0"),
    }
}
