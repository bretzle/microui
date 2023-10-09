#![feature(portable_simd)]

use self::pool::Pool;
use bitflags::*;
use std::fmt::Write;

pub mod atlas;
mod color;
mod input;
mod layout;
mod math;
mod pool;
mod util;
mod widget;

pub use self::input::*;
pub use self::layout::*;
pub use self::math::*;
pub use self::util::*;
pub use self::widget::*;
pub use self::color::*;

#[derive(PartialEq, Copy, Clone)]
pub enum Clip {
    None,
    Part,
    All,
}

#[derive(PartialEq, Copy, Clone)]
pub enum ControlColor {
    Text,
    Border,
    WindowBG,
    TitleBG,
    TitleText,
    PanelBG,
    Button,
    ButtonHover,
    ButtonFocus,
    Base,
    BaseHover,
    BaseFocus,
    ScrollBase,
    ScrollThumb,
}

impl ControlColor {
    pub fn hover(&mut self) {
        *self = match self {
            Self::Base => Self::BaseHover,
            Self::Button => Self::ButtonHover,
            _ => *self,
        }
    }

    pub fn focus(&mut self) {
        *self = match self {
            Self::Base => Self::BaseFocus,
            Self::Button => Self::ButtonFocus,
            Self::BaseHover => Self::BaseFocus,
            Self::ButtonHover => Self::ButtonFocus,
            _ => *self,
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum Icon {
    Close = 1,
    Check,
    Collapsed,
    Expanded,
}

bitflags! {
    pub struct ResourceState : u32 {
        const ACTIVE = 1 << 0;
        const SUBMIT = 1 << 1;
        const CHANGE = 1 << 2;
    }
}

impl ResourceState {
    pub const fn is_changed(&self) -> bool { self.intersects(Self::CHANGE) }
    pub const fn is_submitted(&self) -> bool { self.intersects(Self::SUBMIT) }
    pub const fn is_active(&self) -> bool { self.intersects(Self::ACTIVE) }
    pub const fn is_none(&self) -> bool { self.is_empty() }
}

pub struct Context {
    pub char_width: fn(FontId, char) -> usize,
    pub font_height: fn(FontId) -> usize,
    pub style: Style,
    pub hover: Option<Id>,
    pub focus: Option<Id>,
    pub last_id: Option<Id>,
    pub last_rect: Rect,
    pub last_zindex: i32,
    pub updated_focus: bool,
    pub frame: usize,
    pub hover_root: Option<usize>,
    pub next_hover_root: Option<usize>,
    pub scroll_target: Option<usize>,
    pub number_edit_buf: String,
    pub number_edit: Option<Id>,
    pub command_list: Vec<Command>,
    pub root_list: Vec<usize>,
    pub container_stack: Vec<usize>,
    pub clip_stack: Vec<Rect>,
    pub id_stack: Vec<Id>,
    pub layout_stack: Vec<Layout>,
    pub text_stack: String,
    pub container_pool: Pool<48>,
    pub containers: [Container; 48],
    pub treenode_pool: Pool<48>,
    pub mouse_pos: Vec2,
    pub last_mouse_pos: Vec2,
    pub mouse_delta: Vec2,
    pub scroll_delta: Vec2,
    pub mouse_down: MouseButton,
    pub mouse_pressed: MouseButton,
    pub key_down: KeyMode,
    pub key_pressed: KeyMode,
    pub input_text: String,
}

#[derive(Default, Copy, Clone)]
pub struct Container {
    pub head_idx: Option<usize>,
    pub tail_idx: Option<usize>,
    pub rect: Rect,
    pub body: Rect,
    pub content_size: Vec2,
    pub scroll: Vec2,
    pub zindex: i32,
    pub open: bool,
}

#[derive(Copy, Clone)]
pub enum Command {
    Clip {
        rect: Rect,
    },
    Rect {
        rect: Rect,
        color: Color,
    },
    Text {
        font: FontId,
        pos: Vec2,
        color: Color,
        str_start: usize,
        str_len: usize,
    },
    Icon {
        rect: Rect,
        id: Icon,
        color: Color,
    },
}

#[derive(Copy, Clone)]
pub struct FontId(pub usize);

#[derive(Copy, Clone)]
pub struct Style {
    pub font: FontId,
    pub size: Vec2,
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
            size: Vec2 { x: 68, y: 10 },
            padding: 5,
            spacing: 4,
            indent: 24,
            title_height: 24,
            scrollbar_size: 12,
            thumb_size: 8,
            colors: [
                Color::rgba(230, 230, 230, 255),
                Color::rgba(25, 25, 25, 255),
                Color::rgba(50, 50, 50, 255),
                Color::rgba(25, 25, 25, 255),
                Color::rgba(240, 240, 240, 255),
                Color::rgba(0, 0, 0, 0),
                Color::rgba(75, 75, 75, 255),
                Color::rgba(95, 95, 95, 255),
                Color::rgba(115, 115, 115, 255),
                Color::rgba(30, 30, 30, 255),
                Color::rgba(35, 35, 35, 255),
                Color::rgba(40, 40, 40, 255),
                Color::rgba(43, 43, 43, 255),
                Color::rgba(30, 30, 30, 255),
            ],
        }
    }
}

impl Context {
    pub fn new(char_width: fn(FontId, char) -> usize, font_height: fn(FontId) -> usize) -> Self {
        Self {
            char_width,
            font_height,
            style: Style::default(),
            hover: None,
            focus: None,
            last_id: None,
            last_rect: Rect::ZERO,
            last_zindex: 0,
            updated_focus: false,
            frame: 0,
            hover_root: None,
            next_hover_root: None,
            scroll_target: None,
            number_edit_buf: String::new(),
            number_edit: None,
            command_list: vec![],
            root_list: vec![],
            container_stack: vec![],
            clip_stack: vec![],
            id_stack: vec![],
            layout_stack: vec![],
            text_stack: String::new(),
            container_pool: Pool::default(),
            containers: [Container::default(); 48],
            treenode_pool: Pool::default(),
            mouse_pos: Vec2::default(),
            last_mouse_pos: Vec2::default(),
            mouse_delta: Vec2::default(),
            scroll_delta: Vec2::default(),
            mouse_down: MouseButton::empty(),
            mouse_pressed: MouseButton::empty(),
            key_down: KeyMode::empty(),
            key_pressed: KeyMode::empty(),
            input_text: String::new(),
        }
    }

