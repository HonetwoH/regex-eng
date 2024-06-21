use core::panic;
use std::iter::Peekable;
use std::str::Chars;

use super::{
    Anchor, Expression, ParsingError, Pattern, PredefinedSet, Range, Repetition, Sets, SubPattern,
};

pub(super) fn process(line: &'_ str) -> Result<Expression, ParsingError> {
    let mut iter = line.chars().peekable();
    let mut expression: Expression = (Anchor::None, Vec::new());

    const NORMAL_CHAR: [char; 80] = [
        ' ', '!', '"', '#', '%', '&', '\'', ',', '-', '/', '0', '1', '2', '3', '4', '5', '6', '7',
        '8', '9', ':', ';', '<', '=', '>', '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
        'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '_', '`',
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

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
                repetition: check_repetition(&mut iter)?,
            };
        } else {
            pattern = match ch {
                '\\' => {
                    escaped = true;
                    continue;
                }
                '.' => Pattern {
                    sub_pattern: SubPattern::Dot,
                    repetition: check_repetition(&mut iter)?,
                },
                '[' => Pattern {
                    sub_pattern: scan_bracketed_expression(&mut iter)?,
                    repetition: check_repetition(&mut iter)?,
                },
                x if NORMAL_CHAR.binary_search(&x).is_ok() => Pattern {
                    sub_pattern: SubPattern::Char(ch),
                    repetition: check_repetition(&mut iter)?,
                },
                _ => {
                    dbg!("Lets see what is that: {:#?}", (ch, &iter));
                    return Err(ParsingError::NotAsciiCharacter);
                }
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
            _ => {
                dbg!("Some problem in anchor: {:#?}", &anchor);
                return Err(ParsingError::MisusedAnchorChracter);
            }
        };
        let _ = iter.next();
    };

    expression.0 = anchor;
    Ok(expression)
}

#[inline]
fn look_for(ch: char, iter: &mut Peekable<Chars<'_>>) -> bool {
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

#[inline]
fn get_predefined_set<'a>(iter: &mut Peekable<Chars<'_>>) -> Result<Sets, ParsingError> {
    // consuming ':'
    let _ = iter.next();

    // take all the characters till :
    let mut predefined_set_name = Vec::with_capacity(6);

    while let Some(c) = iter.next_if(|&x| x.is_alphabetic() && x != ':') {
        predefined_set_name.push(c);
    }

    let set = match_name_of_set(predefined_set_name)?;
    let name_terminated_properly = look_for(':', iter) && look_for(']', iter);

    if !name_terminated_properly {
        dbg!("The expression is not valid check the if the brackets where close properly");
        return Err(ParsingError::NotTerminatedProperly);
    }

    Ok(Sets::PredefinedSets(set))
}

#[inline] // take for example [[:punct:]A-Mm-z ]
fn scan_bracketed_expression(iter: &mut Peekable<Chars<'_>>) -> Result<SubPattern, ParsingError> {
    // checking for inverted
    let inverted = look_for('^', iter);

    // container for all sets in the bracket
    let mut sets: Vec<Sets> = Vec::new();

    while iter.peek().is_some() {
        // check for predefined set or custom set
        // [[:lower:]] or [[:alnum:]] or [[:alpha:]] are predefined notice that
        // apart from 1 variant which is `Xdigit` all are 5 character long
        // !!!!!!!!!! very dangerous line down here
        if look_for('[', iter) {
            match iter.peek() {
                Some(':') => sets.push(get_predefined_set(iter)?),

                Some('.') => {
                    todo!("collating elements");
                }

                Some('=') => {
                    todo!("open equivalence class");
                }

                _ => {
                    dbg!("This shouldn't end here: {:#?}", &iter);
                    return Err(ParsingError::UnknownGuardCharacter);
                }
            }
        } else if look_for('\\', iter) {
            if let Some(c) = iter.next() {
                match sets.last_mut() {
                    Some(Sets::Custom(x)) => {
                        x.push(c);
                    }
                    _ => sets.push(Sets::Custom(vec![c])),
                }
            } else {
                dbg!("invalid escaping: {:#?}", &iter);
                return Err(ParsingError::MalformedExpression);
            }
        } else if look_for('-', iter) {
            match sets.last_mut() {
                Some(Sets::Custom(x)) => {
                    x.push('-');
                }
                _ => sets.push(Sets::Custom(vec!['-'])),
            }
        } else {
            // this can be both Custom and Custom Range need to figure out which is which
            if let Some(maybe_lower) = iter.next() {
                // this branch check for custom range
                if look_for('-', iter) {
                    let maybe_upper = iter.next().unwrap();
                    assert!(maybe_lower < maybe_upper);
                    sets.push(Sets::CustomRange(Range(maybe_lower, maybe_upper)));
                } else {
                    // this branch check for custom set
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

        if look_for(']', iter) {
            break;
        }
    }

    if inverted {
        Ok(SubPattern::InvertedSet(sets))
    } else {
        Ok(SubPattern::BracketedSet(sets))
    }
}

fn check_alternation(iter: &mut Peekable<Chars<'_>>) {}

#[inline]
fn match_name_of_set(name: Vec<char>) -> Result<PredefinedSet, ParsingError> {
    assert!(name.len() == 6 || name.len() == 5);
    let name = String::from_iter(name);

    match name.as_str() {
        "alnum" => Ok(PredefinedSet::AlNum),
        "alpha" => Ok(PredefinedSet::Alpha),
        "blank" => Ok(PredefinedSet::Blank),
        "digit" => Ok(PredefinedSet::Digit),
        "graph" => Ok(PredefinedSet::Graph),
        "lower" => Ok(PredefinedSet::Lower),
        "print" => Ok(PredefinedSet::Print),
        "punct" => Ok(PredefinedSet::Punct),
        "space" => Ok(PredefinedSet::Space),
        "xdigit" => Ok(PredefinedSet::XDigit),
        _ => {
            dbg!("discovered some thing while matching predefined set");
            Err(ParsingError::UnknownPredefinedSetName)
        }
    }
}

#[inline]
fn check_repetition(iter: &mut Peekable<Chars<'_>>) -> Result<Repetition, ParsingError> {
    match iter.peek() {
        Some('+') => {
            let _ = iter.next();
            Ok(Repetition::AtLeastOnce)
        }
        Some('?') => {
            let _ = iter.next();
            Ok(Repetition::AtMostOnce)
        }
        Some('*') => {
            let _ = iter.next();
            Ok(Repetition::ZeroOrMore)
        }
        Some('{') => exact_repetitions(iter),
        _ => Ok(Repetition::None),
    }
}

#[inline]
fn exact_repetitions(iter: &mut Peekable<Chars<'_>>) -> Result<Repetition, ParsingError> {
    let mut number_string = [String::new(), String::new()];
    let mut current_number = 0;
    let mut is_exact = true;

    let _ = iter.next();
    for n in iter.by_ref() {
        match n {
            ',' => {
                current_number += 1;
                is_exact = false;
            }
            '}' => {
                break;
            }
            x if x.is_numeric() => {
                number_string[current_number].push(x);
            }
            _ => {
                dbg!("Some thing is stuck here: \n {:#?}", n);
                return Err(ParsingError::MalformedExpression);
            }
        }
    }

    if is_exact && current_number == 0 && number_string[1].is_empty() {
        Ok(Repetition::Exactly(
            number_string[0]
                .parse::<usize>()
                .map_err(|_| ParsingError::NotANumber)?,
        ))
    } else if current_number == 1 {
        match [number_string[0].is_empty(), number_string[1].is_empty()] {
            [true, false] => Ok(Repetition::AtMost(
                number_string[1]
                    .parse::<usize>()
                    .map_err(|_| ParsingError::NotANumber)?,
            )),

            [false, false] => Ok(Repetition::InRange(
                number_string[0]
                    .parse::<usize>()
                    .map_err(|_| ParsingError::NotANumber)?,
                number_string[1]
                    .parse::<usize>()
                    .map_err(|_| ParsingError::NotANumber)?,
            )),

            [false, true] => Ok(Repetition::AtLeast(
                number_string[0]
                    .parse::<usize>()
                    .map_err(|_| ParsingError::NotANumber)?,
            )),

            _ => {
                dbg!(
                    "Found something interesting:\n{}\n {:#?}",
                    current_number,
                    number_string
                );
                Err(ParsingError::MalformedExpression)
            }
        }
    } else {
        dbg!(
            "Found something interesting:\n{}\n {:#?}",
            current_number,
            number_string
        );
        Err(ParsingError::MalformedExpression)
    }
}

#[cfg(test)]
mod test {
    use super::process;
    use crate::parser::*;

    #[test]
    fn test_exact_repetition_1() {
        let expr = "1{25}";
        let ans = (
            Anchor::None,
            vec![Pattern {
                sub_pattern: SubPattern::Char('1'),
                repetition: Repetition::Exactly(25),
            }],
        );
        assert_eq!(ans, process(expr).unwrap());
    }

    #[test]
    fn test_exact_repetition_2() {
        let expr = "1{,25}";
        let ans = (
            Anchor::None,
            vec![Pattern {
                sub_pattern: SubPattern::Char('1'),
                repetition: Repetition::AtMost(25),
            }],
        );
        assert_eq!(ans, process(expr).unwrap());
    }

    #[test]
    fn test_exact_repetition_3() {
        let expr = "1{25,}";
        let ans = (
            Anchor::None,
            vec![Pattern {
                sub_pattern: SubPattern::Char('1'),
                repetition: Repetition::AtLeast(25),
            }],
        );
        assert_eq!(ans, process(expr).unwrap());
    }

    #[test]
    fn test_exact_repetition_4() {
        let expr = "1{2,25}";
        let ans = (
            Anchor::None,
            vec![Pattern {
                sub_pattern: SubPattern::Char('1'),
                repetition: Repetition::InRange(2, 25),
            }],
        );
        assert_eq!(ans, process(expr).unwrap());
    }

    #[test]
    fn test_bracketed_expression1() {
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

        assert_eq!(process(exp).unwrap(), ans);
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

        assert_eq!(process(exp).unwrap(), ans);
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

        assert_eq!(process(exp).unwrap(), ans);
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

        assert_eq!(process(exp).unwrap(), ans);
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

        assert_eq!(process(exp).unwrap(), ans);
    }

    #[test]
    fn test_bracketed_expression_range_simple() {
        let exp = r"[a-z]";

        let ans = (
            Anchor::None,
            vec![Pattern {
                sub_pattern: SubPattern::BracketedSet(vec![Sets::CustomRange(Range('a', 'z'))]),
                repetition: Repetition::None,
            }],
        );

        assert_eq!(process(exp).unwrap(), ans);
    }

    #[test]
    fn test_bracketed_expression_range_compound() {
        let exp = r"[a-zA-Z0-9]";

        let ans = (
            Anchor::None,
            vec![Pattern {
                sub_pattern: SubPattern::BracketedSet(vec![
                    Sets::CustomRange(Range('a', 'z')),
                    Sets::CustomRange(Range('A', 'Z')),
                    Sets::CustomRange(Range('0', '9')),
                ]),
                repetition: Repetition::None,
            }],
        );

        assert_eq!(process(exp).unwrap(), ans);
    }

    #[test]
    fn all_bracketed_expression_together() {
        let exp = r"[^0-9a-f[:space:]xX]+";

        let ans = (
            Anchor::None,
            vec![Pattern {
                sub_pattern: SubPattern::InvertedSet(vec![
                    Sets::CustomRange(Range('0', '9')),
                    Sets::CustomRange(Range('a', 'f')),
                    Sets::PredefinedSets(PredefinedSet::Space),
                    Sets::Custom(vec!['x', 'X']),
                ]),
                repetition: Repetition::AtLeastOnce,
            }],
        );

        assert_eq!(process(exp).unwrap(), ans);
    }

    #[test]
    fn test_bracketed_expression_range_compound2() {
        let exp = r"[-a-zA-Z0-9]";

        let ans = (
            Anchor::None,
            vec![Pattern {
                sub_pattern: SubPattern::BracketedSet(vec![
                    Sets::Custom(vec!['-']),
                    Sets::CustomRange(Range('a', 'z')),
                    Sets::CustomRange(Range('A', 'Z')),
                    Sets::CustomRange(Range('0', '9')),
                ]),
                repetition: Repetition::None,
            }],
        );

        assert_eq!(process(exp).unwrap(), ans);
    }

    #[test]
    fn test_bracketed_expression_range_compound3() {
        let exp = r"[a-zA-Z0-9-]";

        let ans = (
            Anchor::None,
            vec![Pattern {
                sub_pattern: SubPattern::BracketedSet(vec![
                    Sets::CustomRange(Range('a', 'z')),
                    Sets::CustomRange(Range('A', 'Z')),
                    Sets::CustomRange(Range('0', '9')),
                    Sets::Custom(vec!['-']),
                ]),
                repetition: Repetition::None,
            }],
        );

        assert_eq!(process(exp).unwrap(), ans);
    }

    #[test]
    fn test_bracketed_expression_range_compound4() {
        let exp = r"[a-eA-Z0-9ac-]";

        let ans = (
            Anchor::None,
            vec![Pattern {
                sub_pattern: SubPattern::BracketedSet(vec![
                    Sets::CustomRange(Range('a', 'e')),
                    Sets::CustomRange(Range('A', 'Z')),
                    Sets::CustomRange(Range('0', '9')),
                    Sets::Custom(vec!['a', 'c', '-']),
                ]),
                repetition: Repetition::None,
            }],
        );

        assert_eq!(process(exp).unwrap(), ans);
    }

    #[test]
    fn test_bracketed_expression_escaping1() {
        let exp = r"[a-eA-Z0-9\\ac-]";

        let ans = (
            Anchor::None,
            vec![Pattern {
                sub_pattern: SubPattern::BracketedSet(vec![
                    Sets::CustomRange(Range('a', 'e')),
                    Sets::CustomRange(Range('A', 'Z')),
                    Sets::CustomRange(Range('0', '9')),
                    Sets::Custom(vec!['\\', 'a', 'c', '-']),
                ]),
                repetition: Repetition::None,
            }],
        );

        assert_eq!(process(exp).unwrap(), ans);
    }

    #[test]
    fn test_bracketed_expression_escaping2() {
        let exp = r"[a-eA-Z0-9\]ac-]";

        let ans = (
            Anchor::None,
            vec![Pattern {
                sub_pattern: SubPattern::BracketedSet(vec![
                    Sets::CustomRange(Range('a', 'e')),
                    Sets::CustomRange(Range('A', 'Z')),
                    Sets::CustomRange(Range('0', '9')),
                    Sets::Custom(vec![']', 'a', 'c', '-']),
                ]),
                repetition: Repetition::None,
            }],
        );

        assert_eq!(process(exp).unwrap(), ans);
    }
}
