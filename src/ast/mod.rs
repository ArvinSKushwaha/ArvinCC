#![allow(dead_code)]

mod types;

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
            $($ttype),*
        }

        use std::str::FromStr;
        use anyhow::Result;
        use regex::RegexSet;
        use once_cell::sync::Lazy;

        static REGEXES: Lazy<RegexSet> = Lazy::new(|| {
            RegexSet::new(&[
                $(concat!("^", $parse_string)),*
            ]).unwrap()
        });

        static TOKENTYPES: Lazy<Vec<$a>> = Lazy::new(|| {
            vec![
                $($a::$ttype),*
            ]
        });
        
        impl FromStr for $a {
            type Err = TokenizeError;
            fn from_str(a: &str) -> Result<Self, TokenizeError> {
                let matches = REGEXES.matches(a);
                if !matches.matched_any() {
                    Err(TokenizeError::NoMatchingToken)
                } else {
                    Ok(TOKENTYPES[matches.iter().next().unwrap()])
                }
            }
        }
    };
}

generate_greedy_parse! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub(crate) enum Tokens {
        Number: "(\\+|-)?(0\\.|[1-9][0-9]*)[0-9]*(e(\\+|-)?[0-9]+)?",

        Comma: ",",
        Period: "\\.",
        SemiColon: ";",
        Colon: ":",
        Question: "\\?",
        Apostrophe: "'",
        Quotation: "\"",
        Exclamation: "!",
        VerticalBar: "\\|",
        Slash: "/",
        BackSlash: "\\\\",
        Tilde: "~",
        Underscore: "_",
        Dollar: "$",
        Percent: "%",
        LeftCurly: "\\{",
        RightCurly: "}",
        LeftBracket: "\\[",
        RightBracket: "]",
        LeftParen: "\\(",
        RightParen: "\\)",
        Ampersand: "&",
        Caret: "\\^",
        Plus: "\\+",
        Minus: "-",
        Equals: "=",
        LeftAngle: "<",
        RightAngle: ">",
        Hash: "#",

        Identifier: "[_a-zA-Z][a-zA-Z0-9_]",
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::Tokens;

    macro_rules! test_tokenizer {
        ($l:literal, $i:ident) => {
            assert_eq!(Tokens::from_str($l).ok(), Some(Tokens::$i));
        };
        ($l:literal) => {
            assert!(Tokens::from_str($l).is_err());
        }
    }

    #[test]
    fn test_tokenizer() {
        test_tokenizer!("alphaBeta_theta", Identifier);
        test_tokenizer!("1e+9", Number);
        test_tokenizer!("1e-9", Number);
        test_tokenizer!("1e9", Number);
        test_tokenizer!("0.1e+9", Number);
        test_tokenizer!("-1.035", Number);
        test_tokenizer!("1.0045", Number);
        test_tokenizer!("-0.1024", Number);
        test_tokenizer!("102355", Number);
        test_tokenizer!("<13566", LeftAngle);


        test_tokenizer!("023405");
        test_tokenizer!(" ");
    }
}