    pub(crate) fn draw_frame(&mut self, rect: Rect, colorid: ControlColor) {
        self.draw_rect(rect, self.style.colors[colorid as usize]);
        if colorid == ControlColor::ScrollBase || colorid == ControlColor::ScrollThumb || colorid == ControlColor::TitleBG {
            return;
        }
        if self.style.colors[ControlColor::Border as usize].a != 0 {
            self.draw_box(rect.expand(1), self.style.colors[ControlColor::Border as usize]);
        }
    }

    pub fn frame(&mut self, f: impl FnOnce(&mut Self)) {
        self.root_list.clear();
        self.text_stack.clear();
        self.scroll_target = None;
        self.hover_root = self.next_hover_root;
        self.next_hover_root = None;
        self.mouse_delta.x = self.mouse_pos.x - self.last_mouse_pos.x;
        self.mouse_delta.y = self.mouse_pos.y - self.last_mouse_pos.y;
        self.command_list.clear();
        self.frame += 1;

        f(self);

        assert_eq!(self.container_stack.len(), 0);
        assert_eq!(self.clip_stack.len(), 0);
        assert_eq!(self.id_stack.len(), 0);
        assert_eq!(self.layout_stack.len(), 0);
        if self.scroll_target.is_some() {
            self.containers[self.scroll_target.unwrap()].scroll.x += self.scroll_delta.x;
            self.containers[self.scroll_target.unwrap()].scroll.y += self.scroll_delta.y;
        }
        if !self.updated_focus {
            self.focus = None;
        }
        self.updated_focus = false;
        if !self.mouse_pressed.is_none()
            && self.next_hover_root.is_some()
            && self.containers[self.next_hover_root.unwrap()].zindex < self.last_zindex
            && self.containers[self.next_hover_root.unwrap()].zindex >= 0
        {
            self.bring_to_front(self.next_hover_root.unwrap());
        }
        self.key_pressed = KeyMode::empty();
        self.input_text.clear();
        self.mouse_pressed = MouseButton::empty();
        self.scroll_delta = vec2(0, 0);
        self.last_mouse_pos = self.mouse_pos;
        self.root_list
            .sort_unstable_by(|a, b| self.containers[*a].zindex.cmp(&self.containers[*b].zindex));
    }

