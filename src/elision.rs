//! French elision removal.

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use hashbrown::HashSet;
use pizza_engine::analysis::{Token, TokenFilter};

/// Default French elision articles.
const FRENCH_ARTICLES: &[&str] = &[
    "l", "m", "t", "qu", "n", "s", "j", "d", "c", "jusqu", "quoiqu", "lorsqu", "puisqu",
];

/// Removes French article elisions (l', d', qu', etc.) from tokens.
#[derive(Clone, Debug)]
pub struct FrenchElisionFilter {
    articles: HashSet<String>,
}

impl FrenchElisionFilter {
    pub fn new() -> Self {
        Self {
            articles: FRENCH_ARTICLES.iter().map(|a| a.to_string()).collect(),
        }
    }

    pub fn with_articles(articles: &[&str]) -> Self {
        Self {
            articles: articles.iter().map(|a| a.to_lowercase()).collect(),
        }
    }
}

impl Default for FrenchElisionFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl TokenFilter for FrenchElisionFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let term = token.term.as_ref();
        if let Some(apos_pos) = term.find(|c| c == '\'' || c == '\u{2019}') {
            let prefix = &term[..apos_pos];
            if self.articles.contains(&prefix.to_lowercase()) {
                let remainder = &term[apos_pos + 1..];
                if !remainder.is_empty() {
                    token.term = Cow::Owned(remainder.to_string());
                    token.start_offset += (apos_pos as u32) + 1;
                }
            }
        }
        (false, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_french_elision() {
        let f = FrenchElisionFilter::new();
        let mut token = Token::new("l'avion", 0, 7, 0);
        f.filter(&mut token);
        assert_eq!(token.term, "avion");
    }

    #[test]
    fn test_case_insensitive() {
        let f = FrenchElisionFilter::new();
        let mut token = Token::new("L'homme", 0, 7, 0);
        f.filter(&mut token);
        assert_eq!(token.term, "homme");
    }

    #[test]
    fn test_no_elision() {
        let f = FrenchElisionFilter::new();
        let mut token = Token::new("bonjour", 0, 7, 0);
        f.filter(&mut token);
        assert_eq!(token.term, "bonjour");
    }
}
