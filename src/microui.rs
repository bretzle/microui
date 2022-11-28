use core::fmt::{Write};
use core::str::FromStr;
use ::libc;
use crate::fixed_collections::*;

pub type _IO_wide_data = libc::c_int;
pub type _IO_codecvt = libc::c_int;
pub type _IO_marker = libc::c_int;

extern "C" {
    fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t, __compar: __compar_fn_t);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
}

pub type size_t = libc::c_ulong;

pub type __compar_fn_t = Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>;

pub struct Pool<const N : usize> {
    vec: [mu_PoolItem; N],
}

impl<const N: usize> Pool<N> {
    pub fn alloc(&mut self, id: mu_Id, frame: usize) -> usize {
        let mut res = None;
        let mut latest_update = frame;
        for i in 0..N {
            if self.vec[i].last_update < latest_update {
                latest_update = self.vec[i].last_update;
                res = Some(i);
            }
        }

        assert!(res.is_some());
        self.vec[res.unwrap()].id = id;
        self.update(res.unwrap(), frame);
        return res.unwrap();
    }

    pub fn get(&self, id: mu_Id) -> Option<usize> {
        for i in 0..N {
            if self.vec[i].id == id {
                return Some(i)
            }
        }
        None
    }

    pub fn update(&mut self, idx: usize, frame: usize) {
        self.vec[idx].last_update = frame;
    }

    pub fn reset(&mut self, idx: usize) {
        self.vec[idx] = mu_PoolItem::default();
    }
}

#[derive(PartialEq)]
#[repr(u32)]
pub enum Clip {
    None = 0,
    Part = 1,
    All = 2,
}

#[derive(PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum Command {
    None = 0,
    Jump = 1,
    Clip = 2,
    Rect = 3,
    Text = 4,
    Icon = 5,
    Max = 6,
}

#[derive(PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum ControlColor {
    Max = 14,
    ScrollThumb = 13,
    ScrollBase = 12,
    BaseFocus = 11,
    BaseHover = 10,
    Base = 9,
    ButtonFocus = 8,
    ButtonHover = 7,
    Button = 6,
    PanelBG = 5,
    TitleText = 4,
    TitleBG = 3,
    WindowBG = 2,
    Border = 1,
    Text = 0,
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
#[repr(u32)]
pub enum Icon {
    Max = 5,
    Expanded = 4,
    Collapsed = 3,
    Check = 2,
    Close = 1,
    None = 0,
}

#[derive(PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum ResourceState {
    Change = 4,
    Submit = 2,
    Active = 1,
    None = 0,
}

impl ResourceState {
    pub fn is_changed(&self) -> bool {
        *self as u32 & ResourceState::Change as u32 != 0
    }
    pub fn is_submitted(&self) -> bool {
        *self as u32 & ResourceState::Submit as u32 != 0
    }
    pub fn is_active(&self) -> bool {
        *self as u32 & ResourceState::Active as u32 != 0
    }
    pub fn is_none(&self) -> bool {
        *self as u32 == 0
    }

    pub fn change(&mut self) {
        let u0 = *self as u32;
        let u1 = Self::Change as u32;
        *self = unsafe { core::mem::transmute(u0 | u1) }
    }

    pub fn submit(&mut self) {
        let u0 = *self as u32;
        let u1 = Self::Submit as u32;
        *self = unsafe { core::mem::transmute(u0 | u1) }
    }

    pub fn active(&mut self) {
        let u0 = *self as u32;
        let u1 = Self::Active as u32;
        *self = unsafe { core::mem::transmute(u0 | u1) }
    }
}

#[derive(PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum WidgetOption {
    Expanded = 4096,
    Closed = 2048,
    Popup = 1024,
    AutoSize = 512,
    HoldFocus = 256,
    NoTitle = 128,
    NoClose = 64,
    NoScroll = 32,
    NoResize = 16,
    NoFrame = 8,
    NoInteract = 4,
    AlignRight = 2,
    AlignCenter = 1,
    None = 0,
}

impl WidgetOption {
    pub fn is_expanded(&self) -> bool {
        *self as u32 & WidgetOption::Expanded as u32 != 0
    }
    pub fn is_closed(&self) -> bool {
        *self as u32 & WidgetOption::Closed as u32 != 0
    }
    pub fn is_popup(&self) -> bool {
        *self as u32 & WidgetOption::Popup as u32 != 0
    }
    pub fn is_auto_sizing(&self) -> bool {
        *self as u32 & WidgetOption::AutoSize as u32 != 0
    }
    pub fn is_holding_focus(&self) -> bool {
        *self as u32 & WidgetOption::HoldFocus as u32 != 0
    }
    pub fn has_no_title(&self) -> bool {
        *self as u32 & WidgetOption::NoTitle as u32 != 0
    }
    pub fn has_no_close(&self) -> bool {
        *self as u32 & WidgetOption::NoClose as u32 != 0
    }
    pub fn has_no_scroll(&self) -> bool {
        *self as u32 & WidgetOption::NoScroll as u32 != 0
    }
    pub fn is_fixed(&self) -> bool {
        *self as u32 & WidgetOption::NoResize as u32 != 0
    }
    pub fn has_no_frame(&self) -> bool {
        *self as u32 & WidgetOption::NoFrame as u32 != 0
    }
    pub fn is_not_interactive(&self) -> bool {
        *self as u32 & WidgetOption::NoInteract as u32 != 0
    }
    pub fn is_aligned_right(&self) -> bool {
        *self as u32 & WidgetOption::AlignRight as u32 != 0
    }
    pub fn is_aligned_center(&self) -> bool {
        *self as u32 & WidgetOption::AlignCenter as u32 != 0
    }
    pub fn is_none(&self) -> bool {
        *self as u32 == 0
    }

    pub fn with_expanded(self) -> Self {
        let u0 = self as u32;
        let u1 = Self::Expanded as u32;
        unsafe { core::mem::transmute(u0 | u1) }
    }

    pub fn with_closed(self) -> Self {
        let u0 = self as u32;
        let u1 = Self::Closed as u32;
        unsafe { core::mem::transmute(u0 | u1) }
    }

    pub fn with_popup(self) -> Self {
        let u0 = self as u32;
        let u1 = Self::Popup as u32;
        unsafe { core::mem::transmute(u0 | u1) }
    }

    pub fn with_auto_size(self) -> Self {
        let u0 = self as u32;
        let u1 = Self::AutoSize as u32;
        unsafe { core::mem::transmute(u0 | u1) }
    }

    pub fn with_hold_focus(self) -> Self {
        let u0 = self as u32;
        let u1 = Self::HoldFocus as u32;
        unsafe { core::mem::transmute(u0 | u1) }
    }

    pub fn with_no_title(self) -> Self {
        let u0 = self as u32;
        let u1 = Self::NoTitle as u32;
        unsafe { core::mem::transmute(u0 | u1) }
    }

    pub fn with_no_close(self) -> Self {
        let u0 = self as u32;
        let u1 = Self::NoClose as u32;
        unsafe { core::mem::transmute(u0 | u1) }
    }

    pub fn with_no_scroll(self) -> Self {
        let u0 = self as u32;
        let u1 = Self::NoScroll as u32;
        unsafe { core::mem::transmute(u0 | u1) }
    }

    pub fn with_no_resize(self) -> Self {
        let u0 = self as u32;
        let u1 = Self::NoResize as u32;
        unsafe { core::mem::transmute(u0 | u1) }
    }

    pub fn with_no_frame(self) -> Self {
        let u0 = self as u32;
        let u1 = Self::NoFrame as u32;
        unsafe { core::mem::transmute(u0 | u1) }
    }

    pub fn with_no_interaction(self) -> Self {
        let u0 = self as u32;
        let u1 = Self::NoInteract as u32;
        unsafe { core::mem::transmute(u0 | u1) }
    }

    pub fn with_align_center(self) -> Self {
        let u0 = self as u32;
        let u1 = Self::AlignCenter as u32;
        unsafe { core::mem::transmute(u0 | u1) }
    }

    pub fn with_align_right(self) -> Self {
        let u0 = self as u32;
        let u1 = Self::AlignRight as u32;
        unsafe { core::mem::transmute(u0 | u1) }
    }
}

#[derive(PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum MouseButton {
    Middle = 4,
    Right = 2,
    Left = 1,
    None = 0,
}