    pub fn set_focus(&mut self, id: Option<Id>) {
        self.focus = id;
        self.updated_focus = true;
    }

    pub fn get_id_u32(&mut self, orig_id: u32) -> Id {
        let mut res = self.id_stack.last().copied().unwrap_or(Id::START);
        res.hash(orig_id);
        self.last_id = Some(res);
        res
    }

    pub fn get_id_from_ptr<T: ?Sized>(&mut self, orig_id: &T) -> Id {
        let mut res = self.id_stack.last().copied().unwrap_or(Id::START);
        let ptr = orig_id as *const T as *const u8 as usize;
        let bytes = ptr.to_le_bytes();
        res.hash(&bytes[..]);
        self.last_id = Some(res);
        res
    }

    pub fn get_id_from_str(&mut self, s: &str) -> Id {
        let mut res = self.id_stack.last().copied().unwrap_or(Id::START);
        res.hash(s);
        self.last_id = Some(res);
        res
    }

    pub fn push_id_from_ptr<T>(&mut self, orig_id: &T) {
        let id = self.get_id_from_ptr(orig_id);
        self.id_stack.push(id);
    }

    pub fn push_id_from_str(&mut self, s: &str) {
        let id = self.get_id_from_str(s);
        self.id_stack.push(id);
    }

    pub fn pop_id(&mut self) { self.id_stack.pop(); }

    pub fn push_clip_rect(&mut self, rect: Rect) {
        let last = self.get_clip_rect();
        self.clip_stack.push(rect.intersect(last));
    }

    pub fn pop_clip_rect(&mut self) { self.clip_stack.pop(); }

    pub fn get_clip_rect(&mut self) -> Rect { *self.clip_stack.last().unwrap() }

    pub fn check_clip(&mut self, r: Rect) -> Clip {
        let cr = self.get_clip_rect();
        if r.x > cr.x + cr.w || r.x + r.w < cr.x || r.y > cr.y + cr.h || r.y + r.h < cr.y {
            return Clip::All;
        }
        if r.x >= cr.x && r.x + r.w <= cr.x + cr.w && r.y >= cr.y && r.y + r.h <= cr.y + cr.h {
            return Clip::None;
        }
        Clip::Part
    }

    fn pop_container(&mut self) {
        let cnt = self.get_current_container();
        let layout = *self.get_layout();
        self.containers[cnt].content_size.x = layout.max.x - layout.body.x;
        self.containers[cnt].content_size.y = layout.max.y - layout.body.y;

        self.container_stack.pop();
        self.layout_stack.pop();
        self.pop_id();
    }

    fn get_current_container(&self) -> usize { *self.container_stack.last().unwrap() }

    pub fn get_current_container_rect(&self) -> Rect { self.containers[*self.container_stack.last().unwrap()].rect }

    pub fn set_current_container_rect(&mut self, rect: &Rect) { self.containers[*self.container_stack.last().unwrap()].rect = *rect; }

