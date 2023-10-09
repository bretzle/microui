use super::Context;
use crate::math::{max, rect, vec2, Rect, Vec2};

#[derive(Default, PartialEq, Copy, Clone)]
pub enum LayoutPosition {
    #[default]
    None,
    Relative,
    Absolute,
}

#[derive(Default, Copy, Clone)]
pub struct Layout {
    pub body: Rect,
    pub next: Rect,
    pub position: Vec2,
    pub size: Vec2,
    pub max: Vec2,
    pub widths: [i32; 16],
    pub items: usize,
    pub item_index: usize,
    pub next_row: i32,
    pub next_type: LayoutPosition,
    pub indent: i32,
}

impl Layout {
    pub fn row(&mut self, widths: &[i32], height: i32) {
        self.items = widths.len();
        assert!(widths.len() <= 16);
        self.widths[..widths.len()].copy_from_slice(widths);
        self.position = vec2(self.indent, self.next_row);
        self.size.y = height;
        self.item_index = 0;
    }
}

impl Context {
    pub(crate) fn push_layout(&mut self, body: Rect, scroll: Vec2) {
        self.layout_stack.push(Layout {
            body: rect(body.x - scroll.x, body.y - scroll.y, body.w, body.h),
            next: Rect::ZERO,
            position: Vec2 { x: 0, y: 0 },
            size: Vec2 { x: 0, y: 0 },
            max: vec2(-0x1000000, -0x1000000),
            widths: [0; 16],
            items: 0,
            item_index: 0,
            next_row: 0,
            next_type: LayoutPosition::None,
            indent: 0,
        });
        self.layout_row(&[0], 0);
    }

    pub(crate) fn get_layout(&self) -> &Layout { return self.layout_stack.last().unwrap(); }

    pub(crate) fn get_layout_mut(&mut self) -> &mut Layout { return self.layout_stack.last_mut().unwrap(); }

    pub fn layout_begin_column(&mut self) {
        let layout = self.layout_next();
        self.push_layout(layout, vec2(0, 0));
    }

    pub fn layout_end_column(&mut self) {
        let b = *self.get_layout();
        self.layout_stack.pop();

        let a = self.get_layout_mut();
        a.position.x = if a.position.x > b.position.x + b.body.x - a.body.x {
            a.position.x
        } else {
            b.position.x + b.body.x - a.body.x
        };
        a.next_row = if a.next_row > b.next_row + b.body.y - a.body.y {
            a.next_row
        } else {
            b.next_row + b.body.y - a.body.y
        };
        a.max.x = max(a.max.x, b.max.x);
        a.max.y = max(a.max.y, b.max.y);
    }

    pub fn layout_row(&mut self, widths: &[i32], height: i32) { self.get_layout_mut().row(widths, height); }

    pub fn layout_width(&mut self, width: i32) { self.get_layout_mut().size.x = width; }

    pub fn layout_height(&mut self, height: i32) { self.get_layout_mut().size.y = height; }

    pub fn layout_set_next(&mut self, r: Rect, position: LayoutPosition) {
        let layout = self.get_layout_mut();
        layout.next = r;
        layout.next_type = position;
    }

    pub fn layout_next(&mut self) -> Rect {
        let style = self.style;
        let layout = self.get_layout_mut();
        let mut res = Rect { x: 0, y: 0, w: 0, h: 0 };
        if layout.next_type != LayoutPosition::None {
            let type_0 = layout.next_type;
            layout.next_type = LayoutPosition::None;
            res = layout.next;
            if type_0 == LayoutPosition::Absolute {
                self.last_rect = res;
                return self.last_rect;
            }
        } else {
            let litems = layout.items;
            let lsize_y = layout.size.y;
            let mut undefined_widths = [0; 16];
            undefined_widths[0..litems].copy_from_slice(&layout.widths[0..litems]);
            if layout.item_index == layout.items {
                layout.row(&undefined_widths[0..litems], lsize_y);
            }
            res.x = layout.position.x;
            res.y = layout.position.y;
            res.w = if layout.items > 0 { layout.widths[layout.item_index] } else { layout.size.x };
            res.h = layout.size.y;
            if res.w == 0 {
                res.w = style.size.x + style.padding * 2;
            }
            if res.h == 0 {
                res.h = style.size.y + style.padding * 2;
            }
            if res.w < 0 {
                res.w += layout.body.w - res.x + 1;
            }
            if res.h < 0 {
                res.h += layout.body.h - res.y + 1;
            }
            layout.item_index += 1;
        }
        layout.position.x += res.w + style.spacing;
        layout.next_row = if layout.next_row > res.y + res.h + style.spacing {
            layout.next_row
        } else {
            res.y + res.h + style.spacing
        };
        res.x += layout.body.x;
        res.y += layout.body.y;
        layout.max.x = if layout.max.x > res.x + res.w { layout.max.x } else { res.x + res.w };
        layout.max.y = if layout.max.y > res.y + res.h { layout.max.y } else { res.y + res.h };
        self.last_rect = res;
        self.last_rect
    }
}
