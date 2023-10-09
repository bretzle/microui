use crate::*;

pub struct Panel<'a> {
    name: &'a str,
    opt: WidgetOption,
}

impl<'a> Panel<'a> {
    pub const fn new(name: &'a str) -> Self { Self { name, opt: WidgetOption::empty() } }

    pub const fn options(mut self, opt: WidgetOption) -> Self {
        self.opt = opt;
        self
    }

    pub fn show(self, ctx: &mut Context, f: impl FnOnce(&mut Context)) {
        ctx.begin_panel(self.name, self.opt);
        f(ctx);
        ctx.end_panel();
    }
}

impl Context {
    pub const fn panel<'a>(&self, name: &'a str) -> Panel<'a> { Panel::new(name) }

    fn begin_panel(&mut self, name: &str, opt: WidgetOption) {
        self.push_id_from_str(name);
        let cnt_id = self.get_container_index_intern(self.last_id.unwrap(), opt);
        let rect = self.layout_next();
        self.containers[cnt_id.unwrap()].rect = rect;
        if !opt.has_no_frame() {
            self.draw_frame(rect, ControlColor::PanelBG);
        }

        self.container_stack.push(cnt_id.unwrap());
        self.push_container_body(cnt_id.unwrap(), rect, opt);
        self.push_clip_rect(self.containers[cnt_id.unwrap()].body);
    }

    fn end_panel(&mut self) {
        self.pop_clip_rect();
        self.pop_container();
    }
}