    pub fn get_current_container_scroll(&self) -> Vec2 { self.containers[*self.container_stack.last().unwrap()].scroll }

    pub fn set_current_container_scroll(&mut self, scroll: &Vec2) { self.containers[*self.container_stack.last().unwrap()].scroll = *scroll; }

    pub fn get_current_container_content_size(&self) -> Vec2 { self.containers[*self.container_stack.last().unwrap()].content_size }

    pub fn get_current_container_body(&self) -> Rect { self.containers[*self.container_stack.last().unwrap()].body }

    fn get_container_index_intern(&mut self, id: Id, opt: WidgetOption) -> Option<usize> {
        if let ret @ Some(idx) = self.container_pool.get(id) {
            if self.containers[idx].open || !opt.is_closed() {
                self.container_pool.update(idx, self.frame);
            }
            return ret;
        }
        if opt.is_closed() {
            return None;
        }
        let idx = self.container_pool.alloc(id, self.frame);
        self.containers[idx] = Container::default();
        self.containers[idx].head_idx = None;
        self.containers[idx].tail_idx = None;
        self.containers[idx].open = true;
        self.bring_to_front(idx);
        Some(idx)
    }

    fn get_container_index(&mut self, name: &str) -> Option<usize> {
        let id = self.get_id_from_str(name);
        self.get_container_index_intern(id, WidgetOption::empty())
    }

    pub fn bring_to_front(&mut self, cnt: usize) {
        self.last_zindex += 1;
        self.containers[cnt].zindex = self.last_zindex;
    }

    fn push_command(&mut self, cmd: Command) -> (&mut Command, usize) {
        self.command_list.push(cmd);
        let idx = self.command_list.len() - 1;
        (self.command_list.last_mut().unwrap(), idx)
    }

    pub fn push_text(&mut self, str: &str) -> usize {
        let str_start = self.text_stack.len();
        self.text_stack.push_str(str);
        str_start
    }

    pub fn commands(&self) -> impl Iterator<Item = &Command> + '_ {
        self.root_list.iter().flat_map(|&idx| {
            let container = &self.containers[idx];
            let head = container.head_idx.unwrap();
            let tail = container.tail_idx.unwrap();
            self.command_list[head..tail].iter()
        })
    }

    fn jump(&mut self) -> usize { self.command_list.len() }

    pub fn set_clip(&mut self, rect: Rect) { self.push_command(Command::Clip { rect }); }

    pub fn draw_rect(&mut self, mut rect: Rect, color: Color) {
        rect = rect.intersect(self.get_clip_rect());
        if rect.w > 0 && rect.h > 0 {
            self.push_command(Command::Rect { rect, color });
        }
    }

    pub fn draw_box(&mut self, r: Rect, color: Color) {
        self.draw_rect(rect(r.x + 1, r.y, r.w - 2, 1), color);
        self.draw_rect(rect(r.x + 1, r.y + r.h - 1, r.w - 2, 1), color);
        self.draw_rect(rect(r.x, r.y, 1, r.h), color);
        self.draw_rect(rect(r.x + r.w - 1, r.y, 1, r.h), color);
    }

    pub fn draw_text(&mut self, font: FontId, str: &str, pos: Vec2, color: Color) {
        let rect: Rect = rect(pos.x, pos.y, self.get_text_width(font, str), self.get_text_height(font, str));
        let clipped = self.check_clip(rect);
        match clipped {
            Clip::All => return,
            Clip::Part => {
                let clip = self.get_clip_rect();
                self.set_clip(clip)
            }
            _ => (),
        }

        let str_start = self.push_text(str);
        self.push_command(Command::Text {
            str_start,
            str_len: str.len(),
            pos,
            color,
            font,
        });
        if clipped != Clip::None {
            self.set_clip(Rect::UNCLIPPED);
        }
    }

