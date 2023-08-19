use std::collections::HashMap;

use ash::vk;
use drowsed_math::{Rectangle, FVec2};
use shax::font::atlas::FontAtlas;
use yum_mocha::{buffer::{img::ImageTexture, self}, device::LogicalDevice};

/// This entire section of the project will be dedicated to be written fully in german
/// since im trying to learn the language, except for function names and rust practices which
/// will have german translations in comments.
pub struct Kalligraphie {
    pub schriftart_atlas: FontAtlas,
    pub skala: f32,
    pub briefe_eintrage: HashMap<char, SchriftartGlipheEingang>,
    pub textur: ImageTexture
}
pub struct SchriftartGlipheEingang {
    /// rechteck in deutsch ist rectangle in englisch und position ist das gleicher
    /// in deutsch wie es ist in englisch also die abkürzung ist "pos".
    /// Ich verwenden rechteck nicht und stattdessen verwenden begrenzungsrahem
    /// auch bekannt als "bounding box" in englisch. das ist allerdings sehr lang also stattdessen
    /// ich benutze eine andere abkürzung: "rahmen" für "frame".
    pub pos_rahmen : Rectangle<f32>,
    pub brief_rahmen: Rectangle<f32>
}
impl Kalligraphie {
    // neu
    pub fn new(device: std::sync::Arc<LogicalDevice>,briefe: Vec<char>, daten: &Vec<u8>, bild_skala: f32) -> Self {
        let schriftart_atlas = FontAtlas::new(briefe, daten, bild_skala);
        let textur = buffer::img::ImageTexture::from_vec(device.clone(), vk::Format::R8G8B8A8_UINT, schriftart_atlas.bmp.width() as u32, schriftart_atlas.bmp.height() as u32, &schriftart_atlas.bmp.as_byte_slice().to_vec());
        let briefe_eintrage: HashMap<char, SchriftartGlipheEingang> = schriftart_atlas.offsets.iter().map(|(c, val)|{
            (*c, Self::place_letter(*c, bild_skala, &schriftart_atlas))
        }).collect();
        Self { schriftart_atlas, skala: bild_skala, briefe_eintrage, textur }
    }
    // stellen_brief
    fn place_letter(breif: char, skala: f32, schriftart_atlas: &FontAtlas) -> SchriftartGlipheEingang {
        
        let (mut pos_min,mut pos_max) = schriftart_atlas.place_letter(breif);
        pos_min = pos_min * skala;
        pos_max = pos_max * skala;
        let pos_rechteck = Rectangle::<f32> {
            top_left: FVec2::new(pos_min.x, pos_max.y),
            bottom_left: FVec2::new(pos_max.x, pos_min.y),
            bottom_right: FVec2::new(pos_min.x, pos_min.y),
            top_right: FVec2::new(pos_max.x, pos_max.y)
        };
        let (brief_min, brief_max) = schriftart_atlas.bbox_normalized(breif);
        let breif_rechteck = Rectangle::<f32> {
            top_left: FVec2::new(brief_min.x, brief_max.y),
            bottom_left: FVec2::new(brief_max.x, brief_min.y),
            bottom_right: FVec2::new(brief_min.x, brief_min.y),
            top_right: FVec2::new(brief_max.x, brief_max.y)
        };
        SchriftartGlipheEingang { 
            pos_rahmen : pos_rechteck,
            brief_rahmen : breif_rechteck
        }
    }
    pub fn get_entry(&self, breif: char) -> &SchriftartGlipheEingang {
        self.briefe_eintrage.get(&breif).unwrap()
    }
    // stellen_skala (probably a mistranslation)
    pub fn set_scale(&mut self, skala: f32) {
        self.skala = skala;
    }
}
// "Paragraph" in englisch 
pub struct Absatz<'a> {
    string: String,
    atlanten: Vec<&'a Kalligraphie>,
    schriftarten: Vec<usize>
}
impl<'a> Absatz<'a> {
    // neu
    pub fn new() -> Self {
        Self { string: String::new(), atlanten: Vec::new(), schriftarten: Vec::new() }
    }
    pub fn add_font(&mut self, device: std::sync::Arc<LogicalDevice>, font: &'a Kalligraphie) -> usize {
        let idx = self.atlanten.len();
        self.atlanten.push(font);
        idx
    }
    pub fn push_str(&mut self, string: &str, idx: usize) {
        // länge
        let length = self.string.len()-1;
        let mut v = vec![idx; string.len()-1];
        self.schriftarten.append(&mut v);
        self.string.push_str(string);
    }
    pub fn iter(&self) -> AbsatzIter {
        AbsatzIter { absatz: self, aktuell: 0, current_offset: 0.0 }
    }
}
pub struct AbsatzIter<'a> {
    absatz: &'a Absatz<'a>,
    aktuell: usize,
    current_offset: f32,
}
pub struct BreifInfo<'a> {
    pub breif: char,
    pub offset: f32,
    pub gliphe: &'a SchriftartGlipheEingang,
}
impl<'a> Iterator for AbsatzIter<'a> {
    fn next(&mut self) -> Option<Self::Item> {
        if self.aktuell > self.absatz.string.len() {
            return None;
        }
        let breif = self.absatz.string.chars().nth(self.aktuell).unwrap();
        let font = self.absatz.atlanten[self.absatz.schriftarten[self.aktuell]];
        let bbox = font.schriftart_atlas.place_letter(breif);
        let entry = font.get_entry(breif);
        
        self.current_offset += bbox.0.x;
        self.aktuell += 1;
        Some(BreifInfo { breif, offset: self.current_offset, gliphe: entry })
    }
    type Item = BreifInfo<'a>;
}
// In case I dont want to use german
type Paragraph<'a> = Absatz<'a>;
type ParagraphIter<'a> = AbsatzIter<'a>;