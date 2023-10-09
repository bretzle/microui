use super::Context;
use crate::math::vec2;
use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy)]
    pub struct MouseButton : u32 {
        const LEFT   = 1 << 0;
        const RIGHT  = 1 << 1;
        const MIDDLE = 1 << 2;
    }
}

impl MouseButton {
    pub const fn is_middle(&self) -> bool { self.intersects(Self::MIDDLE) }
    pub const fn is_right(&self) -> bool { self.intersects(Self::RIGHT) }
    pub const fn is_left(&self) -> bool { self.intersects(Self::LEFT) }
    pub const fn is_none(&self) -> bool { self.is_empty() }
}

bitflags! {
    #[derive(Clone, Copy)]
    pub struct KeyMode : u32 {
        const SHIFT     = 1 << 0;
        const CTRL      = 1 << 1;
        const ALT       = 1 << 2;
        const BACKSPACE = 1 << 3;
        const RETURN    = 1 << 4;
    }
}

impl KeyMode {
    pub const fn is_none(&self) -> bool { self.is_empty() }
    pub const fn is_return(&self) -> bool { self.intersects(Self::RETURN) }
    pub const fn is_backspace(&self) -> bool { self.intersects(Self::BACKSPACE) }
    pub const fn is_alt(&self) -> bool { self.intersects(Self::ALT) }
    pub const fn is_ctrl(&self) -> bool { self.intersects(Self::CTRL) }
    pub const fn is_shift(&self) -> bool { self.intersects(Self::SHIFT) }
}

impl Context {
    pub fn input_mousemove(&mut self, x: i32, y: i32) { self.mouse_pos = vec2(x, y); }

    pub fn input_mousedown(&mut self, btn: MouseButton) {
        self.mouse_down |= btn;
        self.mouse_pressed |= btn;
    }

    pub fn input_mouseup(&mut self, btn: MouseButton) { self.mouse_down &= !btn; }

    pub fn input_scroll(&mut self, x: i32, y: i32) {
        self.scroll_delta.x += x;
        self.scroll_delta.y += y;
    }

    pub fn input_keydown(&mut self, key: KeyMode) {
        self.key_pressed |= key;
        self.key_down |= key;
    }

    pub fn input_keyup(&mut self, key: KeyMode) { self.key_down &= !key; }

    pub fn input_text(&mut self, text: &str) { self.input_text += text; }
}