    pub fn draw_icon(&mut self, id: Icon, rect: Rect, color: Color) {
        let clipped = self.check_clip(rect);
        match clipped {
            Clip::All => return,
            Clip::Part => {
                let clip = self.get_clip_rect();
                self.set_clip(clip)
            }
            _ => (),
        }
        self.push_command(Command::Icon { id, rect, color });
        if clipped != Clip::None {
            self.set_clip(Rect::UNCLIPPED);
        }
    }

    fn in_hover_root(&mut self) -> bool {
        match self.hover_root {
            Some(hover_root) => {
                let len = self.container_stack.len();
                for i in 0..len {
                    if self.container_stack[len - i - 1] == hover_root {
                        return true;
                    }
                    if self.containers[self.container_stack[len - i - 1]].head_idx.is_some() {
                        break;
                    }
                }
                false
            }
            None => false,
        }
    }

    pub fn draw_control_frame(&mut self, id: Id, rect: Rect, mut colorid: ControlColor, opt: WidgetOption) {
        if opt.has_no_frame() {
            return;
        }

        if self.focus == Some(id) {
            colorid.focus()
        } else if self.hover == Some(id) {
            colorid.hover()
        }
        self.draw_frame(rect, colorid);
    }

    pub fn draw_control_text(&mut self, str: &str, rect: Rect, colorid: ControlColor, opt: WidgetOption) {
        let mut pos: Vec2 = Vec2 { x: 0, y: 0 };
        let font = self.style.font;
        let tw = self.get_text_width(font, str);
        self.push_clip_rect(rect);
        pos.y = rect.y + (rect.h - self.get_text_height(font, str)) / 2;
        if opt.is_aligned_center() {
            pos.x = rect.x + (rect.w - tw) / 2;
        } else if opt.is_aligned_right() {
            pos.x = rect.x + rect.w - tw - self.style.padding;
        } else {
            pos.x = rect.x + self.style.padding;
        }
        self.draw_text(font, str, pos, self.style.colors[colorid as usize]);
        self.pop_clip_rect();
    }

    pub fn mouse_over(&mut self, rect: Rect) -> bool { rect.overlaps(self.mouse_pos) && self.get_clip_rect().overlaps(self.mouse_pos) && self.in_hover_root() }

    pub fn update_control(&mut self, id: Id, rect: Rect, opt: WidgetOption) {
        let mouseover = self.mouse_over(rect);
        if self.focus == Some(id) {
            self.updated_focus = true;
        }
        if opt.is_not_interactive() {
            return;
        }
        if mouseover && self.mouse_down.is_none() {
            self.hover = Some(id);
        }
        if self.focus == Some(id) {
            if !self.mouse_pressed.is_none() && !mouseover {
                self.set_focus(None);
            }
            if self.mouse_down.is_none() && !opt.is_holding_focus() {
                self.set_focus(None);
            }
        }
        if self.hover == Some(id) {
            if !self.mouse_pressed.is_none() {
                self.set_focus(Some(id));
            } else if !mouseover {
                self.hover = None;
            }
        }
    }

    pub fn get_text_width(&self, font: FontId, text: &str) -> i32 {
        let mut res = 0;
        let mut acc = 0;
        for c in text.chars() {
            if c == '\n' {
                res = usize::max(res, acc);
                acc = 0;
            }
            acc += (self.char_width)(font, c);
        }
        res = usize::max(res, acc);
        res as i32
    }

    pub fn get_text_height(&self, font: FontId, text: &str) -> i32 {
        let font_height = (self.font_height)(font);
        let lc = text.lines().count();
        (lc * font_height) as i32
    }

