use std::iter::Peekable;
use std::str::Chars;

use super::{Anchor, Expression, Pattern, Repetition, SubPattern};

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
        Some('{') => exact_repetitions(iter),
        _ => Repetition::None,
    }
}

#[inline]
fn exact_repetitions<'a>(iter: &mut Peekable<Chars<'a>>) -> Repetition {
    let mut number_string = [String::new(), String::new()];
    let mut current_number = 0;
    let mut is_exact = true;

    let _ = iter.next();
    while let Some(n) = iter.next() {
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
            _ => unreachable!("Some thing is stuck here: \n {:#?}", n),
        }
    }

    if is_exact && current_number == 0 && number_string[1].is_empty() {
        Repetition::Exactly(number_string[0].parse::<usize>().unwrap())
    } else if current_number == 1 {
        match [number_string[0].is_empty(), number_string[1].is_empty()] {
            [true, false] => Repetition::AtMost(number_string[1].parse::<usize>().unwrap()),

            [false, false] => Repetition::InRange(
                number_string[0].parse::<usize>().unwrap(),
                number_string[1].parse::<usize>().unwrap(),
            ),

            [false, true] => Repetition::AtLeast(number_string[0].parse::<usize>().unwrap()),

            _ => {
                panic!(
                    "Found something interesting:\n{}\n {:#?}",
                    current_number, number_string
                );
            }
        }
    } else {
        panic!(
            "Found something interesting:\n{}\n {:#?}",
            current_number, number_string
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(ans, process(expr));
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
        assert_eq!(ans, process(expr));
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
        assert_eq!(ans, process(expr));
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
        assert_eq!(ans, process(expr));
    }
}
