use codespan_reporting::diagnostic::Label;
use logos::Logos;

pub type Span = core::ops::Range<usize>;
pub type Diagnostic = codespan_reporting::diagnostic::Diagnostic<()>;

// TODO: change err macro if codespan_reporting is not used
macro_rules! err {
    [$span:expr, $($tk:literal),*] => {
        {
            let expected = [$($tk),*];
            let mut msg = "invalid syntax, expected".to_string();
            if expected.len() > 1 {
                msg.push_str(" one of: ");
            } else {
                msg.push_str(": ");
            }
            let mut count = 0;
            for e in expected {
                count += 1;
                let s = format!("{}", e);
                let s = if s.starts_with('<') && s.ends_with('>') && s.len() > 2 {
                    s
                } else {
                    format!("'{}'", s)
                };
                msg.push_str(&s);
                if count < expected.len() {
                    msg.push_str(", ");
                }
            }
            Diagnostic::error()
                .with_message(msg)
                .with_labels(vec![Label::primary((), $span.start as usize..$span.end as usize)])
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum LexerError {
    #[default]
    Invalid,
    // TODO: add more errors if required
}

impl LexerError {
    pub fn into_diagnostic(self, span: Span) -> Diagnostic {
        match self {
            Self::Invalid => Diagnostic::error()
                .with_message("invalid token")
                .with_labels(vec![Label::primary((), span)]),
        }
    }
}

// TODO: implement lexer
#[allow(clippy::upper_case_acronyms)]
#[derive(Logos, Debug, PartialEq, Copy, Clone)]
#[logos(error = LexerError)]
pub enum Token {
    EOF,
    #[token("if")]
    If,
    #[token("while")]
    While,
    #[token("{")]
    LCur,
    #[token("}")]
    RCur,
    Error,
}

// TODO: choose type of CstIndex (in some cases 32 bit is enough)
type CstIndex = usize;

// TODO: add context information to the parser if required
#[derive(Default)]
struct Context<'a> {
    marker: std::marker::PhantomData<&'a ()>
}

// TODO: extend tokenization (e.g. check for mismatched parentheses)
pub fn tokenize(
    lexer: logos::Lexer<Token>,
    diags: &mut Vec<Diagnostic>,
) -> (Vec<Token>, Vec<std::ops::Range<CstIndex>>) {
    let mut tokens = vec![];
    let mut ranges = vec![];

    for (token, span) in lexer.spanned() {
        match token {
            Ok(token) => {
                tokens.push(token);
            }
            Err(err) => {
                diags.push(err.into_diagnostic(span.clone()));
                tokens.push(Token::Error);
            }
        }
        ranges.push(span.start as CstIndex..span.end as CstIndex);
    }
    (tokens, ranges)
}

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

impl<'a> Parser<'a> {
    fn build(&mut self, rule: Rule, node: NodeRef, diags: &mut Vec<Diagnostic>) {}
}