    pub fn text(&mut self, text: &str) {
        let font = self.style.font;
        let color = self.style.colors[ControlColor::Text as usize];
        self.layout_begin_column();
        let h = (self.font_height)(font) as i32;
        self.layout_row(&[-1], h);
        let mut r = self.layout_next();
        for line in text.lines() {
            let mut rx = r.x;
            let words = line.split_inclusive(' ');
            for w in words {
                // TODO: split w when its width > w into many lines
                let tw = self.get_text_width(font, w);
                if tw + rx < r.x + r.w {
                    self.draw_text(font, w, vec2(rx, r.y), color);
                    rx += tw;
                } else {
                    r = self.layout_next();
                    rx = r.x;
                }
            }
            r = self.layout_next();
        }
        self.layout_end_column();
    }

    pub fn label(&mut self, text: &str) {
        let layout = self.layout_next();
        self.draw_control_text(text, layout, ControlColor::Text, WidgetOption::empty());
    }

    pub fn checkbox(&mut self, label: &str, state: &mut bool) -> ResourceState {
        let mut res = ResourceState::empty();
        let id: Id = self.get_id_from_ptr(state);
        let mut r: Rect = self.layout_next();
        let box_0: Rect = rect(r.x, r.y, r.h, r.h);
        self.update_control(id, r, WidgetOption::empty());
        if self.mouse_pressed.is_left() && self.focus == Some(id) {
            res |= ResourceState::CHANGE;
            *state = !(*state);
        }
        self.draw_control_frame(id, box_0, ControlColor::Base, WidgetOption::empty());
        if *state {
            self.draw_icon(Icon::Check, box_0, self.style.colors[ControlColor::Text as usize]);
        }
        r = rect(r.x + box_0.w, r.y, r.w - box_0.w, r.h);
        self.draw_control_text(label, r, ControlColor::Text, WidgetOption::empty());
        res
    }

    pub fn textbox_raw(&mut self, buf: &mut String, id: Id, r: Rect, opt: WidgetOption) -> ResourceState {
        let mut res = ResourceState::empty();
        self.update_control(id, r, opt | WidgetOption::HOLD_FOCUS);
        if self.focus == Some(id) {
            let mut len = buf.len();

            if !self.input_text.is_empty() {
                buf.push_str(&self.input_text);
                len += self.input_text.len();
                res |= ResourceState::CHANGE
            }

            if self.key_pressed.is_backspace() && len > 0 {
                // skip utf-8 continuation bytes
                buf.pop();
                res |= ResourceState::CHANGE
            }
            if self.key_pressed.is_return() {
                self.set_focus(None);
                res |= ResourceState::SUBMIT;
            }
        }
        self.draw_control_frame(id, r, ControlColor::Base, opt);
        if self.focus == Some(id) {
            let color = self.style.colors[ControlColor::Text as usize];
            let font = self.style.font;
            let textw = self.get_text_width(font, buf.as_str());
            let texth = self.get_text_height(font, buf.as_str());
            let ofx = r.w - self.style.padding - textw - 1;
            let textx = r.x + (if ofx < self.style.padding { ofx } else { self.style.padding });
            let texty = r.y + (r.h - texth) / 2;
            self.push_clip_rect(r);
            self.draw_text(font, buf.as_str(), vec2(textx, texty), color);
            self.draw_rect(rect(textx + textw, texty, 1, texth), color);
            self.pop_clip_rect();
        } else {
            self.draw_control_text(buf.as_str(), r, ControlColor::Text, opt);
        }
        res
    }

    fn number_textbox(&mut self, precision: usize, value: &mut f32, r: Rect, id: Id) -> ResourceState {
        if self.mouse_pressed.is_left() && self.key_down.is_shift() && self.hover == Some(id) {
            self.number_edit = Some(id);
            self.number_edit_buf.clear();
            let _ = write!(self.number_edit_buf, "{:.*}", precision, value);
        }

        if self.number_edit == Some(id) {
            let mut temp = self.number_edit_buf.clone();
            let res = self.textbox_raw(&mut temp, id, r, WidgetOption::empty());
            self.number_edit_buf = temp;
            if res.is_submitted() || self.focus != Some(id) {
                if let Ok(v) = self.number_edit_buf.parse::<f32>() {
                    *value = v
                }
                self.number_edit = None;
            } else {
                return ResourceState::ACTIVE;
            }
        }
        ResourceState::empty()
    }

