use crate::math::Vec2i;

#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub fn color(r: u8, g: u8, b: u8, a: u8) -> Color { Color { r, g, b, a } }

#[derive(Copy, Clone)]
pub struct Style {
    pub font: FontId,
    pub size: Vec2i,
    pub padding: i32,
    pub spacing: i32,
    pub indent: i32,
    pub title_height: i32,
    pub scrollbar_size: i32,
    pub thumb_size: i32,
    pub colors: [Color; 14],
}

impl Default for Style {
    fn default() -> Self {
        Self {
            font: FontId(0),
            size: Vec2i { x: 68, y: 10 },
            padding: 5,
            spacing: 4,
            indent: 24,
            title_height: 24,
            scrollbar_size: 12,
            thumb_size: 8,
            colors: [
                Color { r: 230, g: 230, b: 230, a: 255 },
                Color { r: 25, g: 25, b: 25, a: 255 },
                Color { r: 50, g: 50, b: 50, a: 255 },
                Color { r: 25, g: 25, b: 25, a: 255 },
                Color { r: 240, g: 240, b: 240, a: 255 },
                Color { r: 0, g: 0, b: 0, a: 0 },
                Color { r: 75, g: 75, b: 75, a: 255 },
                Color { r: 95, g: 95, b: 95, a: 255 },
                Color { r: 115, g: 115, b: 115, a: 255 },
                Color { r: 30, g: 30, b: 30, a: 255 },
                Color { r: 35, g: 35, b: 35, a: 255 },
                Color { r: 40, g: 40, b: 40, a: 255 },
                Color { r: 43, g: 43, b: 43, a: 255 },
                Color { r: 30, g: 30, b: 30, a: 255 },
            ],
        }
    }
}

#[derive(Copy, Clone)]
pub struct FontId(pub usize);

pub trait Font {
    fn name(&self) -> &str;
    fn get_size(&self) -> usize;
    fn get_char_size(&self, c: char) -> (usize, usize);
}
