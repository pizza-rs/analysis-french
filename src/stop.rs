//! French stop words (from Lucene/Snowball project).

use alloc::borrow::Cow;
use alloc::vec::Vec;
use hashbrown::HashSet;
use once_cell::sync::Lazy;
use pizza_engine::analysis::{Token, TokenFilter};

/// Default French stop words sourced from Apache Lucene.
static DEFAULT_STOP_WORDS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let words: &[&str] = &[
    "ai",
    "aie",
    "aient",
    "aies",
    "ait",
    "au",
    "aurai",
    "auraient",
    "aurais",
    "aurait",
    "aurez",
    "auriez",
    "aurions",
    "aurons",
    "auront",
    "aux",
    "avaient",
    "avais",
    "avait",
    "avec",
    "avez",
    "aviez",
    "avons",
    "ayant",
    "ayez",
    "ayons",
    "c",
    "ce",
    "ceci",
    "cela",
    "celà",
    "ces",
    "cet",
    "cette",
    "d",
    "dans",
    "de",
    "des",
    "du",
    "elle",
    "en",
    "es",
    "et",
    "eu",
    "eue",
    "eues",
    "eurent",
    "eus",
    "eusse",
    "eussent",
    "eusses",
    "eussiez",
    "eussions",
    "eut",
    "eux",
    "eûmes",
    "eût",
    "eûtes",
    "furent",
    "fus",
    "fusse",
    "fussent",
    "fusses",
    "fussiez",
    "fussions",
    "fut",
    "fûmes",
    "fûtes",
    "ici",
    "il",
    "ils",
    "j",
    "je",
    "l",
    "la",
    "le",
    "les",
    "leur",
    "leurs",
    "lui",
    "m",
    "ma",
    "mais",
    "me",
    "mes",
    "moi",
    "mon",
    "même",
    "n",
    "ne",
    "nos",
    "notre",
    "nous",
    "on",
    "ont",
    "ou",
    "par",
    "pas",
    "pour",
    "qu",
    "que",
    "quel",
    "quelle",
    "quelles",
    "quels",
    "qui",
    "s",
    "sa",
    "sans",
    "se",
    "sera",
    "serai",
    "seraient",
    "serais",
    "serait",
    "seras",
    "serez",
    "seriez",
    "serions",
    "serons",
    "seront",
    "ses",
    "soi",
    "soient",
    "sois",
    "soit",
    "sont",
    "soyez",
    "soyons",
    "suis",
    "sur",
    "t",
    "ta",
    "te",
    "tes",
    "toi",
    "ton",
    "tu",
    "un",
    "une",
    "vos",
    "votre",
    "vous",
    "y",
    "à",
    "étaient",
    "étais",
    "était",
    "étant",
    "étiez",
    "étions",
    "étée",
    "étées",
    "êtes",
    ];
    words.iter().copied().collect()
});

/// Removes French stop words from the token stream.
#[derive(Clone, Debug)]
pub struct FrenchStopFilter {
    stop_words: HashSet<String>,
}

impl Default for FrenchStopFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl FrenchStopFilter {
    pub fn new() -> Self {
        Self {
            stop_words: DEFAULT_STOP_WORDS.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn with_words(words: &[&str]) -> Self {
        Self {
            stop_words: words.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl TokenFilter for FrenchStopFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let term = token.term.as_ref();
        if self.stop_words.contains(term) {
            return (true, None);
        }
        (false, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stop_word_count() {
        assert!(DEFAULT_STOP_WORDS.len() >= 154);
    }

    #[test]
    fn test_filters_stop_word() {
        let f = FrenchStopFilter::new();
        let word = DEFAULT_STOP_WORDS.iter().next().unwrap();
        let mut token = Token::new(word, 0, word.len() as u32, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted);
    }

    #[test]
    fn test_passes_non_stop_word() {
        let f = FrenchStopFilter::new();
        let mut token = Token::new("xyzzy_not_a_stop_word", 0, 21, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(!deleted);
    }

    #[test]
    fn test_custom_words() {
        let f = FrenchStopFilter::with_words(&["custom", "words"]);
        let mut token = Token::new("custom", 0, 6, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted);
    }
}
