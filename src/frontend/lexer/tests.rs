//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use super::*;

#[test]
fn test_lexer_next_token_returns_token()
{
    let s = "+";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut lexer = Lexer::new(String::from("test.vscfl"), &mut cursor);
    match lexer.next_token() {
        Ok((Token::Plus, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_lexer_next_token_skips_one_line_comments()
{
    let s = "
// first text
// second text

+
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(String::from("test.vscfl"), &mut cursor);
    match lexer.next_token() {
        Ok((Token::Plus, pos)) => {
            assert_eq!(4, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_lexer_next_token_skips_star_comments()
{
    let s = "
/* first text */
/* second text
 * third text
 **/

+
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(String::from("test.vscfl"), &mut cursor);
    match lexer.next_token() {
        Ok((Token::Plus, pos)) => {
            assert_eq!(6, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_lexer_next_token_skips_comments_after_token()
{
    let s = "
+
// first text
/* second text
 **/
-
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(String::from("test.vscfl"), &mut cursor);
    match lexer.next_token() {
        Ok((Token::Plus, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Minus, pos)) => {
            assert_eq!(5, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_lexer_undo_token_undoes_two_tokens()
{
    let s = "+-";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut lexer = Lexer::new(String::from("test.vscfl"), &mut cursor);
    let (token1, pos1) = lexer.next_token().unwrap();
    let (token2, pos2) = lexer.next_token().unwrap();
    lexer.undo_token(token2, pos2);
    lexer.undo_token(token1, pos1);
    match lexer.next_token() {
        Ok((Token::Plus, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Minus, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(2, pos.column);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_lexer_undo_token_returns_inpunctuation_tokens()
{
    let s = "()[]{}!*/%+-<<>><>=><===!=&^|=@.<--><->=>,:;";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut lexer = Lexer::new(String::from("test.vscfl"), &mut cursor);
    match lexer.next_token() {
        Ok((Token::LParen, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::RParen, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(2, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::LBracket, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(3, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::RBracket, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(4, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::LBrace, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(5, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::RBrace, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(6, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Ex, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(7, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Star, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(8, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Slash, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(9, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Perc, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(10, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Plus, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(11, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Minus, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(12, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::LtLt, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(13, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::GtGt, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(15, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Lt, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(17, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::GtEq, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(18, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Gt, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(20, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::LtEq, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(21, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::EqEq, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(23, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::ExEq, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(25, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Amp, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(27, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Caret, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(28, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Bar, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(29, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Eq, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(30, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::At, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(31, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Dot, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(32, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::LArrow, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(33, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::RArrow, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(35, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::DArrow, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(37, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::EqGt, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(40, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Comma, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(42, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Colon, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(43, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Semi, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(44, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Eof, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(45, pos.column);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_lexer_undo_token_returns_keyword_tokens()
{
    let s = "
_
as
builtin
data
else
false
for
if
impl
in
inline
let
match
printf
shared
then
trait
true
type
uniq
where
kernel
private
local
global
constant
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(String::from("test.vscfl"), &mut cursor);
    match lexer.next_token() {
        Ok((Token::Wildcard, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::As, pos)) => {
            assert_eq!(2, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Builtin, pos)) => {
            assert_eq!(3, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Data, pos)) => {
            assert_eq!(4, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Else, pos)) => {
            assert_eq!(5, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::False, pos)) => {
            assert_eq!(6, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::For, pos)) => {
            assert_eq!(7, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::If, pos)) => {
            assert_eq!(8, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Impl, pos)) => {
            assert_eq!(9, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::In, pos)) => {
            assert_eq!(10, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Inline, pos)) => {
            assert_eq!(11, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Let, pos)) => {
            assert_eq!(12, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Match, pos)) => {
            assert_eq!(13, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Printf, pos)) => {
            assert_eq!(14, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Shared, pos)) => {
            assert_eq!(15, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Then, pos)) => {
            assert_eq!(16, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Trait, pos)) => {
            assert_eq!(17, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::True, pos)) => {
            assert_eq!(18, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Type, pos)) => {
            assert_eq!(19, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Uniq, pos)) => {
            assert_eq!(20, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Where, pos)) => {
            assert_eq!(21, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Kernel, pos)) => {
            assert_eq!(22, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Private, pos)) => {
            assert_eq!(23, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Local, pos)) => {
            assert_eq!(24, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Global, pos)) => {
            assert_eq!(25, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Constant, pos)) => {
            assert_eq!(26, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Eof, pos)) => {
            assert_eq!(27, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_lexer_undo_token_returns_character_tokens()
{
    let s = "
'a'
'\\''
'\\\"'
'\"'
'\0'
'\\n'
'\\r'
'\\t'
'\\x2a'
'\\X3B'
'
'
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(String::from("test.vscfl"), &mut cursor);
    match lexer.next_token() {
        Ok((Token::Char(n), pos)) => {
            assert_eq!(b'a' as i8, n);
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Char(n), pos)) => {
            assert_eq!(b'\'' as i8, n);
            assert_eq!(2, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Char(n), pos)) => {
            assert_eq!(b'"' as i8, n);
            assert_eq!(3, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Char(n), pos)) => {
            assert_eq!(b'"' as i8, n);
            assert_eq!(4, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Char(n), pos)) => {
            assert_eq!(b'\0' as i8, n);
            assert_eq!(5, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Char(n), pos)) => {
            assert_eq!(b'\n' as i8, n);
            assert_eq!(6, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Char(n), pos)) => {
            assert_eq!(b'\r' as i8, n);
            assert_eq!(7, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Char(n), pos)) => {
            assert_eq!(b'\t' as i8, n);
            assert_eq!(8, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Char(n), pos)) => {
            assert_eq!(b'\x2a' as i8, n);
            assert_eq!(9, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Char(n), pos)) => {
            assert_eq!(b'\x3b' as i8, n);
            assert_eq!(10, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Char(n), pos)) => {
            assert_eq!(b'\n' as i8, n);
            assert_eq!(11, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Eof, pos)) => {
            assert_eq!(13, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_lexer_undo_token_returns_string_token()
{
    let s = "\"abc\\'\\\"'\0\\n\\r\\t\\x2a\\X3B\n\" \"\"";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut lexer = Lexer::new(String::from("test.vscfl"), &mut cursor);
    match lexer.next_token() {
        Ok((Token::String(bs), pos)) => {
            assert_eq!("abc\'\"'\0\n\r\t\x2a\x3b\n".as_bytes(), bs.as_slice());
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::String(bs), pos)) => {
            assert_eq!("".as_bytes(), bs.as_slice());
            assert_eq!(2, pos.line);
            assert_eq!(3, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Eof, pos)) => {
            assert_eq!(2, pos.line);
            assert_eq!(5, pos.column);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_lexer_undo_token_returns_integer_tokens()
{
    let s = "
1234
0o1234
0x12af
0XABcd
02345
3456i
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(String::from("test.vscfl"), &mut cursor);
    match lexer.next_token() {
        Ok((Token::Int(n), pos)) => {
            assert_eq!(1234, n);
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Int(n), pos)) => {
            assert_eq!(0o1234, n);
            assert_eq!(2, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Int(n), pos)) => {
            assert_eq!(0x12af, n);
            assert_eq!(3, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Int(n), pos)) => {
            assert_eq!(0xabcd, n);
            assert_eq!(4, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Int(n), pos)) => {
            assert_eq!(2345, n);
            assert_eq!(5, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Int(n), pos)) => {
            assert_eq!(3456, n);
            assert_eq!(6, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Eof, pos)) => {
            assert_eq!(7, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_lexer_undo_token_returns_long_integer_tokens()
{
    let s = "
1234I
0o1234I
0x12afI
0XABcdI
02345I
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(String::from("test.vscfl"), &mut cursor);
    match lexer.next_token() {
        Ok((Token::Long(n), pos)) => {
            assert_eq!(1234, n);
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Long(n), pos)) => {
            assert_eq!(0o1234, n);
            assert_eq!(2, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Long(n), pos)) => {
            assert_eq!(0x12af, n);
            assert_eq!(3, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Long(n), pos)) => {
            assert_eq!(0xabcd, n);
            assert_eq!(4, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Long(n), pos)) => {
            assert_eq!(2345, n);
            assert_eq!(5, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Eof, pos)) => {
            assert_eq!(6, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_lexer_undo_token_returns_unsigned_integer_tokens()
{
    let s = "
1234u
0o1234u
0x12afu
0XABcdu
02345u
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(String::from("test.vscfl"), &mut cursor);
    match lexer.next_token() {
        Ok((Token::Uint(n), pos)) => {
            assert_eq!(1234, n);
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Uint(n), pos)) => {
            assert_eq!(0o1234, n);
            assert_eq!(2, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Uint(n), pos)) => {
            assert_eq!(0x12af, n);
            assert_eq!(3, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Uint(n), pos)) => {
            assert_eq!(0xabcd, n);
            assert_eq!(4, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Uint(n), pos)) => {
            assert_eq!(2345, n);
            assert_eq!(5, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Eof, pos)) => {
            assert_eq!(6, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_lexer_undo_token_returns_unsigned_long_integer_tokens()
{
    let s = "
1234U
0o1234U
0x12afU
0XABcdU
02345U
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(String::from("test.vscfl"), &mut cursor);
    match lexer.next_token() {
        Ok((Token::Ulong(n), pos)) => {
            assert_eq!(1234, n);
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Ulong(n), pos)) => {
            assert_eq!(0o1234, n);
            assert_eq!(2, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Ulong(n), pos)) => {
            assert_eq!(0x12af, n);
            assert_eq!(3, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Ulong(n), pos)) => {
            assert_eq!(0xabcd, n);
            assert_eq!(4, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Ulong(n), pos)) => {
            assert_eq!(2345, n);
            assert_eq!(5, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Eof, pos)) => {
            assert_eq!(6, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_lexer_undo_token_returns_floating_point_number_tokens()
{
    let s = "
12.34
23.45e10
34E+12
45E-23
1234f
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(String::from("test.vscfl"), &mut cursor);
    match lexer.next_token() {
        Ok((Token::Float(n), pos)) => {
            assert_eq!(12.34, n);
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Float(n), pos)) => {
            assert_eq!(23.45e10, n);
            assert_eq!(2, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Float(n), pos)) => {
            assert_eq!(34e+12, n);
            assert_eq!(3, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Float(n), pos)) => {
            assert_eq!(45e-23, n);
            assert_eq!(4, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Float(n), pos)) => {
            assert_eq!(1234.0, n);
            assert_eq!(5, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Eof, pos)) => {
            assert_eq!(6, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_lexer_undo_token_returns_double_floating_point_number_tokens()
{
    let s = "
12.34F
23.45e10F
34E+12F
45E-23F
1234F
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(String::from("test.vscfl"), &mut cursor);
    match lexer.next_token() {
        Ok((Token::Double(n), pos)) => {
            assert_eq!(12.34, n);
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Double(n), pos)) => {
            assert_eq!(23.45e10, n);
            assert_eq!(2, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Double(n), pos)) => {
            assert_eq!(34e+12, n);
            assert_eq!(3, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Double(n), pos)) => {
            assert_eq!(45e-23, n);
            assert_eq!(4, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Double(n), pos)) => {
            assert_eq!(1234.0, n);
            assert_eq!(5, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.next_token() {
        Ok((Token::Eof, pos)) => {
            assert_eq!(6, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
}
