#[derive(Default, Copy, Clone)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

pub const fn vec2(x: i32, y: i32) -> Vec2 { Vec2 { x, y } }

#[derive(Default, Copy, Clone, PartialEq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    pub const ZERO: Self = rect(0, 0, 0, 0);
}

pub const fn rect(x: i32, y: i32, w: i32, h: i32) -> Rect { Rect { x, y, w, h } }

impl Rect {
    pub const UNCLIPPED: Rect = rect(0, 0, 0x1000000, 0x1000000);

    pub const fn expand(&self, n: i32) -> Self { rect(self.x - n, self.y - n, self.w + n * 2, self.h + n * 2) }

    pub const fn intersect(&self, r2: Self) -> Self {
        let x1 = max(self.x, r2.x);
        let y1 = max(self.y, r2.y);
        let mut x2 = min(self.x + self.w, r2.x + r2.w);
        let mut y2 = min(self.y + self.h, r2.y + r2.h);
        if x2 < x1 {
            x2 = x1;
        }
        if y2 < y1 {
            y2 = y1;
        }

        rect(x1, y1, x2 - x1, y2 - y1)
    }

    pub fn overlaps(&self, p: Vec2) -> bool { p.x >= self.x && p.x < self.x + self.w && p.y >= self.y && p.y < self.y + self.h }
}

#[inline]
pub(crate) const fn min(a: i32, b: i32) -> i32 {
    if a < b {
        a
    } else {
        b
    }
}

#[inline]
pub(crate) const fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

#[inline]
pub(crate) const fn clamp(x: i32, a: i32, b: i32) -> i32 { min(b, max(a, x)) }
