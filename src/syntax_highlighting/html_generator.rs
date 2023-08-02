use syntect::parsing::{
    BasicScopeStackOp, ParseState, Scope, ScopeStack, ScopeStackOp, SyntaxReference, SyntaxSet,
    SCOPE_REPO,
};

pub struct HighlightedHtmlGenerator<'a> {
    syntax_set: &'a SyntaxSet,
    open_spans: isize,
    parse_state: ParseState,
    scope_stack: ScopeStack,
    html: String,
}

impl<'a> HighlightedHtmlGenerator<'a> {
    pub fn new_with_class_style(
        syntax_reference: &'a SyntaxReference,
        syntax_set: &'a SyntaxSet,
    ) -> HighlightedHtmlGenerator<'a> {
        let parse_state = ParseState::new(syntax_reference);
        let open_spans = 0;
        let html = String::new();
        let scope_stack = ScopeStack::new();

        HighlightedHtmlGenerator {
            syntax_set,
            open_spans,
            parse_state,
            scope_stack,
            html,
        }
    }

    /// Parse the line of code and update the internal HTML buffer with
    /// tagged HTML
    ///
    /// *Note:* This function requires `line` to include a newline at the
    /// end and also use of the `load_defaults_newlines` version of
    /// the syntaxes.
    pub fn parse_html_for_line_which_includes_newline(
        &mut self,
        line: &str,
    ) -> Result<(), syntect::Error> {
        let parsed_line = self.parse_state.parse_line(line, self.syntax_set)?;

        let (formatted_line, delta) =
            line_tokens_to_classed_spans(line, parsed_line.as_slice(), &mut self.scope_stack)?;

        self.open_spans += delta;

        self.html.push_str(r#"<span class="line-start"></span>"#);

        self.html.push_str(formatted_line.as_str());

        Ok(())
    }

    /// Close all open `<span>` tags and return the finished HTML string
    pub fn finalize(mut self) -> String {
        for _ in 0..self.open_spans {
            self.html.push_str("</span>");
        }

        self.html
    }
}

pub fn line_tokens_to_classed_spans(
    line: &str,
    ops: &[(usize, ScopeStackOp)],
    stack: &mut ScopeStack,
) -> Result<(String, isize), syntect::Error> {
    let mut s = String::with_capacity(line.len() + ops.len() * 8);
    let mut cur_index = 0;
    let mut span_delta = 0;

    let mut span_empty = false;
    let mut span_start = 0;

    for &(i, ref op) in ops {
        if i > cur_index {
            span_empty = false;
            html_escape::encode_text_minimal_to_string(&line[cur_index..i], &mut s);
            cur_index = i;
        }
        stack.apply_with_hook(op, |basic_op, _| match basic_op {
            BasicScopeStackOp::Push(scope) => {
                span_start = s.len();
                span_empty = true;
                s.push_str("<span class=\"");
                scope_to_classes(&mut s, scope);
                s.push_str("\">");
                span_delta += 1;
            }
            BasicScopeStackOp::Pop => {
                if span_empty {
                    s.truncate(span_start);
                } else {
                    s.push_str("</span>");
                }
                span_delta -= 1;
                span_empty = false;
            }
        })?;
    }

    html_escape::encode_text_minimal_to_string(&line[cur_index..line.len()], &mut s);

    Ok((s, span_delta))
}

fn scope_to_classes(s: &mut String, scope: Scope) {
    let repo = SCOPE_REPO.lock().unwrap();

    (0..scope.len())
        .map(|i| (i, repo.atom_str(scope.atom_at(i as usize))))
        .for_each(|(i, atom_s)| {
            if i != 0 {
                s.push(' ');
            }
            s.push_str(atom_s);
        });
}
