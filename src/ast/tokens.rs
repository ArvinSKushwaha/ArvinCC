use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum TokenizeError {
    #[error("No matching token found")]
    NoMatchingToken,
}

macro_rules! generate_greedy_parse {
    ($(#[$($t:tt)*])* $v: vis enum $a:ident {
        $($ttype:ident: $parse_string:literal),* $(,)?
    }) => {

        $(#[$($t)*])*
        $v enum $a {
            $($ttype,)*
            Unknown
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub(crate) struct TokenSize(usize, usize, $a);

        use anyhow::Result;
        use regex::Regex;
        use once_cell::sync::Lazy;

        static REGEX_COMPILED: Lazy<Vec<Regex>> = Lazy::new(|| {
            vec![
                $(Regex::new($parse_string).unwrap()),*
            ]
        });

        static TOKENTYPES: Lazy<Vec<$a>> = Lazy::new(|| {
            vec![
                $($a::$ttype,)*
                $a::Unknown
            ]
        });

        impl Tokens {
            fn from_str(a: &str) -> Result<TokenSize, TokenizeError> {
                let mut matches = REGEX_COMPILED.iter().enumerate().filter_map(|(i, m)| {
                    if let Some(m) = m.captures(a) {
                        Some((m.get(1).unwrap().start(), m.get(1).unwrap().end(), TOKENTYPES[i]))
                    } else {
                        None
                    }
                }).collect::<Vec<_>>();
                matches.sort_by(|a, b| a.0.cmp(&b.0));
                if matches.is_empty() {
                    Err(TokenizeError::NoMatchingToken)
                } else {
                    let &(start, end, token) = matches.iter().next().unwrap();
                    Ok(TokenSize(start, end, token))
                }
            }
        }
    };
}

generate_greedy_parse! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub(crate) enum Tokens {
        Number: "(((?i)0x[0-9a-fA-F]+)|((?i)0b[01]+)|(0|(0\\.|[1-9][0-9]*)[0-9]*)(e(\\+|-)?[0-9]+)?)($|[^\\d])",
        String: "[^\\\\]?(\"(\\\\\"|.)*?\")",

        // Special
        BlockCommentLeft: "(/\\*)",
        BlockCommentRight: "(\\*/)",
        SingleComment: "(//)",
        Asperand: "(@)",

        // Inc/Dec
        Increment: "(\\+\\+)",
        Decrement: "(--)",

        // Comparison Ops
        Equals: "(==)",
        Unequals: "(!=)",
        GreaterOrEqual: "(>=)",
        LessOrEquals: "(<=)",
        LogicalAnd: "(&&)",
        LogicalOr: "(\\|\\|)",
        LogicalNot: "(!)",

        // AssignOp
        Assign: "(=)",

        // StructOps
        Pointer: "(->)",
        Period: "(\\.)",

        // Arithmetic AssignOps
        AddAssign: "(\\+=)",
        SubAssign: "(\\-=)",
        MulAssign: "(\\*=)",
        DivAssign: "(/=)",
        ModAssign: "(%=)",

        // Binary AssignOps
        BinaryShiftLeftAssign: "(<<=)",
        BinaryShiftRightAssign: "(>>=)",
        BinaryAndAssign: "(&=)",
        BinaryOrAssign: "(\\|=)",
        BinaryXorAssign: "(\\^=)",

        // Binary Ops
        BinaryAnd: "(&)",
        BinaryOr: "(\\|)",
        BinaryNot: "(~)",
        BinaryXor: "(\\^)",
        BinaryShiftLeft: "(<<)",
        BinaryShiftRight: "(>>)",

        // Keywords
        Break: "(break)",
        Case: "(case)",
        Char: "(char)",
        Const: "(const)",
        Continue: "(continue)",
        Default: "(default)",
        Do: "(do)",
        Double: "(double)",
        Else: "(else)",
        Enum: "(enum)",
        Extern: "(extern)",
        Float: "(float)",
        For: "(for)",
        Goto: "(goto)",
        If: "(if)",
        Int: "(int)",
        Long: "(long)",
        Register: "(register)",
        Return: "(return)",
        Short: "(short)",
        Signed: "(signed)",
        SizeOf: "(sizeof)",
        Static: "(static)",
        Struct: "(struct)",
        Switch: "(switch)",
        Typedef: "(typedef)",
        Union: "(union)",
        Unsigned: "(unsigned)",
        Void: "(void)",
        Volatile: "(volatile)",
        While: "(while)",

        Comma: "(,)",
        SemiColon: "(;)",
        Colon: "(:)",
        Question: "(\\?)",
        Apostrophe: "(')",
        Quotation: "(\")",
        Slash: "(/)",
        BackSlash: "(\\\\)",
        Dollar: "(\\$)",
        Percent: "(%)",
        LeftCurly: "(\\{)",
        RightCurly: "(})",
        LeftBracket: "(\\[)",
        RightBracket: "(])",
        LeftParen: "(\\()",
        RightParen: "(\\))",
        Plus: "(\\+)",
        Minus: "(-)",
        Asterisk: "(\\*)",
        Less: "(<)",
        Greater: "(>)",
        Hash: "(#)",

        Identifier: "([_a-zA-Z][a-zA-Z0-9_]*)",
        Whitespace: "([\\t\\v\\n\\f ]+)",
    }
}

// To speed this up, we can do some fun with caching the results of [`Tokens::from_str(&str)`].
pub(crate) fn tokenize(mut str: &str) -> Vec<(usize, usize, Tokens)> {
    let mut tokens = Vec::new();
    let mut offset = 0;

    while let Ok(tokenizable) = Tokens::from_str(str) {
        let TokenSize(start, end, token) = tokenizable;
        if start != 0 {
            tokens.push((offset, offset + start, Tokens::Unknown));
        }
        tokens.push((offset + start, offset + end, token));
        str = &str[end..];
        offset += end;
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::{tokenize, TokenSize, Tokens};

    macro_rules! test_token_detection {
        ($l:literal, $i:ident) => {
            assert!(matches!(
                Tokens::from_str($l).ok(),
                Some(TokenSize(0, _, Tokens::$i))
            ));
        };
        ($l:literal) => {
            assert!(!matches!(
                Tokens::from_str($l).ok(),
                Some(TokenSize(0, _, _))
            ));
        };
    }

    #[test]
    fn test_token_detection() {
        test_token_detection!("alphaBeta_theta", Identifier);
        test_token_detection!("1e+9", Number);
        test_token_detection!("1e-9", Number);
        test_token_detection!("1e9", Number);
        test_token_detection!("0.1e+9", Number);
        test_token_detection!("1.0045", Number);
        test_token_detection!("0.0045", Number);
        test_token_detection!("102355", Number);
        test_token_detection!("<13566", Less);
        test_token_detection!("/*", BlockCommentLeft);
        test_token_detection!("*/", BlockCommentRight);
        test_token_detection!("//", SingleComment);
        test_token_detection!(" ", Whitespace);
        test_token_detection!("\"T'was upon this cursed day that on which he proclaimed, \\\"Such protest shall prove futile!\\\", as armies lay slain\"", String);

        test_token_detection!("023405");
        test_token_detection!("");
    }

    macro_rules! test_tokenizer {
        ($l:literal, $(($a:literal, $b:literal, $c:ident)),* $(,)?) => {
            assert_eq!(tokenize($l), vec![$(($a, $b, Tokens::$c)),*]);
        }
    }

    #[test]
    fn test_tokenizer_on_str() {
        test_tokenizer!("a", (0, 1, Identifier));
        test_tokenizer!("a=5", (0, 1, Identifier), (1, 2, Assign), (2, 3, Number));
        test_tokenizer!(
            "Here are some words;",
            (0, 4, Identifier),
            (4, 5, Whitespace),
            (5, 8, Identifier),
            (8, 9, Whitespace),
            (9, 13, Identifier),
            (13, 14, Whitespace),
            (14, 19, Identifier),
            (19, 20, SemiColon),
        );
        test_tokenizer!(
            "1e+5-1+5*0x1245AFG",
            (0, 4, Number),
            (4, 5, Minus),
            (5, 6, Number),
            (6, 7, Plus),
            (7, 8, Number),
            (8, 9, Asterisk),
            (9, 17, Number),
            (17, 18, Identifier),
        );
        test_tokenizer!(
            ">>!=2",
            (0, 2, BinaryShiftRight),
            (2, 4, Unequals),
            (4, 5, Number)
        );
        test_tokenizer!(
            "2^(2++)",
            (0, 1, Number),
            (1, 2, BinaryXor),
            (2, 3, LeftParen),
            (3, 4, Number),
            (4, 6, Increment),
            (6, 7, RightParen)
        );
        test_tokenizer!(
            "avada.kedavra_imperio^crucio",
            (0, 5, Identifier),
            (5, 6, Period),
            (6, 21, Identifier),
            (21, 22, BinaryXor),
            (22, 28, Identifier)
        );

        test_tokenizer!(
            "const int *(void (*vtable)[])[]",
            (0, 5, Const),
            (5, 6, Whitespace),
            (6, 9, Int),
            (9, 10, Whitespace),
            (10, 11, Asterisk),
            (11, 12, LeftParen),
            (12, 16, Void),
            (16, 17, Whitespace),
            (17, 18, LeftParen),
            (18, 19, Asterisk),
            (19, 25, Identifier),
            (25, 26, RightParen),
            (26, 27, LeftBracket),
            (27, 28, RightBracket),
            (28, 29, RightParen),
            (29, 30, LeftBracket),
            (30, 31, RightBracket)
        );

        test_tokenizer!(
            "return 0;",
            (0, 6, Return),
            (6, 7, Whitespace),
            (7, 8, Number),
            (8, 9, SemiColon),
        );

        test_tokenizer!(
            "\"Here are multiple strings\" \"And one with quite \\\"extravagant\\\" usage of escapes \\r\\nand a newline!\" \"Amongst other things...\"",
            (0, 27, String),
            (27, 28, Whitespace),
            (28, 100, String),
            (100, 101, Whitespace),
            (101, 126, String),
        );
    }

    #[test]
    fn test_tokenizer_on_code() {
        let code_snippets = [
            r#"
                #include <stdio.h>
                int main(int argc, const char **argv) {
                    printf("Hello, world!");
                    return 0;
                }
            "#,
            r#"
                #include <stdio.h>
                // Here's a comment!
                /*
                 * And another one!
                 */
                int main() {
                    char c;
                    for (c = 'A'; c <= 'Z'; ++c)
                        printf("%c ", c);
                    return 0;
                }
            "#,
        ];

        code_snippets
            .iter()
            .for_each(|snippet| assert!(!tokenize(snippet).is_empty()));
    }
}

