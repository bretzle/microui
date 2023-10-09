mod button;
mod header;
mod panel;
mod tree;
mod window;

pub use window::*;

bitflags::bitflags! {
    #[derive(Clone, Copy)]
    pub struct WidgetOption : u32 {
        const ALIGN_CENTER = 1 << 0;
        const ALIGN_RIGHT  = 1 << 1;
        const NO_INTERACT  = 1 << 2;
        const NO_FRAME     = 1 << 3;
        const NO_RESIZE    = 1 << 4;
        const NO_SCROLL    = 1 << 5;
        const NO_CLOSE     = 1 << 6;
        const NO_TITLE     = 1 << 7;
        const HOLD_FOCUS   = 1 << 8;
        const AUTO_SIZE    = 1 << 9;
        const POPUP        = 1 << 10;
        const CLOSED       = 1 << 11;
        const EXPANDED     = 1 << 12;
    }
}

impl WidgetOption {
    pub const fn is_expanded(&self) -> bool { self.intersects(WidgetOption::EXPANDED) }
    pub const fn is_closed(&self) -> bool { self.intersects(WidgetOption::CLOSED) }
    pub const fn is_popup(&self) -> bool { self.intersects(WidgetOption::POPUP) }
    pub const fn is_auto_sizing(&self) -> bool { self.intersects(WidgetOption::AUTO_SIZE) }
    pub const fn is_holding_focus(&self) -> bool { self.intersects(WidgetOption::HOLD_FOCUS) }
    pub const fn has_no_title(&self) -> bool { self.intersects(WidgetOption::NO_TITLE) }
    pub const fn has_no_close(&self) -> bool { self.intersects(WidgetOption::NO_CLOSE) }
    pub const fn has_no_scroll(&self) -> bool { self.intersects(WidgetOption::NO_SCROLL) }
    pub const fn is_fixed(&self) -> bool { self.intersects(WidgetOption::NO_RESIZE) }
    pub const fn has_no_frame(&self) -> bool { self.intersects(WidgetOption::NO_FRAME) }
    pub const fn is_not_interactive(&self) -> bool { self.intersects(WidgetOption::NO_INTERACT) }
    pub const fn is_aligned_right(&self) -> bool { self.intersects(WidgetOption::ALIGN_RIGHT) }
    pub const fn is_aligned_center(&self) -> bool { self.intersects(WidgetOption::ALIGN_CENTER) }
    pub const fn is_none(&self) -> bool { self.is_empty() }
}