impl MouseButton {
    pub fn is_middle(&self) -> bool {
        *self as u32 & Self::Middle as u32 != 0
    }
    pub fn is_right(&self) -> bool {
        *self as u32 & Self::Right as u32 != 0
    }
    pub fn is_left(&self) -> bool {
        *self as u32 & Self::Left as u32 != 0
    }
    pub fn is_none(&self) -> bool {
        *self as u32 == 0
    }

    pub fn with_middle(self) -> Self {
        let u0 = self as u32;
        let u1 = Self::Middle as u32;
        unsafe { core::mem::transmute(u0 | u1) }
    }

    pub fn with_right(self) -> Self {
        let u0 = self as u32;
        let u1 = Self::Right as u32;
        unsafe { core::mem::transmute(u0 | u1) }
    }

    pub fn with_left(self) -> Self {
        let u0 = self as u32;
        let u1 = Self::Left as u32;
        unsafe { core::mem::transmute(u0 | u1) }
    }

    pub fn with(self, btn: Self) -> Self {
        let u0 = self as u32;
        let u1 = btn as u32;
        unsafe { core::mem::transmute(u0 | u1) }
    }

    pub fn set(&mut self, btn: Self) {
        let u0 = *self as u32;
        let u1 = btn as u32;
        *self = unsafe { core::mem::transmute(u0 | u1) }
    }

    pub fn clear(&mut self, btn: Self) {
        let u0 = *self as u32;
        let u1 = !(btn as u32);
        *self = unsafe { core::mem::transmute(u0 & u1) }
    }
}

pub const MU_KEY_RETURN: u32 = 16;
pub const MU_KEY_BACKSPACE: u32 = 8;
pub const MU_KEY_ALT: u32 = 4;
pub const MU_KEY_CTRL: u32 = 2;
pub const MU_KEY_SHIFT: u32 = 1;

#[repr(C)]
pub struct mu_Context {
    pub char_width: Option<extern "C" fn(mu_Font, char) -> usize>,
    pub char_height: Option<extern "C" fn(mu_Font, char) -> usize>,
    pub draw_frame: Option<extern "C" fn(&mut mu_Context, mu_Rect, ControlColor) -> ()>,
    pub style: mu_Style,
    pub hover: mu_Id,
    pub focus: mu_Id,
    pub last_id: mu_Id,
    pub last_rect: mu_Rect,
    pub last_zindex: libc::c_int,
    pub updated_focus: libc::c_int,
    pub frame: usize,
    pub hover_root: Option<usize>,
    pub next_hover_root: Option<usize>,
    pub scroll_target: Option<usize>,
    pub number_edit_buf: FixedString<127>,
    pub number_edit: mu_Id,
    pub command_list: FixedVec<mu_Command, 4096>,
    pub root_list: FixedVec<usize, 32>,
    pub container_stack: FixedVec<usize, 32>,
    pub clip_stack: FixedVec<mu_Rect, 32>,
    pub id_stack: FixedVec<mu_Id, 32>,
    pub layout_stack: FixedVec<mu_Layout, 16>,
    pub text_stack: FixedString<65536>,
    pub container_pool: Pool<48>,
    pub containers: [mu_Container; 48],
    pub treenode_pool: Pool<48>,
    pub mouse_pos: mu_Vec2,
    pub last_mouse_pos: mu_Vec2,
    pub mouse_delta: mu_Vec2,
    pub scroll_delta: mu_Vec2,
    pub mouse_down: MouseButton,
    pub mouse_pressed: MouseButton,
    pub key_down: libc::c_int,
    pub key_pressed: libc::c_int,
    pub input_text: FixedString<32>,
}

#[derive(Default, Copy, Clone)]
#[repr(C)]
pub struct mu_Vec2 {
    pub x: i32,
    pub y: i32,
}

#[derive(Default, Copy, Clone)]
#[repr(C)]
pub struct mu_PoolItem {
    pub id: mu_Id,
    pub last_update: usize,
}

pub type mu_Id = u32;

#[derive(Default, Copy, Clone)]
#[repr(C)]
pub struct mu_Container {
    pub head_idx: Option<usize>,
    pub tail_idx: Option<usize>,
    pub rect: mu_Rect,
    pub body: mu_Rect,
    pub content_size: mu_Vec2,
    pub scroll: mu_Vec2,
    pub zindex: libc::c_int,
    pub open: libc::c_int,
}

#[derive(Default, Copy, Clone)]
#[repr(C)]
pub struct mu_Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

