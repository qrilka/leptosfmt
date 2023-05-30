use syn_rsx::{Node, NodeBlock, NodeComment, NodeDoctype, NodeName, NodeText};

use crate::formatter::Formatter;

impl Formatter<'_> {
    pub fn node(&mut self, node: &Node) {
        match node {
            Node::Element(ele) => self.element(ele),
            Node::Attribute(attr) => self.attribute(attr),
            Node::Text(text) => self.node_text(text),
            Node::Comment(comment) => self.comment(comment),
            Node::Doctype(doctype) => self.doctype(doctype),
            Node::Block(block) => self.node_block(block),
            Node::Fragment(frag) => self.fragment(frag),
        }
    }

    pub fn comment(&mut self, comment: &NodeComment) {
        self.printer.word("<!-- ");
        self.node_value_expr(&comment.value, false, false);
        self.printer.word(" -->");
    }

    pub fn doctype(&mut self, doctype: &NodeDoctype) {
        self.printer.word("<!DOCTYPE ");
        self.node_value_expr(&doctype.value, false, false);
        self.printer.word("> ");
    }

    pub fn node_text(&mut self, text: &NodeText) {
        self.node_value_expr(&text.value, false, false);
    }

    pub fn node_name(&mut self, name: &NodeName) {
        self.printer.word(name.to_string());
    }

    pub fn node_block(&mut self, block: &NodeBlock) {
        self.node_value_expr(&block.value, false, false);
    }
}

#[cfg(test)]
mod tests {
    use crate::formatter::*;
    use crate::test_helpers::{comment, doctype};
    use leptosfmt_pretty_printer::Printer;

    macro_rules! format_comment {
        ($($tt:tt)*) => {{
            let comment = comment! { $($tt)* };
            let settings = FormatterSettings { max_width: 40, ..Default::default() };
            let mut printer = Printer::new((&settings).into());
            let mut formatter = Formatter::new(settings, &mut printer);
            formatter.comment(&comment);
            printer.eof()
        }};
    }

    macro_rules! format_doctype {
        ($($tt:tt)*) => {{
            let doctype = doctype! { $($tt)* };
            let settings = FormatterSettings { max_width: 40, ..Default::default() };
            let mut printer = Printer::new((&settings).into());
            let mut formatter = Formatter::new(settings, &mut printer);
            formatter.doctype(&doctype);
            printer.eof()
        }};
    }

    #[test]
    fn html_comment() {
        let formatted = format_comment!(<!--   "comment"   -->);
        insta::assert_snapshot!(formatted, @r###"<!-- "comment" -->"###);
    }

    #[test]
    fn html_comment_long() {
        let formatted = format_comment!(<!--   "this is a very loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong comment"   -->);
        insta::assert_snapshot!(formatted, @r###"<!-- "this is a very loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong comment" -->"###);
    }

    #[test]
    fn html_doctype() {
        let formatted = format_doctype!(< !DOCTYPE html   >);
        insta::assert_snapshot!(formatted, @"<!DOCTYPE html> ");
    }
}
