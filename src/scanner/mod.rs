// Scanner will try to make ast of the regex passed after validating it.

type Expression = (Anchor, Vec<Pattern>);

#[derive(Debug, PartialEq, Eq)]
struct Pattern {
    sub_pattern: SubPattern,
    repetition: Repetition,
}

// SubPattern together form Pattern
#[derive(Debug, PartialEq, Eq)]
enum SubPattern {
    Dot,
    Char(char),
    InvertedChar(char),
    BracketedSet(Sets),
    InvertedSet(Sets),
}

#[derive(Debug, PartialEq, Eq)]
enum Sets {
    AlphaNum,
    Alpha,
    Blank, // TODO: how is that different from the space
    // Cntrl,// Control Sequence, will not implement
    Digit,
    // Graph // Grapheme Cluster, will not implement for
    Lower,
    Upper,
    Print,
    Punct,  // Puntuation
    Space,  // TODO: how is that different from the blank
    XDigit, // What is that

    Custom(Range),
}

// The custom range will be like this [0-5] [4-9]
#[derive(Debug, PartialEq, Eq)]
struct Range {
    // both start and end are inclusive
    start: char,
    end: char,
}

#[derive(Debug, PartialEq, Eq)]
enum Anchor {
    Start,
    End,
    Both,
    None,
}

#[derive(Debug, PartialEq, Eq)]
enum Repetition {
    AtMostOnce,            // ?
    AtLeastOnce,           // +
    ZeroOrMore,            // *     //TODO: suggest a better name
    Exactly(usize),        // {n}
    AtLeast(usize),        // {n,}
    AtMost(usize),         // {,m}
    Between(usize, usize), // {n,m} // TODO: suggest a better name
    None,
}

mod scanner;

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

    assert_eq!(scanner::process(exp), ans);
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
    assert_eq!(scanner::process(exp), ans);
}
