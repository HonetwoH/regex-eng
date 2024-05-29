// Scanner will try to make ast of the regex passed after validating it.

//TODO: Expression should be a link list of the following types but try brainstorm it but there will be problems

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
    // InvertedChar(char), //TODO: check if this is correct according to spec
    BracketedSet(Vec<Sets>),
    InvertedSet(Vec<Sets>),
}

#[derive(Debug, PartialEq, Eq)]
enum Sets {
    PredefinedSets(PredefinedSet),
    CustomRange(Range),
    Custom(Vec<char>),
}
#[derive(Debug, PartialEq, Eq)]
enum PredefinedSet {
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

// The custom range will be like this [0-9A-Za-z]
#[derive(Debug, PartialEq, Eq)]
struct Range {
    // both start and end are inclusive
    start: char,
    end: char,
}

//TODO: need to figure out how to include Anchor in all this
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
fn t1() {
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
fn t2() {
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
