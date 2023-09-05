pub(crate) const KW_SEMI: &str = ";";
pub(crate) const KW_EXIT: &str = "exit";
pub(crate) const KW_OPEN_PARENTHESIS: &str = "(";
pub(crate) const KW_CLOSE_PARENTHESIS: &str = ")";
pub(crate) const KW_LET: &str = "let";
pub(crate) const KW_ASSIGN: &str = "=";

/// Keywords that should be split from other tokens (e.g. ";x;" -> ";", "x", ";") during
/// tokenization.  Tokens that could feasibly be included as substrings in other tokens should
/// be excluded from this list.
pub(crate) const TOKENIZER_KEYWORDS: [&str; 4] = [
    KW_SEMI,
    KW_OPEN_PARENTHESIS,
    KW_CLOSE_PARENTHESIS,
    KW_ASSIGN,
];
