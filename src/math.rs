pub type Real = f32;

#[derive(Default, Copy, Clone)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

#[derive(Default, Copy, Clone)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

pub fn vec2(x: i32, y: i32) -> Vec2i { Vec2i { x, y } }

pub fn rect(x: i32, y: i32, w: i32, h: i32) -> Rect { Rect { x, y, w, h } }

pub fn expand_rect(r: Rect, n: i32) -> Rect { rect(r.x - n, r.y - n, r.w + n * 2, r.h + n * 2) }

pub fn intersect_rects(r1: Rect, r2: Rect) -> Rect {
    let x1 = i32::max(r1.x, r2.x);
    let y1 = i32::max(r1.y, r2.y);
    let mut x2 = i32::min(r1.x + r1.w, r2.x + r2.w);
    let mut y2 = i32::min(r1.y + r1.h, r2.y + r2.h);
    if x2 < x1 {
        x2 = x1;
    }
    if y2 < y1 {
        y2 = y1;
    }
    rect(x1, y1, x2 - x1, y2 - y1)
}

pub fn rect_overlaps_vec2(r: Rect, p: Vec2i) -> bool { p.x >= r.x && p.x < r.x + r.w && p.y >= r.y && p.y < r.y + r.h }
