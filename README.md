# regex-eng

A small basic regular expression engine.

## Spec (source grep man page)
```
       A  regular  expression  is  a  pattern  that  describes  a  set  of strings.  Regular expressions are constructed analogously to arithmetic expressions, by using various operators to combine smaller expressions.

       grep understands three different versions of regular expression syntax: “basic” (BRE), “extended” (ERE) and “perl”  (PCRE).   In GNU grep, basic and extended regular expressions are merely different notations for the same pattern-matching functionality.  In other implementations, basic regular expressions are ordinarily less powerful than extended, though occasionally it is the other way  around.   The  following description applies to extended regular expressions; differences for basic regular expressions are summarized afterwards.  Perl-compatible regular expressions have different functionality, and are documented  in  pcre2syntax(3) and pcre2pattern(3), but work only if PCRE support is enabled.

       The  fundamental  building  blocks  are  the  regular expressions that match a single character.  Most characters, including all letters and digits, are regular expressions that match themselves.  Any meta-character with special meaning  may  be  quoted  by preceding it with a backslash.

       The period . matches any single character.  It is unspecified whether it matches an encoding error.

   Character Classes and Bracket Expressions
       A  bracket  expression is a list of characters enclosed by [ and ].  It matches any single character in that list.  If the first character of the list is the caret ^ then it matches any character not in the list; it is  unspecified  whether  it  matches  an encoding error.  For example, the regular expression [0123456789] matches any single digit.

       Within  a  bracket  expression,  a  range  expression  consists  of two characters separated by a hyphen.  It matches any single character that sorts between the two characters, inclusive, using the  locale's  collating  sequence  and  character  set.   For example, in the default C locale, [a-d] is equivalent to [abcd].  Many locales sort characters in dictionary order, and in these locales  [a-d]  is  typically  not  equivalent  to  [abcd];  it  might  be  equivalent to [aBbCcDd], for example.  To obtain the traditional interpretation of bracket expressions, you can use the C locale by setting the LC_ALL environment  variable  to  the value C.

       Finally,  certain  named  classes  of  characters  are  predefined within bracket expressions, as follows.  Their names are self explanatory, and they are [:alnum:], [:alpha:], [:blank:], [:cntrl:], [:digit:],  [:graph:],  [:lower:],  [:print:],  [:punct:], [:space:],  [:upper:], and [:xdigit:].  For example, [[:alnum:]] means the character class of numbers and letters in the current locale.  In the C locale and ASCII character set encoding, this is the same as [0-9A-Za-z].  (Note that the  brackets  in  these class names are part of the symbolic names, and must be included in addition to the brackets delimiting the bracket expression.) Most  meta-characters lose their special meaning inside bracket expressions.  To include a literal ] place it first in the list. Similarly, to include a literal ^ place it anywhere but first.  Finally, to include a literal - place it last.

   Anchoring
       The caret ^ and the dollar sign $ are meta-characters that respectively match the empty string at the beginning  and  end  of  a line.

   The Backslash Character and Special Expressions
       The  symbols  \< and \> respectively match the empty string at the beginning and end of a word.  The symbol \b matches the empty string at the edge of a word, and \B matches the empty string provided it's not at the edge of a  word.   The  symbol  \w  is  a synonym for [_[:alnum:]] and \W is a synonym for [^_[:alnum:]]. Repetition

       A regular expression may be followed by one of several repetition operators:
       ?      The preceding item is optional and matched at most once.
       *      The preceding item will be matched zero or more times.
       +      The preceding item will be matched one or more times.
       {n}    The preceding item is matched exactly n times.
       {n,}   The preceding item is matched n or more times.
       {,m}   The preceding item is matched at most m times.  This is a GNU extension.
       {n,m}  The preceding item is matched at least n times, but not more than m times.

   Concatenation
       Two  regular  expressions  may  be concatenated; the resulting regular expression matches any string formed by concatenating two substrings that respectively match the concatenated expressions.

   Alternation
       Two regular expressions may be joined by the infix operator |; the resulting regular  expression  matches  any  string  matching either alternate expression.

   Precedence
       Repetition  takes  precedence  over  concatenation,  which in turn takes precedence over alternation.  A whole expression may be enclosed in parentheses to override these precedence rules and form a subexpression.

   Back-references and Subexpressions
       The back-reference \n, where n  is  a  single  digit,  matches  the  substring  previously  matched  by  the  nth  parenthesized subexpression of the regular expression.

   Basic vs Extended Regular Expressions
       In  basic  regular  expressions the meta-characters ?, +, {, |, (, and ) lose their special meaning; instead use the backslashed versions \?, \+, \{, \|, \(, and \).
```

### Workflow for the project 

+ `Always code as if the guy who ends up maintaining your code will be violent psychopath who knows where you live` - *John F. Woods*
+ *Dont write code which you cannot document* (Documentation Driven Development) and *test* is considered integral part of documentation.
+ Always try to make funcitons *pure* 
+ Try not to take in dependencies

