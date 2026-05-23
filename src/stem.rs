//! French stemmers (light and minimal).

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use pizza_engine::analysis::{Token, TokenFilter};

/// French light stemmer — removes plural, feminine, and adverbial suffixes.
#[derive(Clone, Debug, Default)]
pub struct FrenchLightStemFilter;

impl FrenchLightStemFilter {
    pub fn new() -> Self {
        Self
    }
}

impl TokenFilter for FrenchLightStemFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let text = token.term.as_ref();
        if text.len() < 6 {
            return (false, None);
        }
        let stemmed = stem_french_light(text);
        if stemmed != text {
            token.term = Cow::Owned(stemmed);
        }
        (false, None)
    }
}

fn stem_french_light(word: &str) -> String {
    let mut result = String::from(word);

    // Remove trailing 's' or 'x' (plural markers)
    if result.ends_with('s') || result.ends_with('x') {
        result.pop();
    }

    let len = result.len();
    if len < 5 {
        return result;
    }

    if result.ends_with("ment") && len > 7 {
        result.truncate(len - 4);
        return result;
    }

    if result.ends_with("eux") {
        result.truncate(result.len() - 1);
        return result;
    }

    if result.ends_with("euse") || result.ends_with("ière") {
        result.truncate(result.len() - 4);
        return result;
    }

    if result.ends_with("ère") || result.ends_with("eur") {
        result.truncate(result.len() - 3);
        return result;
    }

    if result.ends_with("ive") {
        result.truncate(result.len() - 3);
        result.push_str("if");
        return result;
    }

    if result.ends_with("aux") {
        result.truncate(result.len() - 3);
        result.push_str("al");
        return result;
    }

    if result.ends_with("ée") || result.ends_with("ie") {
        result.truncate(result.len() - 2);
        return result;
    }

    if result.ends_with('é') || result.ends_with('è') {
        result.pop();
        return result;
    }

    norm_french(&mut result);
    result
}

/// Normalize accents throughout the word (Lucene's norm() function).
fn norm_french(s: &mut String) {
    let mut changed = false;
    let normalized: String = s.chars().map(|c| match c {
        'à' | 'â' => { changed = true; 'a' }
        'ç' => { changed = true; 'c' }
        'è' | 'é' | 'ê' | 'ë' => { changed = true; 'e' }
        'î' | 'ï' => { changed = true; 'i' }
        'ô' => { changed = true; 'o' }
        'ù' | 'û' | 'ü' => { changed = true; 'u' }
        _ => c,
    }).collect();
    if changed {
        *s = normalized;
    }
}

/// French minimal stemmer — only removes plural/gender markers.
#[derive(Clone, Debug, Default)]
pub struct FrenchMinimalStemFilter;

impl FrenchMinimalStemFilter {
    pub fn new() -> Self {
        Self
    }
}

impl TokenFilter for FrenchMinimalStemFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let text = token.term.as_ref();
        if text.len() < 4 {
            return (false, None);
        }
        let stemmed = stem_french_minimal(text);
        if stemmed != text {
            token.term = Cow::Owned(stemmed);
        }
        (false, None)
    }
}

fn stem_french_minimal(word: &str) -> String {
    let mut result = String::from(word);

    if result.ends_with("aux") {
        result.truncate(result.len() - 3);
        result.push_str("al");
        return result;
    }

    if result.ends_with('s') || result.ends_with('x') {
        result.pop();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_french_light_plural() {
        let f = FrenchLightStemFilter::new();
        let mut token = Token::new("chateaux", 0, 8, 0);
        f.filter(&mut token);
        assert_eq!(token.term, "chateau");
    }

    #[test]
    fn test_french_light_feminine() {
        let f = FrenchLightStemFilter::new();
        let mut token = Token::new("sportive", 0, 8, 0);
        f.filter(&mut token);
        assert_eq!(token.term, "sportif");
    }

    #[test]
    fn test_french_minimal_aux() {
        let f = FrenchMinimalStemFilter::new();
        let mut token = Token::new("chevaux", 0, 7, 0);
        f.filter(&mut token);
        assert_eq!(token.term, "cheval");
    }
}
