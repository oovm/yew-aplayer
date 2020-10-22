use crate::create_aplayer;
use serde_derive::{Deserialize, Serialize};
use wasm_bindgen::{prelude::*, JsValue};
use web_sys::Element;

pub fn build_aplayer(e: &Element, o: &APlayerOptions) {
    create_aplayer(e, &JsValue::from_serde(o).unwrap())
}

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct APlayerAudio {
    #[wasm_bindgen(skip)]
    pub name: String,
    #[wasm_bindgen(skip)]
    pub artist: String,
    #[wasm_bindgen(skip)]
    pub url: String,
    #[wasm_bindgen(skip)]
    pub cover: String,
    #[wasm_bindgen(skip)]
    pub lrc: Option<String>,
}

#[wasm_bindgen]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct APlayerOptions {
    pub autoplay: bool,
    pub volume: f32,
    pub mutex: bool,
    #[serde(rename = "listFolded")]
    pub list_folded: bool,
    #[serde(rename = "listMaxHeight")]
    pub list_max_height: u32,

    fixed: bool,
    mini: bool,

    #[serde(rename = "theme")]
    theme_color: String,
    #[serde(rename = "loop")]
    audio_loop: String,
    #[serde(rename = "order")]
    audio_order: String,
    preload: String,
    #[serde(rename = "lrcType")]
    lrc_type: u32,
    audio: Vec<APlayerAudio>,
    #[serde(rename = "storageName")]
    storage_name: String,
}

impl Default for APlayerOptions {
    fn default() -> Self {
        Self {
            autoplay: false,
            volume: 0.7,
            mutex: true,
            list_folded: false,
            list_max_height: 90,
            fixed: false,
            mini: false,
            theme_color: "#b7daff".to_string(),
            audio_loop: "all".to_string(),
            audio_order: "list".to_string(),
            preload: "auto".to_string(),
            lrc_type: 3,
            audio: vec![],
            storage_name: "aplayer-setting".to_string(),
        }
    }
}

impl APlayerOptions {
    pub fn fixed_player() -> Self {
        Self { fixed: true, ..APlayerOptions::default() }
    }
    pub fn mini_player() -> Self {
        Self { mini: true, ..APlayerOptions::default() }
    }
}

impl APlayerOptions {
    pub fn set_audio_list(&mut self, list: &[APlayerAudio]) -> bool {
        let mut audios = Vec::with_capacity(list.len());
        for a in list {
            audios.push(APlayerAudio::from(a.clone()))
        }
        self.audio = audios;
        true
    }

    pub fn set_audio_loop(&mut self, cfg: &str) -> bool {
        match cfg {
            "all" => self.audio_loop = String::from("all"),
            "one" => self.audio_loop = String::from("one"),
            "none" => self.audio_loop = String::from("none"),
            _ => return false,
        }
        true
    }
    pub fn set_audio_order(&mut self) -> bool {
        unimplemented!()
    }
    pub fn set_preload(&mut self) -> bool {
        unimplemented!()
    }
}

impl APlayerAudio {
    pub fn new(name: &str, artist: &str, url: &str, cover: &str, lrc: Option<&str>) -> Self {
        Self {
            name: name.to_string(),
            artist: artist.to_string(),
            url: url.to_string(),
            cover: cover.to_string(),
            lrc: lrc.map(ToString::to_string),
        }
    }
}
