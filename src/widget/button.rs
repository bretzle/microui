use crate::*;

enum ButtonLabel<'t> {
    Text(&'t str),
    Icon(Icon),
}

pub struct Button<'t> {
    label: ButtonLabel<'t>,
    opt: WidgetOption,
}

impl<'t> Button<'t> {
    fn new(label: ButtonLabel<'t>) -> Self { Self { label, opt: WidgetOption::ALIGN_CENTER } }

    pub fn show(self, ctx: &mut Context, f: impl FnOnce(&mut Context)) {
        if !ctx.button_ex(self.label, self.opt).is_none() {
            f(ctx);
        }
    }
}

impl Context {
    pub fn button<'t>(&self, label: &'t str) -> Button<'t> { Button::new(ButtonLabel::Text(label)) }

    pub fn button_icon(&self, icon: Icon) -> Button<'_> { Button::new(ButtonLabel::Icon(icon)) }

    fn button_ex(&mut self, label: ButtonLabel, opt: WidgetOption) -> ResourceState {
        let mut res = ResourceState::empty();
        let id = match label {
            ButtonLabel::Text(text) => self.get_id_from_str(text),
            ButtonLabel::Icon(icon) => self.get_id_u32(icon as _),
        };

        let r = self.layout_next();
        self.update_control(id, r, opt);
        if self.mouse_pressed.is_left() && self.focus == Some(id) {
            res |= ResourceState::SUBMIT;
        }
        self.draw_control_frame(id, r, ControlColor::Button, opt);

        match label {
            ButtonLabel::Text(text) => self.draw_control_text(text, r, ControlColor::Text, opt),
            ButtonLabel::Icon(icon) => self.draw_icon(icon, r, self.style.colors[ControlColor::Text as usize]),
        };

        res
    }
}
