// Scanner will try to make ast of the regex passed after validating it.

//TODO: Expression should be a link list of the following types but try brainstorm it but there will be problems

type Expression = (Option<Anchor>, Vec<Pattern>);

#[derive(Debug, PartialEq, Eq)]
struct Pattern {
    sub_pattern: SubPattern,
    repetition: Option<Repetition>,
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
}

#[test]
fn t1() {
    use std::iter::Peekable;
    use std::str::Chars;

    let exp = "^s.+e$";
    let ans = (
        Some(Anchor::Both),
        vec![
            Pattern {
                sub_pattern: SubPattern::Char('s'),
                repetition: None,
            },
            Pattern {
                sub_pattern: SubPattern::Dot,
                repetition: Some(Repetition::AtLeastOnce),
            },
            Pattern {
                sub_pattern: SubPattern::Char('e'),
                repetition: None,
            },
        ],
    );

    fn process<'a>(line: &'a str) -> Expression {
        // cannot trim the line
        let mut iter = line.chars().peekable();
        let check_repetition = |iter: &mut Peekable<Chars<'a>>| match iter.peek() {
            Some('+') => {
                let _ = iter.next();
                Some(Repetition::AtLeastOnce)
            }
            Some('?') => {
                let _ = iter.next();
                Some(Repetition::AtMostOnce)
            }
            Some('*') => {
                let _ = iter.next();
                Some(Repetition::ZeroOrMore)
            }
            Some('{') => unreachable!("did not implement that"),
            _ => None,
        };

        let mut anchor: Option<Anchor> = None;
        let mut expression: Expression = (None, Vec::new());
        if let Some('^') = iter.next() {
            anchor = Some(Anchor::Start);
        };
        // this will not work for escape sequences
        while let Some(ch) = iter.next_if(|&x| x != '$') {
            let pattern = match ch {
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
            expression.1.push(pattern);
        }
        if let Some('$') = iter.next() {
            match anchor {
                Some(Anchor::Start) => {
                    anchor = Some(Anchor::Both);
                }
                None => {
                    anchor = Some(Anchor::End);
                }
                _ => unreachable!("Some problem in anchor: {:#?}", anchor),
            };
        };

        expression.0 = anchor;
        expression
    }

    assert_eq!(process(exp), ans);
}
