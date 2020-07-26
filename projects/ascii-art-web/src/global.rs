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
        match self {
            GlobalSettings { scene, ascii_image, braille_image, emoji_image } => f
                .debug_struct("GlobalSettings")
                .field("scene", scene)
                .field("ascii_image", &format_image_size(ascii_image))
                .field("braille_image", &format_image_size(braille_image))
                .field("emoji_image", &format_image_size(emoji_image))
                .finish(),
        }
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

impl GlobalSettings {}

fn format_image_size(img: &Option<DynamicImage>) -> String {
    match img {
        Some(i) => format!("{}×{}", i.width(), i.height()),
        None => String::from("0"),
    }
}
