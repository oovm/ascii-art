use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GlobalSettings {
    pub scene: Scene,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Scene {
    AsciiArt,
    BrailleArt,
    EmojiArt,
}

impl Default for GlobalSettings {
    fn default() -> Self {
        Self { scene: Scene::AsciiArt }
    }
}

impl GlobalSettings {

}
