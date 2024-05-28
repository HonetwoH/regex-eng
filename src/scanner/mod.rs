// Scanner will try to make ast of the regex passed after validating it.

//TODO: Expression should be a link list of the following types but try brainstorm it but there will be problems

struct Pattern {
    subpattern: (SubPattern, Option<Repetition>),
    next: Box<Pattern>,
}

// SubPattern together form Pattern
enum SubPattern {
    Dot,
    Char(char),
    InvertedChar(char),
    BracketedSet(Sets),
    InvertedSet(Sets),
}

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

// The custom range will be like this [0-9A-Za-z]
struct Range {
    // both start and end are inclusive
    start: char,
    end: char,
}

//TODO: need to figure out how to include Anchor in all this
enum Anchor {
    Start(SubPattern),
    End(SubPattern),
}

enum Repetition {
    AtmostOnce,            // ?
    AtleastOnce,           // +
    ZeroOrMore,            // *     //TODO: suggest a better name
    Exactly(usize),        // {n}
    Atleast(usize),        // {n,}
    Atmost(usize),         // {,m}
    Between(usize, usize), // {n,m} // TODO: suggest a better name
}
