// Scanner will try to make ast of the regex passed after validating it.

//TODO: Expression should be a link list of the following types but try brainstorm it but there will be problems

use std::default;

type Expression = Vec<Pattern> ;

#[derive(Debug, Default,PartialEq, Eq)]
struct Pattern {
    sub_pattern: SubPattern,
    repetition: Option<Repetition>,
}

// SubPattern together form Pattern
#[derive(Debug, Default, PartialEq, Eq)]
enum SubPattern {
    #[default]
    Dot,
    Char(char),
    InvertedChar(char),
    BracketedSet(Sets),
    InvertedSet(Sets),
}

#[derive(Debug, Default,PartialEq, Eq)]
enum Sets {
    #[default]
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
#[derive(Debug, Default,PartialEq, Eq)]
struct Range {
    // both start and end are inclusive
    start: char,
    end: char,
}

//TODO: need to figure out how to include Anchor in all this
enum Anchor {
    Start,
    End,
}

#[derive(Debug, Default,PartialEq, Eq)]
enum Repetition {
    AtMostOnce,  // ?
    AtLeastOnce, // +
    #[default]
    ZeroOrMore, // *     //TODO: suggest a better name
    Exactly(usize), // {n}
    AtLeast(usize), // {n,}
    AtMost(usize), // {,m}
    Between(usize, usize), // {n,m} // TODO: suggest a better name
}

#[test]
fn t1() {
    use std::str::Chars;
    use  std::iter::Peekable;

    let exp = "s.+e";
    let ans = vec![
        Pattern{
            sub_pattern: SubPattern::Char('s'),
            repetition: None,
        },
        Pattern{
            sub_pattern: SubPattern::Dot,
            repetition: Some(Repetition::AtLeastOnce),
        },
        Pattern{
            sub_pattern: SubPattern::Char('e'),
            repetition: None,
        },
    ];

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

        let mut expression: Expression = Vec::new();
        while let Some(ch) = iter.next() {
            let pattern = match ch {
                '.' => 
                    Pattern {
                      sub_pattern: SubPattern::Dot,
                      repetition: check_repetition(&mut iter),
                    }
                ,            
                // need to include other ascii char too later
                'a'..='z' | '0'..='9' | 'A'..='Z' => 
                    Pattern {
                      sub_pattern: SubPattern::Char(ch),
                      repetition: check_repetition(&mut iter),
                    }
                ,
                _ => unreachable!("Lets see what is that: {:#?}", (ch, iter))
            };
            expression.push(pattern);
        }
        expression
    }

    assert_eq!(process(exp), ans);
}