    pub fn textbox_ex(&mut self, buf: &mut String, opt: WidgetOption) -> ResourceState {
        let id = self.get_id_from_ptr(buf);
        let r = self.layout_next();
        self.textbox_raw(buf, id, r, opt)
    }

    pub fn slider_ex(&mut self, value: &mut f32, low: f32, high: f32, step: f32, precision: usize, opt: WidgetOption) -> ResourceState {
        let mut res = ResourceState::empty();
        let last = *value;
        let mut v = last;
        let id = self.get_id_from_ptr(value);
        let base = self.layout_next();
        if !self.number_textbox(precision, &mut v, base, id).is_none() {
            return res;
        }
        self.update_control(id, base, opt);
        if self.focus == Some(id) && (!self.mouse_down.is_none() | self.mouse_pressed.is_left()) {
            v = low + (self.mouse_pos.x - base.x) as f32 * (high - low) / base.w as f32;
            if step != 0. {
                v = (v + step / 2.0) / step * step;
            }
        }
        v = if high < (if low > v { low } else { v }) {
            high
        } else if low > v {
            low
        } else {
            v
        };
        *value = v;
        if last != v {
            res |= ResourceState::CHANGE;
        }
        self.draw_control_frame(id, base, ControlColor::Base, opt);
        let w = self.style.thumb_size;
        let x = ((v - low) * (base.w - w) as f32 / (high - low)) as i32;
        let thumb = rect(base.x + x, base.y, w, base.h);
        self.draw_control_frame(id, thumb, ControlColor::Button, opt);
        let buff = format!("{:.*}", precision, value);
        self.draw_control_text(&buff, base, ControlColor::Text, opt);
        res
    }

    pub fn number_ex(&mut self, value: &mut f32, step: f32, precision: usize, opt: WidgetOption) -> ResourceState {
        let mut res = ResourceState::empty();
        let id = self.get_id_from_ptr(value);
        let base = self.layout_next();
        let last = *value;
        if !self.number_textbox(precision, value, base, id).is_none() {
            return res;
        }
        self.update_control(id, base, opt);
        if self.focus == Some(id) && self.mouse_down.is_left() {
            *value += self.mouse_delta.x as f32 * step;
        }
        if *value != last {
            res |= ResourceState::CHANGE;
        }
        self.draw_control_frame(id, base, ControlColor::Base, opt);
        let buff = format!("{:.*}", precision, value);
        self.draw_control_text(&buff, base, ControlColor::Text, opt);
        res
    }

    fn header_ex(&mut self, label: &str, is_treenode: bool, opt: WidgetOption) -> ResourceState {
        let id = self.get_id_from_str(label);
        let idx = self.treenode_pool.get(id);
        self.layout_row(&[-1], 0);
        let mut active = idx.is_some() as i32;
        let expanded = if opt.is_expanded() { (active == 0) as i32 } else { active };
        let mut r = self.layout_next();
        self.update_control(id, r, WidgetOption::empty());
        active ^= (self.mouse_pressed.is_left() && self.focus == Some(id)) as i32;
        if let Some(idx) = idx {
            if active != 0 {
                self.treenode_pool.update(idx, self.frame);
            } else {
                self.treenode_pool.reset(idx);
            }
        } else if active != 0 {
            self.treenode_pool.alloc(id, self.frame);
        }

        if is_treenode {
            if self.hover == Some(id) {
                self.draw_frame(r, ControlColor::ButtonHover);
            }
        } else {
            self.draw_control_frame(id, r, ControlColor::Button, WidgetOption::empty());
        }
        self.draw_icon(
            if expanded != 0 { Icon::Expanded } else { Icon::Collapsed },
            rect(r.x, r.y, r.h, r.h),
            self.style.colors[ControlColor::Text as usize],
        );
        r.x += r.h - self.style.padding;
        r.w -= r.h - self.style.padding;
        self.draw_control_text(label, r, ControlColor::Text, WidgetOption::empty());
        if expanded != 0 {
            ResourceState::ACTIVE
        } else {
            ResourceState::empty()
        }
    }

