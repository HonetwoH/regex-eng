// Scanner will try to make ast of the regex passed after validating it.

//TODO: Expression should be a link list of the following types but try brainstorm it but the
//TODO: problem there will be problems

enum Sets {
    Dot, // The special wildcard

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

enum Anchor {
    Start(Sets),
    End(Sets),
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