#[derive(Default, Copy, Clone)]
#[repr(C)]
pub struct mu_Layout {
    pub body: mu_Rect,
    pub next: mu_Rect,
    pub position: mu_Vec2,
    pub size: mu_Vec2,
    pub max: mu_Vec2,
    pub widths: [libc::c_int; 16],
    pub items: libc::c_int,
    pub item_index: libc::c_int,
    pub next_row: libc::c_int,
    pub next_type: libc::c_int,
    pub indent: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union mu_Command {
    pub type_0: Command,
    pub base: mu_BaseCommand,
    pub jump: mu_JumpCommand,
    pub clip: mu_ClipCommand,
    pub rect: mu_RectCommand,
    pub text: mu_TextCommand,
    pub icon: mu_IconCommand,
}

impl Default for mu_Command {
    fn default() -> Self {
        Self { type_0: Command::None }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct mu_IconCommand {
    pub base: mu_BaseCommand,
    pub rect: mu_Rect,
    pub id: Icon,
    pub color: mu_Color,
}

#[derive(Default, Copy, Clone)]
#[repr(C)]
pub struct mu_Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct mu_BaseCommand {
    pub type_0: Command,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct mu_TextCommand {
    pub base: mu_BaseCommand,
    pub font: mu_Font,
    pub pos: mu_Vec2,
    pub color: mu_Color,
    pub str_start: usize,
    pub str_len: usize,
}

pub type mu_Font = *mut libc::c_void;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct mu_RectCommand {
    pub base: mu_BaseCommand,
    pub rect: mu_Rect,
    pub color: mu_Color,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct mu_ClipCommand {
    pub base: mu_BaseCommand,
    pub rect: mu_Rect,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct mu_JumpCommand {
    pub base: mu_BaseCommand,
    pub dst_idx: Option<usize>,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct mu_Style {
    pub font: mu_Font,
    pub size: mu_Vec2,
    pub padding: libc::c_int,
    pub spacing: libc::c_int,
    pub indent: libc::c_int,
    pub title_height: libc::c_int,
    pub scrollbar_size: libc::c_int,
    pub thumb_size: libc::c_int,
    pub colors: [mu_Color; 14],
}

pub type mu_Real = f32;

pub const ABSOLUTE: C2RustUnnamed_14 = 2;
pub const RELATIVE: C2RustUnnamed_14 = 1;

pub type C2RustUnnamed_14 = libc::c_uint;

static mut unclipped_rect: mu_Rect = mu_Rect { x: 0, y: 0, w: 0x1000000, h: 0x1000000 };

static mut default_style: mu_Style = mu_Style {
    font: 0 as *const libc::c_void as *mut libc::c_void,
    size: mu_Vec2 {
        x: 68 as libc::c_int,
        y: 10 as libc::c_int,
    },
    padding: 5 as libc::c_int,
    spacing: 4 as libc::c_int,
    indent: 24 as libc::c_int,
    title_height: 24 as libc::c_int,
    scrollbar_size: 12 as libc::c_int,
    thumb_size: 8 as libc::c_int,
    colors: [
        mu_Color { r: 230, g: 230, b: 230, a: 255 },
        mu_Color { r: 25, g: 25, b: 25, a: 255 },
        mu_Color { r: 50, g: 50, b: 50, a: 255 },
        mu_Color { r: 25, g: 25, b: 25, a: 255 },
        mu_Color { r: 240, g: 240, b: 240, a: 255 },
        mu_Color { r: 0, g: 0, b: 0, a: 0 },
        mu_Color { r: 75, g: 75, b: 75, a: 255 },
        mu_Color { r: 95, g: 95, b: 95, a: 255 },
        mu_Color { r: 115, g: 115, b: 115, a: 255 },
        mu_Color { r: 30, g: 30, b: 30, a: 255 },
        mu_Color { r: 35, g: 35, b: 35, a: 255 },
        mu_Color { r: 40, g: 40, b: 40, a: 255 },
        mu_Color { r: 43, g: 43, b: 43, a: 255 },
        mu_Color { r: 30, g: 30, b: 30, a: 255 },
    ],
};

pub fn mu_vec2(x: i32, y: i32) -> mu_Vec2 {
    mu_Vec2 { x, y }
}

pub fn mu_rect(x: i32, y: i32, w: i32, h: i32) -> mu_Rect {
    mu_Rect { x, y, w, h }
}

pub fn mu_color(r: u8, g: u8, b: u8, a: u8) -> mu_Color {
    mu_Color { r, g, b, a }
}

pub fn expand_rect(rect: mu_Rect, n: i32) -> mu_Rect {
    mu_rect(rect.x - n, rect.y - n, rect.w + n * 2 as libc::c_int, rect.h + n * 2 as libc::c_int)
}

pub fn intersect_rects(r1: mu_Rect, r2: mu_Rect) -> mu_Rect {
    let x1 = if r1.x > r2.x { r1.x } else { r2.x };
    let y1 = if r1.y > r2.y { r1.y } else { r2.y };
    let mut x2 = if r1.x + r1.w < r2.x + r2.w { r1.x + r1.w } else { r2.x + r2.w };
    let mut y2 = if r1.y + r1.h < r2.y + r2.h { r1.y + r1.h } else { r2.y + r2.h };
    if x2 < x1 {
        x2 = x1;
    }
    if y2 < y1 {
        y2 = y1;
    }
    return mu_rect(x1, y1, x2 - x1, y2 - y1);
}

pub fn rect_overlaps_vec2(r: mu_Rect, p: mu_Vec2) -> bool {
    p.x >= r.x && p.x < r.x + r.w && p.y >= r.y && p.y < r.y + r.h
}

#[no_mangle]
pub unsafe extern "C" fn mu_init(mut ctx: *mut mu_Context) {
    memset(ctx as *mut libc::c_void, 0 as libc::c_int, core::mem::size_of::<mu_Context>() as libc::c_ulong);
    (*ctx).draw_frame = Some(draw_frame as extern "C" fn(&mut mu_Context, mu_Rect, ControlColor) -> ());
    (*ctx).style = default_style.clone();
}

pub extern "C" fn draw_frame(ctx: &mut mu_Context, rect: mu_Rect, colorid: ControlColor) {
    ctx.mu_draw_rect(rect, ctx.style.colors[colorid as usize]);
    if colorid == ControlColor::ScrollBase || colorid == ControlColor::ScrollThumb || colorid == ControlColor::TitleBG {
        return;
    }
    if ctx.style.colors[ControlColor::Border as usize].a != 0 {
        ctx.mu_draw_box(expand_rect(rect, 1 as libc::c_int), ctx.style.colors[ControlColor::Border as usize]);
    }
}

unsafe extern "C" fn compare_zindex(a: *const libc::c_void, b: *const libc::c_void) -> libc::c_int {
    return (**(a as *mut *mut mu_Container)).zindex - (**(b as *mut *mut mu_Container)).zindex;
}

fn hash_u32(hash_0: &mut mu_Id, orig_id: u32) {
    let bytes = orig_id.to_be_bytes();
    for b in bytes {
        let fresh1 = b;
        *hash_0 = (*hash_0 ^ fresh1 as libc::c_uint).wrapping_mul(16777619 as libc::c_int as libc::c_uint);
    }
}

fn hash_str(hash_0: &mut mu_Id, s: &str) {
    for c in s.chars() {
        let fresh1 = c as i32;
        *hash_0 = (*hash_0 ^ fresh1 as libc::c_uint).wrapping_mul(16777619 as libc::c_int as libc::c_uint);
    }
}

fn hash_bytes(hash_0: &mut mu_Id, s: &[u8]) {
    for c in s {
        let fresh1 = *c as i32;
        *hash_0 = (*hash_0 ^ fresh1 as libc::c_uint).wrapping_mul(16777619 as libc::c_int as libc::c_uint);
    }
}

impl mu_Context {
    pub unsafe extern "C" fn mu_begin(&mut self) {
        assert!((self.char_width).is_some() && (self.char_height).is_some());
        self.root_list.clear();
        self.text_stack.clear();
        self.scroll_target = None;
        self.hover_root = self.next_hover_root;
        self.next_hover_root = None;
        self.mouse_delta.x = self.mouse_pos.x - self.last_mouse_pos.x;
        self.mouse_delta.y = self.mouse_pos.y - self.last_mouse_pos.y;
        self.command_list.clear();
        self.frame += 1;
    }

    #[no_mangle]
    pub unsafe extern "C" fn mu_end(&mut self) {
        assert_eq!(self.container_stack.len(), 0);
        assert_eq!(self.clip_stack.len(), 0);
        assert_eq!(self.id_stack.len(), 0);
        assert_eq!(self.layout_stack.len(), 0);
        if !self.scroll_target.is_none() {
            self.containers[self.scroll_target.unwrap()].scroll.x += self.scroll_delta.x;
            self.containers[self.scroll_target.unwrap()].scroll.y += self.scroll_delta.y;
        }
        if self.updated_focus == 0 {
            self.focus = 0 as libc::c_int as mu_Id;
        }
        self.updated_focus = 0 as libc::c_int;
        if !self.mouse_pressed.is_none()
            && !self.next_hover_root.is_none()
            && self.containers[self.next_hover_root.unwrap()].zindex < self.last_zindex
            && self.containers[self.next_hover_root.unwrap()].zindex >= 0 as libc::c_int
        {
            self.mu_bring_to_front(self.next_hover_root.unwrap());
        }
        self.key_pressed = 0 as libc::c_int;
        self.input_text.clear();
        self.mouse_pressed = MouseButton::None;
        self.scroll_delta = mu_vec2(0 as libc::c_int, 0 as libc::c_int);
        self.last_mouse_pos = self.mouse_pos;
        let n = self.root_list.len();
        quick_sort_by(self.root_list.as_slice_mut(), |a, b| {self.containers[*a].zindex.cmp(&self.containers[*b].zindex)});

        for i in 0..n {
            let cnt: *mut mu_Container = &mut self.containers[self.root_list[i as usize]] as *mut mu_Container;
            if i == 0 {
                // root container!
                // if this is the first container then make the first command jump to it.
                // otherwise set the previous container's tail to jump to this one

                let mut cmd: &mut mu_Command = &mut self.command_list[0];
                assert!(cmd.type_0 == Command::Jump);
                cmd.jump.dst_idx = Some((*cnt).head_idx.unwrap() + 1);
                assert!(cmd.jump.dst_idx.unwrap() < self.command_list.len());
            } else {
                let prev: *mut mu_Container = &mut self.containers[self.root_list[i - 1]] as *mut mu_Container;
                self.command_list[(*prev).tail_idx.unwrap()].jump.dst_idx = Some((*cnt).head_idx.unwrap() + 1);
            }
            if i == n - 1 {
                assert!((*cnt).tail_idx.unwrap() < self.command_list.len());
                assert!(self.command_list[(*cnt).tail_idx.unwrap()].type_0 == Command::Jump);
                self.command_list[(*cnt).tail_idx.unwrap()].jump.dst_idx = Some(self.command_list.len());
                // the snake eats its tail
            }
        }
    }

    pub fn mu_set_focus(&mut self, id: mu_Id) {
        self.focus = id;
        self.updated_focus = 1 as libc::c_int;
    }

    pub fn mu_get_id_u32(&mut self, orig_id: u32) -> mu_Id {
        let mut res: mu_Id = match self.id_stack.top() {
            Some(id) => *id,
            None => 2166136261 as mu_Id,
        };
        hash_u32(&mut res, orig_id);
        self.last_id = res;
        return res;
    }

    pub fn mu_get_id_from_ptr<T: ?Sized>(&mut self, orig_id: &T) -> mu_Id {
        let mut res: mu_Id = match self.id_stack.top() {
            Some(id) => *id,
            None => 2166136261 as mu_Id,
        };
        let ptr = orig_id as *const T as *const u8 as usize;
        let bytes = ptr.to_le_bytes();
        hash_bytes(&mut res, &bytes);
        self.last_id = res;
        return res;
    }

    pub fn mu_get_id_from_str(&mut self, s: &str) -> mu_Id {
        let mut res: mu_Id = match self.id_stack.top() {
            Some(id) => *id,
            None => 2166136261 as mu_Id,
        };
        hash_str(&mut res, s);
        self.last_id = res;
        return res;
    }

    pub fn mu_push_id_from_ptr<T>(&mut self, orig_id: &T) {
        let id = self.mu_get_id_from_ptr(orig_id);
        self.id_stack.push(id);
    }

    pub fn mu_push_id_from_str(&mut self, s: &str) {
        let id = self.mu_get_id_from_str(s);
        self.id_stack.push(id);
    }

    pub fn mu_pop_id(&mut self) {
        self.id_stack.pop();
    }

    pub fn mu_push_clip_rect(&mut self, rect: mu_Rect) {
        let last = self.mu_get_clip_rect();
        self.clip_stack.push(intersect_rects(rect, last));
    }

    pub fn mu_pop_clip_rect(&mut self) {
        self.clip_stack.pop();
    }

    pub fn mu_get_clip_rect(&mut self) -> mu_Rect {
        *self.clip_stack.top().unwrap()
    }

    pub fn mu_check_clip(&mut self, r: mu_Rect) -> Clip {
        let cr = self.mu_get_clip_rect();
        if r.x > cr.x + cr.w || r.x + r.w < cr.x || r.y > cr.y + cr.h || r.y + r.h < cr.y {
            return Clip::All;
        }
        if r.x >= cr.x && r.x + r.w <= cr.x + cr.w && r.y >= cr.y && r.y + r.h <= cr.y + cr.h {
            return Clip::None;
        }
        return Clip::Part;
    }

    fn push_layout(&mut self, body: mu_Rect, scroll: mu_Vec2) {
        let mut layout: mu_Layout = mu_Layout {
            body: mu_Rect { x: 0, y: 0, w: 0, h: 0 },
            next: mu_Rect { x: 0, y: 0, w: 0, h: 0 },
            position: mu_Vec2 { x: 0, y: 0 },
            size: mu_Vec2 { x: 0, y: 0 },
            max: mu_Vec2 { x: 0, y: 0 },
            widths: [0; 16],
            items: 0,
            item_index: 0,
            next_row: 0,
            next_type: 0,
            indent: 0,
        };
        let mut width = 0;

        layout.body = mu_rect(body.x - scroll.x, body.y - scroll.y, body.w, body.h);
        layout.max = mu_vec2(-(0x1000000 as libc::c_int), -(0x1000000 as libc::c_int));
        self.layout_stack.push(layout);
        self.mu_layout_row(1 as libc::c_int, &mut width, 0);
    }

    fn get_layout(&mut self) -> &mu_Layout {
        return self.layout_stack.top().unwrap();
    }

    fn get_layout_mut(&mut self) -> &mut mu_Layout {
        return self.layout_stack.top_mut().unwrap();
    }

    unsafe extern "C" fn pop_container(&mut self) {
        let mut cnt: *mut mu_Container = self.mu_get_current_container();
        let layout = self.get_layout();
        (*cnt).content_size.x = layout.max.x - layout.body.x;
        (*cnt).content_size.y = layout.max.y - layout.body.y;

        self.container_stack.pop();
        self.layout_stack.pop();
        self.mu_pop_id();
    }

    #[no_mangle]
    pub unsafe extern "C" fn mu_get_current_container(&mut self) -> *mut mu_Container {
        &mut self.containers[*self.container_stack.top().unwrap()] as *mut mu_Container
    }

    unsafe extern "C" fn get_container(&mut self, id: mu_Id, opt: WidgetOption) -> *mut mu_Container {
        let mut cnt: *mut mu_Container = 0 as *mut mu_Container;
        let idx = self.container_pool.get(id);
        if idx.is_some() {
            if self.containers[idx.unwrap()].open != 0 || !opt.is_closed() {
                self.container_pool.update(idx.unwrap(), self.frame);
            }
            return &mut *(self.containers).as_mut_ptr().offset(idx.unwrap() as isize) as *mut mu_Container;
        }
        if opt.is_closed() {
            return 0 as *mut mu_Container;
        }
        let idx = self.container_pool.alloc(id, self.frame);
        cnt = &mut *(self.containers).as_mut_ptr().offset(idx as isize) as *mut mu_Container;
        memset(
            cnt as *mut libc::c_void,
            0 as libc::c_int,
            core::mem::size_of::<mu_Container>() as libc::c_ulong,
        );
        (*cnt).head_idx = None;
        (*cnt).tail_idx = None;
        (*cnt).open = 1 as libc::c_int;
        self.mu_bring_to_front(idx);
        return cnt;
    }

    fn get_container_index(&mut self, id: mu_Id, opt: WidgetOption) -> Option<usize> {
        let idx = self.container_pool.get(id);
        if idx.is_some() {
            if self.containers[idx.unwrap()].open != 0 || !opt.is_closed() {
                self.container_pool.update(idx.unwrap(), self.frame);
            }
            return idx;
        }
        if opt.is_closed() {
            return None;
        }
        let idx = self.container_pool.alloc(id, self.frame);
        self.containers[idx] = mu_Container::default();
        self.containers[idx].head_idx = None;
        self.containers[idx].tail_idx = None;
        self.containers[idx].open = 1 as libc::c_int;
        self.mu_bring_to_front(idx);
        Some(idx)
    }

    pub unsafe extern "C" fn mu_get_container(&mut self, name: &str) -> *mut mu_Container {
        let id = self.mu_get_id_from_str(name);
        self.get_container(id, WidgetOption::None)
    }

    fn mu_get_container_index(&mut self, name: &str) -> Option<usize> {
        let id = self.mu_get_id_from_str(name);
       self.get_container_index(id, WidgetOption::None)
    }

    pub fn mu_bring_to_front(&mut self, cnt: usize) {
        self.last_zindex += 1;
        self.containers[cnt].zindex = self.last_zindex;
    }

    #[no_mangle]
    pub unsafe extern "C" fn mu_input_mousemove(&mut self, x: i32, y: i32) {
        self.mouse_pos = mu_vec2(x, y);
    }

    #[no_mangle]
    pub unsafe extern "C" fn mu_input_mousedown(&mut self, x: i32, y: i32, btn: MouseButton) {
        self.mu_input_mousemove(x, y);
        self.mouse_down.set(btn);
        self.mouse_pressed.set(btn);
    }

    #[no_mangle]
    pub unsafe extern "C" fn mu_input_mouseup(&mut self, x: i32, y: i32, btn: MouseButton) {
        self.mu_input_mousemove(x, y);
        self.mouse_down.clear(btn);
    }

    pub fn mu_input_scroll(&mut self, x: i32, y: i32) {
        self.scroll_delta.x += x;
        self.scroll_delta.y += y;
    }

    pub fn mu_input_keydown(&mut self, key: libc::c_int) {
        self.key_pressed |= key;
        self.key_down |= key;
    }

    pub fn mu_input_keyup(&mut self, key: libc::c_int) {
        self.key_down &= !key;
    }

    pub fn mu_input_text(&mut self, text: &str) {
        self.input_text += text;
    }

    pub fn mu_push_command(&mut self, type_0: Command) -> (&mut mu_Command, usize) {
        let (cmd, pos) = self.command_list.push(mu_Command::default());
        cmd.base.type_0 = type_0;
        (cmd, pos)
    }

    pub fn mu_push_text(&mut self, str: &str) -> usize {
        let str_start = self.text_stack.len();
        for c in str.chars() {
            self.text_stack.push(c);
        }
        return str_start;
    }

    ///
    /// returns the next command to execute and the next index to use
    ///
    pub fn mu_next_command(&mut self, mut cmd_id: usize) -> Option<(mu_Command, usize)> {
        unsafe {
            if cmd_id >= self.command_list.len() {
                cmd_id = 0
            }

            while cmd_id != self.command_list.len() {
                if self.command_list[cmd_id].type_0 != Command::Jump {
                    return Some((self.command_list[cmd_id], cmd_id + 1));
                }
                cmd_id = self.command_list[cmd_id].jump.dst_idx.unwrap();
            }
            None
        }
    }

    unsafe fn push_jump(&mut self, dst_idx: Option<usize>) -> usize {
        let (cmd, pos) = self.mu_push_command(Command::Jump);
        cmd.jump.dst_idx = dst_idx;
        pos
    }

    pub fn mu_set_clip(&mut self, rect: mu_Rect) {
        let (cmd, _) = self.mu_push_command(Command::Clip);
        cmd.clip.rect = rect;
    }

    pub fn mu_draw_rect(&mut self, mut rect: mu_Rect, color: mu_Color) {
        rect = intersect_rects(rect, self.mu_get_clip_rect());
        if rect.w > 0 as libc::c_int && rect.h > 0 as libc::c_int {
            let (cmd, _) = self.mu_push_command(Command::Rect);
            cmd.rect.rect = rect;
            cmd.rect.color = color;
        }
    }

    pub fn mu_draw_box(&mut self, rect: mu_Rect, color: mu_Color) {
        self.mu_draw_rect(mu_rect(rect.x + 1 as libc::c_int, rect.y, rect.w - 2 as libc::c_int, 1 as libc::c_int), color);
        self.mu_draw_rect(
            mu_rect(
                rect.x + 1 as libc::c_int,
                rect.y + rect.h - 1 as libc::c_int,
                rect.w - 2 as libc::c_int,
                1 as libc::c_int,
            ),
            color,
        );
        self.mu_draw_rect(mu_rect(rect.x, rect.y, 1 as libc::c_int, rect.h), color);
        self.mu_draw_rect(mu_rect(rect.x + rect.w - 1 as libc::c_int, rect.y, 1 as libc::c_int, rect.h), color);
    }

    #[no_mangle]
    pub unsafe extern "C" fn mu_draw_text(&mut self, font: mu_Font, str: &str, pos: mu_Vec2, color: mu_Color) {
        let rect: mu_Rect = mu_rect(pos.x, pos.y, self.get_text_width(font, str), self.get_text_height(font, str));
        let clipped = self.mu_check_clip(rect);
        match clipped {
            Clip::All => return,
            Clip::Part => {
                let clip = self.mu_get_clip_rect();
                self.mu_set_clip(clip)
            }
            _ => (),
        }

        let str_start = self.mu_push_text(str);
        let (cmd, _) = self.mu_push_command(Command::Text);
        cmd.text.str_start = str_start;
        cmd.text.str_len = str.len();
        cmd.text.pos = pos;
        cmd.text.color = color;
        cmd.text.font = font;
        if clipped != Clip::None {
            self.mu_set_clip(unclipped_rect);
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn mu_draw_icon(&mut self, id: Icon, rect: mu_Rect, color: mu_Color) {
        let clipped = self.mu_check_clip(rect);
        match clipped {
            Clip::All => return,
            Clip::Part => {
                let clip = self.mu_get_clip_rect();
                self.mu_set_clip(clip)
            }
            _ => (),
        }
        let (cmd, _) = self.mu_push_command(Command::Icon);
        cmd.icon.id = id;
        cmd.icon.rect = rect;
        cmd.icon.color = color;
        if clipped != Clip::None {
            self.mu_set_clip(unclipped_rect);
        }
    }

    pub fn mu_layout_begin_column(&mut self) {
        let layout = self.mu_layout_next();
        self.push_layout(layout, mu_vec2(0 as libc::c_int, 0 as libc::c_int));
    }

    pub fn mu_layout_end_column(&mut self) {
        let b = self.get_layout().clone();
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
        a.max.x = i32::max(a.max.x, b.max.x);
        a.max.y = i32::max(a.max.y, b.max.y);
    }

    pub unsafe fn mu_layout_row_for_layout(layout: &mut mu_Layout, items: libc::c_int, widths: *const libc::c_int, height: libc::c_int) {
        if !widths.is_null() {
            assert!(items <= 16 as libc::c_int);
            memcpy(
                (layout.widths).as_mut_ptr() as *mut libc::c_void,
                widths as *const libc::c_void,
                (items as libc::c_ulong).wrapping_mul(core::mem::size_of::<libc::c_int>() as libc::c_ulong),
            );
        }
        layout.items = items;
        layout.position = mu_vec2(layout.indent, layout.next_row);
        layout.size.y = height;
        layout.item_index = 0 as libc::c_int;
    }

    pub fn mu_layout_row(&mut self, items: libc::c_int, widths: *const libc::c_int, height: libc::c_int) {
        let layout = self.get_layout_mut();
        unsafe { Self::mu_layout_row_for_layout(layout, items, widths, height) };
    }

    pub fn mu_layout_width(&mut self, width: i32) {
        self.get_layout_mut().size.x = width;
    }

    pub fn mu_layout_height(&mut self, height: i32) {
        self.get_layout_mut().size.y = height;
    }

    pub fn mu_layout_set_next(&mut self, r: mu_Rect, relative: libc::c_int) {
        let layout = self.get_layout_mut();
        layout.next = r;
        layout.next_type = if relative != 0 { RELATIVE as libc::c_int } else { ABSOLUTE as libc::c_int };
    }

    pub fn mu_layout_next(&mut self) -> mu_Rect {
        let style = self.style;
        let layout = self.get_layout_mut();
        let mut res: mu_Rect = mu_Rect { x: 0, y: 0, w: 0, h: 0 };
        if layout.next_type != 0 {
            let type_0: libc::c_int = layout.next_type;
            layout.next_type = 0 as libc::c_int;
            res = layout.next;
            if type_0 == ABSOLUTE as libc::c_int {
                self.last_rect = res;
                return self.last_rect;
            }
        } else {
            let litems = layout.items;
            let lsize_y = layout.size.y;
            if layout.item_index == layout.items {
                unsafe { Self::mu_layout_row_for_layout(layout, litems, 0 as *const libc::c_int, lsize_y) };
            }
            res.x = layout.position.x;
            res.y = layout.position.y;
            res.w = if layout.items > 0 as libc::c_int {
                layout.widths[layout.item_index as usize]
            } else {
                layout.size.x
            };
            res.h = layout.size.y;
            if res.w == 0 as libc::c_int {
                res.w = style.size.x + style.padding * 2 as libc::c_int;
            }
            if res.h == 0 as libc::c_int {
                res.h = style.size.y + style.padding * 2 as libc::c_int;
            }
            if res.w < 0 as libc::c_int {
                res.w += layout.body.w - res.x + 1 as libc::c_int;
            }
            if res.h < 0 as libc::c_int {
                res.h += layout.body.h - res.y + 1 as libc::c_int;
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
        return self.last_rect;
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
            },
            None => false,
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn mu_draw_control_frame(&mut self, id: mu_Id, rect: mu_Rect, mut colorid: ControlColor, opt: WidgetOption) {
        if opt.has_no_frame() {
            return;
        }

        if self.focus == id {
            colorid.focus()
        } else if self.hover == id {
            colorid.hover()
        }
        (self.draw_frame).expect("non-null function pointer")(self, rect, colorid);
    }

    #[no_mangle]
    pub unsafe extern "C" fn mu_draw_control_text(&mut self, str: &str, rect: mu_Rect, colorid: ControlColor, opt: WidgetOption) {
        let mut pos: mu_Vec2 = mu_Vec2 { x: 0, y: 0 };
        let font: mu_Font = self.style.font;
        let tw = self.get_text_width(font, str);
        self.mu_push_clip_rect(rect);
        pos.y = rect.y + (rect.h - self.get_text_height(font, str)) / 2 as libc::c_int;
        if opt.is_aligned_center() {
            pos.x = rect.x + (rect.w - tw) / 2 as libc::c_int;
        } else if opt.is_aligned_right() {
            pos.x = rect.x + rect.w - tw - self.style.padding;
        } else {
            pos.x = rect.x + self.style.padding;
        }
        self.mu_draw_text(font, str, pos, self.style.colors[colorid as usize]);
        self.mu_pop_clip_rect();
    }

    pub fn mu_mouse_over(&mut self, rect: mu_Rect) -> libc::c_int {
        return (rect_overlaps_vec2(rect, self.mouse_pos) && rect_overlaps_vec2(self.mu_get_clip_rect(), self.mouse_pos) && self.in_hover_root())
            as libc::c_int;
    }

    #[no_mangle]
    pub fn mu_update_control(&mut self, id: mu_Id, rect: mu_Rect, opt: WidgetOption) {
        let mouseover: libc::c_int = self.mu_mouse_over(rect);
        if self.focus == id {
            self.updated_focus = 1 as libc::c_int;
        }
        if opt.is_not_interactive() {
            return;
        }
        if mouseover != 0 && self.mouse_down.is_none() {
            self.hover = id;
        }
        if self.focus == id {
            if !self.mouse_pressed.is_none() && mouseover == 0 {
                self.mu_set_focus(0 as libc::c_int as mu_Id);
            }
            if self.mouse_down.is_none() && !opt.is_holding_focus() {
                self.mu_set_focus(0 as libc::c_int as mu_Id);
            }
        }
        if self.hover == id {
            if !self.mouse_pressed.is_none() {
                self.mu_set_focus(id);
            } else if mouseover == 0 {
                self.hover = 0 as libc::c_int as mu_Id;
            }
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn get_text_width(&self, font: mu_Font, text: &str) -> i32 {
        let mut res = 0;
        let mut acc = 0;
        for c in text.chars() {
            if c == '\n' {
                res = usize::max(res, acc);
                acc = 0;
            }
            acc += self.char_width.expect("non-null function pointer")(font, c);
        }
        res = usize::max(res, acc);
        res as i32
    }

    #[no_mangle]
    pub unsafe extern "C" fn get_text_height(&self, font: mu_Font, text: &str) -> i32 {
        let mut res = 0;
        let mut acc = 0;
        for c in text.chars() {
            if c == '\n' {
                res += acc;
                acc = 0;
            }
            acc = usize::max(acc, self.char_height.expect("non-null function pointer")(font, c));
        }
        res += acc;
        res as i32
    }

    #[no_mangle]
    pub unsafe extern "C" fn mu_text(&mut self, text: &str) {
        let mut width: libc::c_int = -(1 as libc::c_int);
        let font: mu_Font = self.style.font;
        let color: mu_Color = self.style.colors[ControlColor::Text as libc::c_int as usize];
        self.mu_layout_begin_column();
        let first_line = match text.lines().next() {
            Some(l) => l,
            None => "",
        };
        let h = self.get_text_height(font, first_line);
        self.mu_layout_row(1 as libc::c_int, &mut width, h);
        for line in text.lines() {
            let r = self.mu_layout_next();
            self.mu_draw_text(font, line, mu_vec2(r.x, r.y), color);
        }
        self.mu_layout_end_column();
    }

    #[no_mangle]
    pub unsafe extern "C" fn mu_label(&mut self, text: &str) {
        let layout = self.mu_layout_next();
        self.mu_draw_control_text(text, layout, ControlColor::Text, WidgetOption::None);
    }

    #[no_mangle]
    pub unsafe extern "C" fn mu_button_ex(&mut self, label: &str, icon: Icon, opt: WidgetOption) -> ResourceState {
        let mut res = ResourceState::None;
        let id: mu_Id = if label.len() > 0 {
            self.mu_get_id_from_str(label)
        } else {
            self.mu_get_id_u32(icon as u32)
        };
        let r: mu_Rect = self.mu_layout_next();
        self.mu_update_control(id, r, opt);
        if self.mouse_pressed.is_left() && self.focus == id {
            res.submit();
        }
        self.mu_draw_control_frame(id, r, ControlColor::Button, opt);
        if label.len() > 0 {
            self.mu_draw_control_text(label, r, ControlColor::Text, opt);
        }
        if icon != Icon::None {
            self.mu_draw_icon(icon, r, self.style.colors[ControlColor::Text as libc::c_int as usize]);
        }
        return res;
    }

    #[no_mangle]
    pub unsafe extern "C" fn mu_checkbox(&mut self, label: &str, state: &mut bool) -> ResourceState {
        let mut res = ResourceState::None;
        let id: mu_Id = self.mu_get_id_from_ptr(state);
        let mut r: mu_Rect = self.mu_layout_next();
        let box_0: mu_Rect = mu_rect(r.x, r.y, r.h, r.h);
        self.mu_update_control(id, r, WidgetOption::None);
        if self.mouse_pressed.is_left() && self.focus == id {
            res.change();
            *state = *state == false;
        }
        self.mu_draw_control_frame(id, box_0, ControlColor::Base, WidgetOption::None);
        if *state {
            self.mu_draw_icon(Icon::Check, box_0, self.style.colors[ControlColor::Text as libc::c_int as usize]);
        }
        r = mu_rect(r.x + box_0.w, r.y, r.w - box_0.w, r.h);
        self.mu_draw_control_text(label, r, ControlColor::Text, WidgetOption::None);
        return res;
    }

    #[no_mangle]
    pub unsafe extern "C" fn mu_textbox_raw(&mut self, buf: &mut dyn IString, id: mu_Id, r: mu_Rect, opt: WidgetOption) -> ResourceState {
        let mut res = ResourceState::None;
        self.mu_update_control(id, r, opt.with_hold_focus());
        if self.focus == id {
            let mut len = buf.len();

            if self.input_text.len() > 0 {
                buf.add_str(self.input_text.as_str());
                len += self.input_text.len() as usize;
                res.change()
            }

            if self.key_pressed & MU_KEY_BACKSPACE as i32 != 0 && len > 0 {
                // skip utf-8 continuation bytes
                buf.pop();
                res.change();
            }
            if self.key_pressed & MU_KEY_RETURN as libc::c_int != 0 {
                self.mu_set_focus(0 as libc::c_int as mu_Id);
                res.submit();
            }
        }
        self.mu_draw_control_frame(id, r, ControlColor::Base, opt);
        if self.focus == id {
            let color: mu_Color = self.style.colors[ControlColor::Text as libc::c_int as usize];
            let font: mu_Font = self.style.font;
            let textw = self.get_text_width(font, buf.as_str());
            let texth = self.get_text_height(font, buf.as_str());
            let ofx: libc::c_int = r.w - self.style.padding - textw - 1 as libc::c_int;
            let textx: libc::c_int = r.x + (if ofx < self.style.padding { ofx } else { self.style.padding });
            let texty: libc::c_int = r.y + (r.h - texth) / 2 as libc::c_int;
            self.mu_push_clip_rect(r);
            self.mu_draw_text(font, buf.as_str(), mu_vec2(textx, texty), color);
            self.mu_draw_rect(mu_rect(textx + textw, texty, 1 as libc::c_int, texth), color);
            self.mu_pop_clip_rect();
        } else {
            self.mu_draw_control_text(buf.as_str(), r, ControlColor::Text, opt);
        }
        return res;
    }

    unsafe extern "C" fn number_textbox(&mut self, value: &mut mu_Real, r: mu_Rect, id: mu_Id) -> ResourceState {
        if self.mouse_pressed.is_left() && self.key_down & MU_KEY_SHIFT as libc::c_int != 0 && self.hover == id {
            self.number_edit = id;
            write!(self.number_edit_buf, "{:.3}", value).unwrap();
        }
        if self.number_edit == id {
            let mut temp = self.number_edit_buf.clone();
            let res: ResourceState = self.mu_textbox_raw(&mut temp, id, r, WidgetOption::None);
            self.number_edit_buf = temp;
            if res.is_submitted() || self.focus != id {
                *value = f32::from_str(self.number_edit_buf.as_str()).unwrap();
                self.number_edit = 0 as libc::c_int as mu_Id;
            } else {
                return ResourceState::Active;
            }
        }
        return ResourceState::None;
    }

    #[no_mangle]
    pub unsafe extern "C" fn mu_textbox_ex(&mut self, buf: &mut dyn IString, opt: WidgetOption) -> ResourceState {
        let id: mu_Id = self.mu_get_id_from_ptr(buf);
        let r: mu_Rect = self.mu_layout_next();
        return self.mu_textbox_raw(buf, id, r, opt);
    }

    #[no_mangle]
    pub unsafe extern "C" fn mu_slider_ex(
        &mut self,
        mut value: &mut mu_Real,
        low: mu_Real,
        high: mu_Real,
        step: mu_Real,
        fmt: &str,
        opt: WidgetOption,
    ) -> ResourceState {
        let mut thumb: mu_Rect = mu_Rect { x: 0, y: 0, w: 0, h: 0 };
        let mut x: libc::c_int = 0;
        let mut w: libc::c_int = 0;
        let mut res = ResourceState::None;
        let last: mu_Real = *value;
        let mut v: mu_Real = last;
        let id: mu_Id = self.mu_get_id_from_ptr(value);
        let base: mu_Rect = self.mu_layout_next();
        if !self.number_textbox(&mut v, base, id).is_none() {
            return res;
        }
        self.mu_update_control(id, base, opt);
        if self.focus == id && (!self.mouse_down.is_none() | self.mouse_pressed.is_left()) {
            v = low + (self.mouse_pos.x - base.x) as libc::c_float * (high - low) / base.w as libc::c_float;
            if step != 0. {
                v = (v + step / 2 as libc::c_int as libc::c_float) / step * step;
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
            res.change();
        }
        self.mu_draw_control_frame(id, base, ControlColor::Base, opt);
        w = self.style.thumb_size;
        x = ((v - low) * (base.w - w) as libc::c_float / (high - low)) as libc::c_int;
        thumb = mu_rect(base.x + x, base.y, w, base.h);
        self.mu_draw_control_frame(id, thumb, ControlColor::Button, opt);
        // TODO: change to variadic format
        let mut buff = FixedString::<64>::new();
        buff.write_fmt(format_args!("{:.2}", v)).unwrap();
        self.mu_draw_control_text(buff.as_str(), base, ControlColor::Text, opt);
        return res;
    }

    #[no_mangle]
    pub unsafe extern "C" fn mu_number_ex(&mut self, value: &mut mu_Real, step: mu_Real, fmt: *const libc::c_char, opt: WidgetOption) -> ResourceState {
        let mut buf: FixedVec<char, 128> = FixedVec::default();
        let mut res = ResourceState::None;
        let id: mu_Id = self.mu_get_id_from_ptr(value);
        let base: mu_Rect = self.mu_layout_next();
        let last: mu_Real = *value;
        if !self.number_textbox(value, base, id).is_none() {
            return res;
        }
        self.mu_update_control(id, base, opt);
        if self.focus == id && self.mouse_down.is_left() {
            *value += self.mouse_delta.x as libc::c_float * step;
        }
        if *value != last {
            res.change();
        }
        self.mu_draw_control_frame(id, base, ControlColor::Base, opt);
        // TODO: change to variadic format
        let mut buff = FixedString::<64>::new();
        buff.write_fmt(format_args!("{:.2}", value)).unwrap();
        self.mu_draw_control_text(buff.as_str(), base, ControlColor::Text, opt);
        return res;
    }

    unsafe extern "C" fn header(&mut self, label: &str, istreenode: libc::c_int, opt: WidgetOption) -> ResourceState {
        let mut r: mu_Rect = mu_Rect { x: 0, y: 0, w: 0, h: 0 };
        let mut active: libc::c_int = 0;
        let mut expanded: libc::c_int = 0;
        let id: mu_Id = self.mu_get_id_from_str(label);
        let idx = self.treenode_pool.get(id);
        let mut width: libc::c_int = -(1 as libc::c_int);
        self.mu_layout_row(1 as libc::c_int, &mut width, 0);
        active = idx.is_some() as libc::c_int;
        expanded = if opt.is_expanded() { (active == 0) as libc::c_int } else { active };
        r = self.mu_layout_next();
        self.mu_update_control(id, r, WidgetOption::None);
        active ^= (self.mouse_pressed.is_left() && self.focus == id) as libc::c_int;
        if idx.is_some() {
            if active != 0 {
                self.treenode_pool.update(idx.unwrap(), self.frame);
            } else {
                self.treenode_pool.reset(idx.unwrap());
            }
        } else if active != 0 {
            self.treenode_pool.alloc(id, self.frame);
        }

        if istreenode != 0 {
            if self.hover == id {
                (self.draw_frame).expect("non-null function pointer")(self, r, ControlColor::ButtonHover);
            }
        } else {
            self.mu_draw_control_frame(id, r, ControlColor::Button, WidgetOption::None);
        }
        self.mu_draw_icon(
            if expanded != 0 { Icon::Expanded } else { Icon::Collapsed },
            mu_rect(r.x, r.y, r.h, r.h),
            self.style.colors[ControlColor::Text as libc::c_int as usize],
        );
        r.x += r.h - self.style.padding;
        r.w -= r.h - self.style.padding;
        self.mu_draw_control_text(label, r, ControlColor::Text, WidgetOption::None);
        return if expanded != 0 { ResourceState::Active } else { ResourceState::None };
    }

    #[no_mangle]
    pub unsafe extern "C" fn mu_header_ex(&mut self, label: &str, opt: WidgetOption) -> ResourceState {
        return self.header(label, 0 as libc::c_int, opt);
    }

    #[no_mangle]
    pub unsafe extern "C" fn mu_begin_treenode_ex(&mut self, label: &str, opt: WidgetOption) -> ResourceState {
        let res = self.header(label, 1 as libc::c_int, opt);
        if res.is_active() {
            self.get_layout_mut().indent += self.style.indent;
            self.id_stack.push(self.last_id);
        }
        return res;
    }

    #[no_mangle]
    pub unsafe extern "C" fn mu_end_treenode(&mut self) {
        self.get_layout_mut().indent -= self.style.indent;
        self.mu_pop_id();
    }

    fn scrollbars(&mut self, cnt_id: usize, body: &mut mu_Rect) {
        let sz: libc::c_int = self.style.scrollbar_size;
        let mut cs: mu_Vec2 = self.containers[cnt_id].content_size;
        cs.x += self.style.padding * 2 as libc::c_int;
        cs.y += self.style.padding * 2 as libc::c_int;
        self.mu_push_clip_rect(body.clone());
        if cs.y > self.containers[cnt_id].body.h {
            body.w -= sz;
        }
        if cs.x > self.containers[cnt_id].body.w {
            body.h -= sz;
        }
        let body = *body;
        let maxscroll: libc::c_int = cs.y - body.h;
        if maxscroll > 0 as libc::c_int && body.h > 0 as libc::c_int {
            let mut base: mu_Rect = mu_Rect { x: 0, y: 0, w: 0, h: 0 };
            let mut thumb: mu_Rect = mu_Rect { x: 0, y: 0, w: 0, h: 0 };
            let id: mu_Id = self.mu_get_id_from_str("!scrollbary");
            base = body;
            base.x = body.x + body.w;
            base.w = self.style.scrollbar_size;
            self.mu_update_control(id, base, WidgetOption::None);
            if self.focus == id && self.mouse_down.is_left() {
                self.containers[cnt_id].scroll.y += self.mouse_delta.y * cs.y / base.h;
            }
            self.containers[cnt_id].scroll.y = if maxscroll
                < (if 0 as libc::c_int > self.containers[cnt_id].scroll.y {
                    0 as libc::c_int
                } else {
                self.containers[cnt_id].scroll.y
                }) {
                maxscroll
            } else if 0 as libc::c_int > self.containers[cnt_id].scroll.y {
                0 as libc::c_int
            } else {
                self.containers[cnt_id].scroll.y
            };

            (self.draw_frame).expect("non-null function pointer")(self, base, ControlColor::ScrollBase);
            thumb = base;
            thumb.h = if self.style.thumb_size > base.h * body.h / cs.y {
                self.style.thumb_size
            } else {
                base.h * body.h / cs.y
            };
            thumb.y += self.containers[cnt_id].scroll.y * (base.h - thumb.h) / maxscroll;
            (self.draw_frame).expect("non-null function pointer")(self, thumb, ControlColor::ScrollThumb);
            if self.mu_mouse_over(body) != 0 {
                self.scroll_target = Some(cnt_id);
            }
        } else {
            self.containers[cnt_id].scroll.y = 0 as libc::c_int;
        }
        let maxscroll_0: libc::c_int = cs.x - body.w;
        if maxscroll_0 > 0 as libc::c_int && body.w > 0 as libc::c_int {
            let mut base_0: mu_Rect = mu_Rect { x: 0, y: 0, w: 0, h: 0 };
            let mut thumb_0: mu_Rect = mu_Rect { x: 0, y: 0, w: 0, h: 0 };
            let id_0: mu_Id = self.mu_get_id_from_str("!scrollbarx");
            base_0 = body;
            base_0.y = body.y + body.h;
            base_0.h = self.style.scrollbar_size;
            self.mu_update_control(id_0, base_0, WidgetOption::None);
            if self.focus == id_0 && self.mouse_down.is_left() {
                self.containers[cnt_id].scroll.x += self.mouse_delta.x * cs.x / base_0.w;
            }
            self.containers[cnt_id].scroll.x = if maxscroll_0
                < (if 0 as libc::c_int > self.containers[cnt_id].scroll.x {
                    0 as libc::c_int
                } else {
                self.containers[cnt_id].scroll.x
                }) {
                maxscroll_0
            } else if 0 as libc::c_int > self.containers[cnt_id].scroll.x {
                0 as libc::c_int
            } else {
                self.containers[cnt_id].scroll.x
            };
            (self.draw_frame).expect("non-null function pointer")(self, base_0, ControlColor::ScrollBase);
            thumb_0 = base_0;
            thumb_0.w = if self.style.thumb_size > base_0.w * body.w / cs.x {
                self.style.thumb_size
            } else {
                base_0.w * body.w / cs.x
            };
            thumb_0.x += self.containers[cnt_id].scroll.x * (base_0.w - thumb_0.w) / maxscroll_0;
            (self.draw_frame).expect("non-null function pointer")(self, thumb_0, ControlColor::ScrollThumb);
            if self.mu_mouse_over(body) != 0 {
                self.scroll_target = Some(cnt_id);
            }
        } else {
            self.containers[cnt_id].scroll.x = 0 as libc::c_int;
        }
        self.mu_pop_clip_rect();
    }

    unsafe extern "C" fn push_container_body(&mut self, cnt_idx: usize, body: mu_Rect, opt: WidgetOption) {
        let mut body = body;
        if !opt.has_no_scroll() {
            self.scrollbars(cnt_idx, &mut body);
        }
        self.push_layout(expand_rect(body, -self.style.padding), self.containers[cnt_idx].scroll);
        self.containers[cnt_idx].body = body;
    }

    unsafe extern "C" fn begin_root_container(&mut self, mut cnt: usize) {
        let cnt_ptr = &mut self.containers[cnt] as *mut mu_Container;
        self.container_stack.push(cnt);

        self.root_list.push(cnt);
        (*cnt_ptr).head_idx = Some(self.push_jump(None));
        if rect_overlaps_vec2((*cnt_ptr).rect, self.mouse_pos) && (self.next_hover_root.is_none() || (*cnt_ptr).zindex > self.containers[self.next_hover_root.unwrap()].zindex) {
            self.next_hover_root = Some(cnt);
        }
        self.clip_stack.push(unclipped_rect);
    }

    unsafe extern "C" fn end_root_container(&mut self) {
        let mut cnt: *mut mu_Container = self.mu_get_current_container();
        (*cnt).tail_idx = Some(self.push_jump(None));
        self.command_list[(*cnt).head_idx.unwrap()].jump.dst_idx = Some(self.command_list.len());
        self.mu_pop_clip_rect();
        self.pop_container();
    }

    #[no_mangle]
    pub unsafe extern "C" fn mu_begin_window_ex(&mut self, title: &str, mut rect: mu_Rect, opt: WidgetOption) -> ResourceState {
        let mut body: mu_Rect = mu_Rect { x: 0, y: 0, w: 0, h: 0 };
        let id: mu_Id = self.mu_get_id_from_str(title);
        let cnt_id = self.get_container_index(id, opt);
        if cnt_id.is_none() || self.containers[cnt_id.unwrap()].open == 0 {
            return ResourceState::None;
        }
        self.id_stack.push(id);

        let mut cnt = &mut self.containers[cnt_id.unwrap()] as *mut mu_Container;

        if (*cnt).rect.w == 0 as libc::c_int {
            (*cnt).rect = rect;
        }
        self.begin_root_container(cnt_id.unwrap());
        body = (*cnt).rect;
        rect = body;
        if !opt.has_no_frame() {
            (self.draw_frame).expect("non-null function pointer")(self, rect, ControlColor::WindowBG);
        }
        if !opt.has_no_title() {
            let mut tr: mu_Rect = rect;
            tr.h = self.style.title_height;
            (self.draw_frame).expect("non-null function pointer")(self, tr, ControlColor::TitleBG);

            // TODO: Is this necessary?
            if !opt.has_no_title() {
                let id_0: mu_Id = self.mu_get_id_from_str("!title");
                self.mu_update_control(id_0, tr, opt);
                self.mu_draw_control_text(title, tr, ControlColor::TitleText, opt);
                if id_0 == self.focus && self.mouse_down.is_left() {
                    (*cnt).rect.x += self.mouse_delta.x;
                    (*cnt).rect.y += self.mouse_delta.y;
                }
                body.y += tr.h;
                body.h -= tr.h;
            }
            if !opt.has_no_close() {
                let id_1: mu_Id = self.mu_get_id_from_str("!close");
                let r: mu_Rect = mu_rect(tr.x + tr.w - tr.h, tr.y, tr.h, tr.h);
                tr.w -= r.w;
                self.mu_draw_icon(Icon::Close, r, self.style.colors[ControlColor::TitleText as libc::c_int as usize]);
                self.mu_update_control(id_1, r, opt);
                if self.mouse_pressed.is_left() && id_1 == self.focus {
                    (*cnt).open = 0 as libc::c_int;
                }
            }
        }
        self.push_container_body(cnt_id.unwrap(), body, opt);
        if !opt.is_auto_sizing() {
            let sz: libc::c_int = self.style.title_height;
            let id_2: mu_Id = self.mu_get_id_from_str("!resize");
            let r_0: mu_Rect = mu_rect(rect.x + rect.w - sz, rect.y + rect.h - sz, sz, sz);
            self.mu_update_control(id_2, r_0, opt);
            if id_2 == self.focus && self.mouse_down.is_left() {
                (*cnt).rect.w = if 96 as libc::c_int > (*cnt).rect.w + self.mouse_delta.x {
                    96 as libc::c_int
                } else {
                    (*cnt).rect.w + self.mouse_delta.x
                };
                (*cnt).rect.h = if 64 as libc::c_int > (*cnt).rect.h + self.mouse_delta.y {
                    64 as libc::c_int
                } else {
                    (*cnt).rect.h + self.mouse_delta.y
                };
            }
        }
        if opt.is_auto_sizing() {
            let r_1: mu_Rect = self.get_layout().body;
            (*cnt).rect.w = (*cnt).content_size.x + ((*cnt).rect.w - r_1.w);
            (*cnt).rect.h = (*cnt).content_size.y + ((*cnt).rect.h - r_1.h);
        }

        if opt.is_popup() && !self.mouse_pressed.is_none() && self.hover_root != cnt_id {
            (*cnt).open = 0 as libc::c_int;
        }
        self.mu_push_clip_rect((*cnt).body);
        return ResourceState::Active;
    }

    #[no_mangle]
    pub unsafe extern "C" fn mu_end_window(&mut self) {
        self.mu_pop_clip_rect();
        self.end_root_container();
    }

    #[no_mangle]
    pub unsafe extern "C" fn mu_open_popup(&mut self, name: &str) {
        let cnt = self.mu_get_container_index(name);
        self.next_hover_root = cnt;
        self.hover_root = self.next_hover_root;
        self.containers[cnt.unwrap()].rect = mu_rect(self.mouse_pos.x, self.mouse_pos.y, 1 as libc::c_int, 1 as libc::c_int);
        self.containers[cnt.unwrap()].open = 1 as libc::c_int;
        self.mu_bring_to_front(cnt.unwrap());
    }

    #[no_mangle]
    pub unsafe extern "C" fn mu_begin_popup(&mut self, name: &str) -> ResourceState {
        let opt = WidgetOption::Popup
            .with_auto_size()
            .with_no_resize()
            .with_no_scroll()
            .with_no_title()
            .with_closed();
        return self.mu_begin_window_ex(name, mu_rect(0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int), opt);
    }

    #[no_mangle]
    pub unsafe extern "C" fn mu_end_popup(&mut self) {
        self.mu_end_window();
    }

    #[no_mangle]
    pub unsafe extern "C" fn mu_begin_panel_ex(&mut self, name: &str, opt: WidgetOption) {
        self.mu_push_id_from_str(name);
        let cnt_id = self.get_container_index(self.last_id, opt);
        let rect = self.mu_layout_next();
        self.containers[cnt_id.unwrap()].rect = rect;
        if !opt.has_no_frame() {
            (self.draw_frame).expect("non-null function pointer")(self, rect, ControlColor::PanelBG);
        }

        self.container_stack.push(cnt_id.unwrap());
        self.push_container_body(cnt_id.unwrap(), rect, opt);
        self.mu_push_clip_rect(self.containers[cnt_id.unwrap()].body);
    }

    #[no_mangle]
    pub unsafe extern "C" fn mu_end_panel(&mut self) {
        self.mu_pop_clip_rect();
        self.pop_container();
    }
}
