use crate::*;

pub struct Header<'t> {
    label: &'t str,
    opt: WidgetOption,
}

impl<'t> Header<'t> {
    pub const fn new(label: &'t str) -> Self { Self { label, opt: WidgetOption::empty() } }

    pub fn expanded(mut self) -> Self {
        self.opt |= WidgetOption::EXPANDED;
        self
    }

    pub fn show(self, ctx: &mut Context, f: impl FnOnce(&mut Context)) {
        if !ctx.header_ex(self.label, false, self.opt).is_none() {
            f(ctx)
        }
    }
}

impl Context {
    pub fn header<'t>(&self, label: &'t str) -> Header<'t> { Header::new(label) }
}