    fn scrollbars(&mut self, cnt_id: usize, body: &mut Rect) {
        let sz = self.style.scrollbar_size;
        let mut cs: Vec2 = self.containers[cnt_id].content_size;
        cs.x += self.style.padding * 2;
        cs.y += self.style.padding * 2;
        self.push_clip_rect(*body);
        if cs.y > self.containers[cnt_id].body.h {
            body.w -= sz;
        }
        if cs.x > self.containers[cnt_id].body.w {
            body.h -= sz;
        }
        let body = *body;
        let maxscroll = cs.y - body.h;
        if maxscroll > 0 && body.h > 0 {
            let id: Id = self.get_id_from_str("!scrollbary");
            let mut base = body;
            base.x = body.x + body.w;
            base.w = self.style.scrollbar_size;
            self.update_control(id, base, WidgetOption::empty());
            if self.focus == Some(id) && self.mouse_down.is_left() {
                self.containers[cnt_id].scroll.y += self.mouse_delta.y * cs.y / base.h;
            }
            self.containers[cnt_id].scroll.y = clamp(self.containers[cnt_id].scroll.y, 0, maxscroll);

            self.draw_frame(base, ControlColor::ScrollBase);
            let mut thumb = base;
            thumb.h = if self.style.thumb_size > base.h * body.h / cs.y {
                self.style.thumb_size
            } else {
                base.h * body.h / cs.y
            };
            thumb.y += self.containers[cnt_id].scroll.y * (base.h - thumb.h) / maxscroll;
            self.draw_frame(thumb, ControlColor::ScrollThumb);
            if self.mouse_over(body) {
                self.scroll_target = Some(cnt_id);
            }
        } else {
            self.containers[cnt_id].scroll.y = 0;
        }
        let maxscroll_0 = cs.x - body.w;
        if maxscroll_0 > 0 && body.w > 0 {
            let id_0: Id = self.get_id_from_str("!scrollbarx");
            let mut base_0 = body;
            base_0.y = body.y + body.h;
            base_0.h = self.style.scrollbar_size;
            self.update_control(id_0, base_0, WidgetOption::empty());
            if self.focus == Some(id_0) && self.mouse_down.is_left() {
                self.containers[cnt_id].scroll.x += self.mouse_delta.x * cs.x / base_0.w;
            }
            self.containers[cnt_id].scroll.x = clamp(self.containers[cnt_id].scroll.x, 0, maxscroll_0);

            self.draw_frame(base_0, ControlColor::ScrollBase);
            let mut thumb_0 = base_0;
            thumb_0.w = if self.style.thumb_size > base_0.w * body.w / cs.x {
                self.style.thumb_size
            } else {
                base_0.w * body.w / cs.x
            };
            thumb_0.x += self.containers[cnt_id].scroll.x * (base_0.w - thumb_0.w) / maxscroll_0;
            self.draw_frame(thumb_0, ControlColor::ScrollThumb);
            if self.mouse_over(body) {
                self.scroll_target = Some(cnt_id);
            }
        } else {
            self.containers[cnt_id].scroll.x = 0;
        }
        self.pop_clip_rect();
    }

    fn push_container_body(&mut self, cnt_idx: usize, body: Rect, opt: WidgetOption) {
        let mut body = body;
        if !opt.has_no_scroll() {
            self.scrollbars(cnt_idx, &mut body);
        }
        self.push_layout(body.expand(-self.style.padding), self.containers[cnt_idx].scroll);
        self.containers[cnt_idx].body = body;
    }
}
