// Scanner will try to make ast of the regex passed after validating it.

pub(crate) type Expression = (Anchor, Vec<Pattern>);

//TODO: need to to add other context, this is not helpful in current state
#[derive(Debug)]
pub(crate) enum ParsingError {
    NotAsciiCharacter,
    MisusedAnchorChracter,
    NotTerminatedProperly,
    UnknownGuardCharacter,
    MalformedExpression,
    UnknownPredefinedSetName,
    NotANumber,
    IncorrectRepetitionLimits,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Pattern {
    pub(crate) sub_pattern: SubPattern,
    pub(crate) repetition: Repetition,
}

// SubPattern together form Pattern
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum SubPattern {
    Dot,
    Char(char),
    // InvertedChar(char), //TODO: check if this is correct according to spec
    BracketedSet(Vec<Sets>),
    InvertedSet(Vec<Sets>),
    Alternative(Vec<Vec<Pattern>>),
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Sets {
    PredefinedSets(PredefinedSet),
    CustomRange(Range),
    Custom(Vec<char>),
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum PredefinedSet {
    AlNum, // that is the name used in `info grep`
    Alpha,
    Blank, // TODO: how is that different from the space
    // Cntrl, // Control Sequence, will not implement
    Digit,
    Graph, // Graphical Cluster which is intersection of [:alnum:] and [:punct:]
    Lower,
    Upper,
    Print,  // Printable character [:alnum:] [:punct:] and space character
    Punct,  // Puntuation
    Space,  // TODO: how is that different from the blank
    XDigit, // Hexa Decimal
}

// The custom range will be like this [0-5] [4-9]
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Range(pub(crate) char, pub(crate) char);

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Anchor {
    Start,
    End,
    Both,
    None,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Repetition {
    AtMostOnce,            // ?
    AtLeastOnce,           // +
    ZeroOrMore,            // *
    Exactly(usize),        // {n}
    AtLeast(usize),        // {n,}
    AtMost(usize),         // {,m}
    InRange(usize, usize), // {n,m}
    None,
}

// impl Display for ParsingError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         writeln!(
//             f,
//             {
//                 use ParsingError::*;
//                 match self {
//                     NotAsciiCharacter,
//                     MisusedAnchorChracter,
//                     NotTerminatedProperly,
//                     UnknownGuardCharacter,
//                     MalformedExpression,
//                     UnknownPredefinedSetName,
//                     NotANumber,
//                     IncorrectRepetitionLimits,
//                 }
//             }
//         )
//     }
// }

mod parse;

pub(crate) fn process(line: &'_ str) -> Result<Expression, ParsingError> {
    parse::process(line)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn testing_anchor_and_repetation() {
        let exp = r"^s.+e\+$";
        let ans = (
            Anchor::Both,
            vec![
                Pattern {
                    sub_pattern: SubPattern::Char('s'),
                    repetition: Repetition::None,
                },
                Pattern {
                    sub_pattern: SubPattern::Dot,
                    repetition: Repetition::AtLeastOnce,
                },
                Pattern {
                    sub_pattern: SubPattern::Char('e'),
                    repetition: Repetition::None,
                },
                Pattern {
                    sub_pattern: SubPattern::Char('+'),
                    repetition: Repetition::None,
                },
            ],
        );

        assert_eq!(process(exp).unwrap(), ans);
    }

    #[test]
    fn testing_escape_sequence_and_anchor() {
        let exp = r"hel\++\**lo?";
        let ans = (
            Anchor::None,
            vec![
                Pattern {
                    sub_pattern: SubPattern::Char('h'),
                    repetition: Repetition::None,
                },
                Pattern {
                    sub_pattern: SubPattern::Char('e'),
                    repetition: Repetition::None,
                },
                Pattern {
                    sub_pattern: SubPattern::Char('l'),
                    repetition: Repetition::None,
                },
                Pattern {
                    sub_pattern: SubPattern::Char('+'),
                    repetition: Repetition::AtLeastOnce,
                },
                Pattern {
                    sub_pattern: SubPattern::Char('*'),
                    repetition: Repetition::ZeroOrMore,
                },
                Pattern {
                    sub_pattern: SubPattern::Char('l'),
                    repetition: Repetition::None,
                },
                Pattern {
                    sub_pattern: SubPattern::Char('o'),
                    repetition: Repetition::AtMostOnce,
                },
            ],
        );
        assert_eq!(process(exp).unwrap(), ans);
    }

    #[test]
    fn parsing_email() {
        let exp = r"[a-z]+@[a-z]+\.[a-z]{2,8}";
        let ans = (
            Anchor::None,
            vec![
                Pattern {
                    sub_pattern: SubPattern::BracketedSet(vec![Sets::CustomRange(Range('a', 'z'))]),
                    repetition: Repetition::AtLeastOnce,
                },
                Pattern {
                    sub_pattern: SubPattern::Char('@'),
                    repetition: Repetition::None,
                },
                Pattern {
                    sub_pattern: SubPattern::BracketedSet(vec![Sets::CustomRange(Range('a', 'z'))]),
                    repetition: Repetition::AtLeastOnce,
                },
                Pattern {
                    sub_pattern: SubPattern::Char('.'),
                    repetition: Repetition::None,
                },
                Pattern {
                    sub_pattern: SubPattern::BracketedSet(vec![Sets::CustomRange(Range('a', 'z'))]),
                    repetition: Repetition::InRange(2, 8),
                },
            ],
        );

        assert_eq!(process(exp).unwrap(), ans);
    }

    #[test]
    fn alternations() {
        let exp = r"(cat|dog)*";
        let ans = (
            Anchor::None,
            vec![Pattern {
                sub_pattern: SubPattern::Alternative(vec![
                    vec![
                        Pattern {
                            sub_pattern: SubPattern::Char('c'),
                            repetition: Repetition::None,
                        },
                        Pattern {
                            sub_pattern: SubPattern::Char('a'),
                            repetition: Repetition::None,
                        },
                        Pattern {
                            sub_pattern: SubPattern::Char('t'),
                            repetition: Repetition::None,
                        },
                    ],
                    vec![
                        Pattern {
                            sub_pattern: SubPattern::Char('d'),
                            repetition: Repetition::None,
                        },
                        Pattern {
                            sub_pattern: SubPattern::Char('o'),
                            repetition: Repetition::None,
                        },
                        Pattern {
                            sub_pattern: SubPattern::Char('g'),
                            repetition: Repetition::None,
                        },
                    ],
                ]),
                repetition: Repetition::ZeroOrMore,
            }],
        );
        assert_eq!(process(exp).unwrap(), ans);
    }
}
