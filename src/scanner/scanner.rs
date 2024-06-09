use core::panic;
use std::iter::Peekable;
use std::str::Chars;

use super::{Anchor, Expression, Pattern, PredefinedSet, Range, Repetition, Sets, SubPattern};

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
            escaped = false;
            pattern = Pattern {
                sub_pattern: SubPattern::Char(ch),
                repetition: check_repetition(&mut iter),
            };
        } else {
            pattern = match ch {
                '\\' => {
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

#[inline]
fn look_for<'a>(ch: char, iter: &mut Peekable<Chars<'a>>) -> bool {
    if let Some(temp) = iter.peek() {
        if *temp == ch {
            let _ = iter.next();
            true
        } else {
            false
        }
    } else {
        panic!("The regex ended here, it is invalid");
    }
}

#[inline] // take for example [[:punct:]A-Mm-z ]
fn scan_bracketed_expression<'a>(iter: &mut Peekable<Chars<'a>>) -> SubPattern {
    // checking for inverted
    let inverted = look_for('^', iter);

    // container for all sets in the bracket
    let mut sets: Vec<Sets> = Vec::new();

    while let Some(_) = iter.peek() {
        // check for predefined set or custom set
        // [[:lower:]] or [[:alnum:]] or [[:alpha:]] are predefined notice that
        // apart from 1 variant which is `Xdigit` all are 5 character long
        // !!!!!!!!!! very dangerous line down here
        if look_for('[', iter) {
            if look_for(':', iter) {
                // take all the characters till :
                let mut predefined_set_name = Vec::with_capacity(6);

                while let Some(c) = iter.next_if(|&x| x.is_alphabetic() && x != ':') {
                    predefined_set_name.push(c);
                }

                let set = match_name_of_set(predefined_set_name);
                let name_terminated_properly = look_for(':', iter) && look_for(']', iter);

                if !name_terminated_properly {
                    panic!(
                    "The expression is not valid check the if the brackets where close properly"
                    );
                }

                sets.push(Sets::PredefinedSets(set));
            }
        } else {
            // panic!("Reached here");
            // this can be both Custom and Custom Range need to figure out which is which
            if let Some(maybe_lower) = iter.next() {
                if look_for('-', iter) {
                    let maybe_upper = iter.next().unwrap();
                    sets.push(Sets::CustomRange(Range {
                        start: maybe_lower,
                        end: maybe_upper,
                    }));
                } else {
                    if let Some(a_char) = iter.next_if(|&x| x != ']') {
                        match sets.last_mut() {
                            Some(Sets::Custom(x)) => {
                                x.push(maybe_lower);
                                x.push(a_char);
                            }
                            _ => sets.push(Sets::Custom(vec![maybe_lower, a_char])),
                        }
                    } else {
                        match sets.last_mut() {
                            Some(Sets::Custom(x)) => {
                                x.push(maybe_lower);
                            }
                            _ => sets.push(Sets::Custom(vec![maybe_lower])),
                        }
                    }
                }
            }
        }
        //TODO: can this be removed ?
        if let Some(x) = iter.peek() {
            if *x == ']' {
                let _ = iter.next();
                break;
            }
        }
        // if look_for(']', iter) {
        //     break;
        // }
    }

    if inverted {
        SubPattern::InvertedSet(sets)
    } else {
        SubPattern::BracketedSet(sets)
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

#[cfg(test)]
mod test {
    use super::process;
    use crate::scanner::*;

    #[test]
    fn test_bracketed_expression1() {
        println!("in test 1");
        let exp = r"[[:alnum:]]";
        let ans = (
            Anchor::None,
            vec![Pattern {
                sub_pattern: SubPattern::BracketedSet(vec![Sets::PredefinedSets(
                    PredefinedSet::AlNum,
                )]),
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

    #[test]
    fn test_inverted_bracketed_expression2() {
        let exp = r"[^[:alnum:][:xdigit:]]";
        let ans = (
            Anchor::None,
            vec![Pattern {
                sub_pattern: SubPattern::InvertedSet(vec![
                    Sets::PredefinedSets(PredefinedSet::AlNum),
                    Sets::PredefinedSets(PredefinedSet::XDigit),
                ]),
                repetition: Repetition::None,
            }],
        );

        assert_eq!(process(exp), ans);
    }
    #[test]
    fn test_inverted_bracketed_expression3() {
        let exp = r"[^[:alnum:][:xdigit:][:punct:]]";
        let ans = (
            Anchor::None,
            vec![Pattern {
                sub_pattern: SubPattern::InvertedSet(vec![
                    Sets::PredefinedSets(PredefinedSet::AlNum),
                    Sets::PredefinedSets(PredefinedSet::XDigit),
                    Sets::PredefinedSets(PredefinedSet::Punct),
                ]),
                repetition: Repetition::None,
            }],
        );

        assert_eq!(process(exp), ans);
    }
    #[test]
    fn test_bracketed_expression3() {
        let exp = r"[aBc09]";
        let ans = (
            Anchor::None,
            vec![Pattern {
                sub_pattern: SubPattern::BracketedSet(vec![Sets::Custom(vec![
                    'a', 'B', 'c', '0', '9',
                ])]),
                repetition: Repetition::None,
            }],
        );
        assert_eq!(process(exp), ans);
    }

    #[test]
    fn test_bracketed_expression_range_simple() {
        let exp = r"[a-z]";
        let ans = (
            Anchor::None,
            vec![Pattern {
                sub_pattern: SubPattern::BracketedSet(vec![Sets::CustomRange(Range {
                    start: 'a',
                    end: 'z',
                })]),
                repetition: Repetition::None,
            }],
        );
        assert_eq!(process(exp), ans);
    }

    #[test]
    fn test_bracketed_expression_range_compound() {
        let exp = r"[a-zA-Z0-9]";
        let ans = (
            Anchor::None,
            vec![Pattern {
                sub_pattern: SubPattern::BracketedSet(vec![
                    Sets::CustomRange(Range {
                        start: 'a',
                        end: 'z',
                    }),
                    Sets::CustomRange(Range {
                        start: 'A',
                        end: 'Z',
                    }),
                    Sets::CustomRange(Range {
                        start: '0',
                        end: '9',
                    }),
                ]),
                repetition: Repetition::None,
            }],
        );
        assert_eq!(process(exp), ans);
    }
    #[test]
    fn all_bracketed_expression_together() {
        let exp = r"[^0-9a-f[:space:]xX]+";
        let ans = (
            Anchor::None,
            vec![Pattern {
                sub_pattern: SubPattern::InvertedSet(vec![
                    Sets::CustomRange(Range {
                        start: '0',
                        end: '9',
                    }),
                    Sets::CustomRange(Range {
                        start: 'a',
                        end: 'f',
                    }),
                    Sets::PredefinedSets(PredefinedSet::Space),
                    Sets::Custom(vec!['x', 'X']),
                ]),
                repetition: Repetition::AtLeastOnce,
            }],
        );
        assert_eq!(process(exp), ans);
    }
}
