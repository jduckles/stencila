use codec_losses::lost_exec_options;

use crate::{prelude::*, If, IfClause};

impl If {
    pub fn to_markdown_special(&self, context: &mut MarkdownEncodeContext) -> (String, Losses) {
        let mut md = String::new();
        let mut losses = lost_exec_options!(self);

        let fence = ":".repeat(3 + context.depth * 2);

        context.down();

        for (index, IfClause { code, content, .. }) in self.clauses.iter().enumerate() {
            md.push_str(&fence);
            let keyword = if index == 0 {
                " if "
            } else if code.is_empty() && index == self.clauses.len() - 1 {
                " else "
            } else {
                " elif "
            };
            md.push_str(keyword);
            md.push_str(code);
            md.push_str("\n\n");

            let (content_md, content_losses) = content.to_markdown(context);
            md.push_str(&content_md);
            losses.merge(content_losses);
        }

        context.up();

        if !self.clauses.is_empty() {
            md.push_str(&fence);
            md.push_str("\n\n");
        }

        (md, losses)
    }
}
