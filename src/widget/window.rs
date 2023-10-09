use crate::*;

pub struct Window<'t> {
    title: &'t str,
    r: Rect,
    opt: WidgetOption,
}

impl<'t> Window<'t> {
    pub const fn new(title: &'t str) -> Self {
        Self {
            title,
            r: Rect::ZERO,
            opt: WidgetOption::empty(),
        }
    }

    pub const fn position(mut self, x: i32, y: i32) -> Self {
        self.r.x = x;
        self.r.y = y;
        self
    }

    pub const fn size(mut self, w: i32, h: i32) -> Self {
        self.r.w = w;
        self.r.h = h;
        self
    }

    pub const fn options(mut self, opt: WidgetOption) -> Self {
        self.opt = opt;
        self
    }

    pub fn show(self, ctx: &mut Context, f: impl FnOnce(&mut Context)) {
        if !ctx.begin_window(self.title, self.r, self.opt).is_none() {
            f(ctx);
            ctx.end_window();
        }
    }
}

pub struct Popup<'t>(Window<'t>);

impl<'t> Popup<'t> {
    pub fn new(title: &'t str) -> Self {
        Self(Window::new(title).options(
            WidgetOption::POPUP | WidgetOption::AUTO_SIZE | WidgetOption::NO_RESIZE | WidgetOption::NO_SCROLL | WidgetOption::NO_TITLE | WidgetOption::CLOSED,
        ))
    }

    pub fn show(self, ctx: &mut Context, f: impl FnOnce(&mut Context)) { self.0.show(ctx, f) }
}

impl Context {
    pub const fn window<'t>(&self, title: &'t str) -> Window<'t> { Window::new(title) }

    pub fn popup<'t>(&self, title: &'t str) -> Popup<'t> { Popup::new(title) }

    pub fn open_popup(&mut self, name: &str) {
        let cnt = self.get_container_index(name);
        self.next_hover_root = cnt;
        self.hover_root = self.next_hover_root;
        self.containers[cnt.unwrap()].rect = rect(self.mouse_pos.x, self.mouse_pos.y, 1, 1);
        self.containers[cnt.unwrap()].open = true;
        self.bring_to_front(cnt.unwrap());
    }

    fn begin_window(&mut self, title: &str, mut r: Rect, opt: WidgetOption) -> ResourceState {
        let id = self.get_id_from_str(title);
        let cnt_id = self.get_container_index_intern(id, opt);
        if cnt_id.is_none() || !self.containers[cnt_id.unwrap()].open {
            return ResourceState::empty();
        }
        self.id_stack.push(id);

        if self.containers[cnt_id.unwrap()].rect.w == 0 {
            self.containers[cnt_id.unwrap()].rect = r;
        }
        self.begin_root_container(cnt_id.unwrap());
        let mut body = self.containers[cnt_id.unwrap()].rect;
        r = body;
        if !opt.has_no_frame() {
            self.draw_frame(r, ControlColor::WindowBG);
        }
        if !opt.has_no_title() {
            let mut tr = r;
            tr.h = self.style.title_height;
            self.draw_frame(tr, ControlColor::TitleBG);

            let id = self.get_id_from_str("!title");
            self.update_control(id, tr, opt);
            self.draw_control_text(title, tr, ControlColor::TitleText, opt);
            if Some(id) == self.focus && self.mouse_down.is_left() {
                self.containers[cnt_id.unwrap()].rect.x += self.mouse_delta.x;
                self.containers[cnt_id.unwrap()].rect.y += self.mouse_delta.y;
            }
            body.y += tr.h;
            body.h -= tr.h;

            if !opt.has_no_close() {
                let id = self.get_id_from_str("!close");
                let r = rect(tr.x + tr.w - tr.h, tr.y, tr.h, tr.h);
                tr.w -= r.w;
                self.draw_icon(Icon::Close, r, self.style.colors[ControlColor::TitleText as usize]);
                self.update_control(id, r, opt);
                if self.mouse_pressed.is_left() && Some(id) == self.focus {
                    self.containers[cnt_id.unwrap()].open = false;
                }
            }
        }
        self.push_container_body(cnt_id.unwrap(), body, opt);
        if !opt.is_auto_sizing() {
            let sz = self.style.title_height;
            let id_2 = self.get_id_from_str("!resize");
            let r_0 = rect(r.x + r.w - sz, r.y + r.h - sz, sz, sz);
            self.update_control(id_2, r_0, opt);
            if Some(id_2) == self.focus && self.mouse_down.is_left() {
                self.containers[cnt_id.unwrap()].rect.w = if 96 > self.containers[cnt_id.unwrap()].rect.w + self.mouse_delta.x {
                    96
                } else {
                    self.containers[cnt_id.unwrap()].rect.w + self.mouse_delta.x
                };
                self.containers[cnt_id.unwrap()].rect.h = if 64 > self.containers[cnt_id.unwrap()].rect.h + self.mouse_delta.y {
                    64
                } else {
                    self.containers[cnt_id.unwrap()].rect.h + self.mouse_delta.y
                };
            }
        } else {
            let r_1 = self.get_layout().body;
            self.containers[cnt_id.unwrap()].rect.w = self.containers[cnt_id.unwrap()].content_size.x + (self.containers[cnt_id.unwrap()].rect.w - r_1.w);
            self.containers[cnt_id.unwrap()].rect.h = self.containers[cnt_id.unwrap()].content_size.y + (self.containers[cnt_id.unwrap()].rect.h - r_1.h);
        }

        if opt.is_popup() && !self.mouse_pressed.is_none() && self.hover_root != cnt_id {
            self.containers[cnt_id.unwrap()].open = false;
        }
        self.push_clip_rect(self.containers[cnt_id.unwrap()].body);
        ResourceState::ACTIVE
    }

    fn end_window(&mut self) {
        self.pop_clip_rect();
        self.end_root_container();
    }

    fn begin_root_container(&mut self, cnt: usize) {
        self.container_stack.push(cnt);

        self.root_list.push(cnt);
        self.containers[cnt].head_idx = Some(self.jump());
        if self.containers[cnt].rect.overlaps(self.mouse_pos)
            && (self.next_hover_root.is_none() || self.containers[cnt].zindex > self.containers[self.next_hover_root.unwrap()].zindex)
        {
            self.next_hover_root = Some(cnt);
        }
        self.clip_stack.push(Rect::UNCLIPPED);
    }

    fn end_root_container(&mut self) {
        let cnt = self.get_current_container();
        self.containers[cnt].tail_idx = Some(self.jump());
        self.pop_clip_rect();
        self.pop_container();
    }
}
