use std::iter::Peekable;
use std::str::Chars;

use super::{Anchor, Expression, Pattern, PredefinedSet, Repetition, Sets, SubPattern};

pub(super) fn process<'a>(line: &'a str) -> Expression {
    let mut iter = line.chars().peekable();
    let mut expression: Expression = (Anchor::None, Vec::new());

    let mut anchor: Anchor = Anchor::None;
    if let Some('^') = iter.peek() {
        anchor = Anchor::Start;
        let _ = iter.next();
    };

    let mut escaped: bool = false;
    while let Some(ch) = iter.next_if(|&x| x != '$') {
        let pattern: Pattern;
        if escaped {
            escaped = false; //TODO: check if this fine
            pattern = Pattern {
                sub_pattern: SubPattern::Char(ch),
                repetition: check_repetition(&mut iter),
            };
        } else {
            pattern = match ch {
                '\\' => {
                    // confusing but this is for escape charachters
                    escaped = true;
                    continue;
                }

                '.' => Pattern {
                    sub_pattern: SubPattern::Dot,
                    repetition: check_repetition(&mut iter),
                },

                // need to include other ascii char too later
                'a'..='z' | '0'..='9' | 'A'..='Z' => Pattern {
                    sub_pattern: SubPattern::Char(ch),
                    repetition: check_repetition(&mut iter),
                },

                '[' => Pattern {
                    sub_pattern: scan_bracketed_expression(&mut iter),
                    repetition: check_repetition(&mut iter),
                },
                _ => unreachable!("Lets see what is that: {:#?}", (ch, iter)),
            };
        }
        expression.1.push(pattern);
    }

    if let Some('$') = iter.peek() {
        match anchor {
            Anchor::Start => {
                anchor = Anchor::Both;
            }
            Anchor::None => {
                anchor = Anchor::End;
            }
            _ => unreachable!("Some problem in anchor: {:#?}", anchor),
        };
        let _ = iter.next();
    };

    expression.0 = anchor;
    expression
}

#[inline] // take for example [[^[0-9]] and [[:punct:]A-Mm-z ]
fn scan_bracketed_expression<'a>(iter: &mut Peekable<Chars<'a>>) -> SubPattern {
    // first consume the '['
    let _ = iter.next();

    let mut look_for = |c: char| -> bool {
        if let Some(c) = iter.peek() {
            // there should be Some(c) am I right?
            let _ = iter.next();
            true
        } else {
            false
        }
    };

    // checking for inverted
    let inverted = look_for('^');

    // check for predefined set or custom set
    // [[:lower:]] or [[:alnum:]] or [[:alpha:]] are predefined notice that
    // apart from 1 variant which is `Xdigit` all are 5 character long
    // !!!!!!!!!! very dangerous line down here
    if look_for('[') {
        if look_for(':') {
            // take all the characters till :
            let mut predefined_set_name = Vec::with_capacity(6);

            while let Some(c) = iter.next_if(|&x| x.is_alphabetic() && x != ':') {
                predefined_set_name.push(c);
            }

            let set = match_name_of_set(predefined_set_name);
            let name_terminated_properly = look_for(':') && look_for(']');

            if !name_terminated_properly {
                panic!(
                    "The expression is not valid check the if the brackets where close properly"
                );
            }

            if inverted {
                SubPattern::InvertedSet(vec![Sets::PredefinedSets(set)])
            } else {
                SubPattern::BracketedSet(Sets::PredefinedSets(set))
            }
        } else {
            // this can be both Custom and Custom Range need to figure out which is which
        }
    }
}

#[inline]
fn match_name_of_set(name: Vec<char>) -> PredefinedSet {
    assert!(name.len() == 6 || name.len() == 5);
    let name = String::from_iter(name.into_iter());
    match name.as_str() {
        "alnum" => PredefinedSet::AlNum,
        "alpha" => PredefinedSet::Alpha,
        "blank" => PredefinedSet::Blank,
        "digit" => PredefinedSet::Digit,
        "graph" => PredefinedSet::Graph,
        "lower" => PredefinedSet::Lower,
        "print" => PredefinedSet::Print,
        "punct" => PredefinedSet::Punct,
        "space" => PredefinedSet::Space,
        "xdigit" => PredefinedSet::XDigit,
        _ => unreachable!("discovered some thing while matching predefined set"),
    }
}

// #[inline]
// fn

#[inline]
fn check_repetition<'a>(iter: &mut Peekable<Chars<'a>>) -> Repetition {
    match iter.peek() {
        Some('+') => {
            let _ = iter.next();
            Repetition::AtLeastOnce
        }
        Some('?') => {
            let _ = iter.next();
            Repetition::AtMostOnce
        }
        Some('*') => {
            let _ = iter.next();
            Repetition::ZeroOrMore
        }
        Some('{') => unreachable!("did not implement that"),
        _ => Repetition::None,
    }
}

#[test]
fn test_bracketed_expression1() {
    let exp = r"[[:alnum:]]";
    let ans = (
        Anchor::None,
        vec![Pattern {
            sub_pattern: SubPattern::BracketedSet(vec![Sets::PredefinedSets(PredefinedSet::AlNum)]),
            repetition: Repetition::None,
        }],
    );

    assert_eq!(process(exp), ans);
}

#[test]
fn test_bracketed_expression2() {
    let exp = r"[[:alnum:][:xdigit:]]";
    let ans = (
        Anchor::None,
        vec![Pattern {
            sub_pattern: SubPattern::BracketedSet(vec![
                Sets::PredefinedSets(PredefinedSet::AlNum),
                Sets::PredefinedSets(PredefinedSet::XDigit),
            ]),
            repetition: Repetition::None,
        }],
    );

    assert_eq!(process(exp), ans);
}
