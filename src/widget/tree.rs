use crate::*;

pub struct TreeNode<'t> {
    label: &'t str,
    opt: WidgetOption,
}

impl<'t> TreeNode<'t> {
    pub const fn new(label: &'t str) -> Self { Self { label, opt: WidgetOption::empty() } }

    pub fn show(self, ctx: &mut Context, f: impl FnOnce(&mut Context)) {
        if !ctx.begin_treenode_ex(self.label, self.opt).is_none() {
            f(ctx);
            ctx.end_treenode();
        }
    }
}

impl Context {
    pub const fn treenode<'t>(&self, label: &'t str) -> TreeNode<'t> { TreeNode::new(label) }

    fn begin_treenode_ex(&mut self, label: &str, opt: WidgetOption) -> ResourceState {
        let res = self.header_ex(label, true, opt);
        if res.is_active() && self.last_id.is_some() {
            self.get_layout_mut().indent += self.style.indent;
            self.id_stack.push(self.last_id.unwrap());
        }
        res
    }

    fn end_treenode(&mut self) {
        self.get_layout_mut().indent -= self.style.indent;
        self.pop_id();
    }
}
