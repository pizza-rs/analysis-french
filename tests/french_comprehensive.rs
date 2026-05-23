//! Comprehensive tests for pizza-analysis-french.

use pizza_analysis_french::*;
use pizza_engine::analysis::{AnalysisFactory, Token, TokenFilter};

fn make_token(term: &str) -> Token<'_> {
    Token::new(term, 0, term.len() as u32, 0)
}

// ═══════════════════════════════════════════════════════════════════════════════
// FrenchElisionFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn elision_construction() {
    let _f = FrenchElisionFilter::new();
}

#[test]
fn elision_removes_l_apostrophe() {
    let f = FrenchElisionFilter::new();
    // "l'homme" → "homme"
    let mut token = make_token("l'homme");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert_eq!(token.term.as_ref(), "homme");
}

#[test]
fn elision_removes_d_apostrophe() {
    let f = FrenchElisionFilter::new();
    // "d'accord" → "accord"
    let mut token = make_token("d'accord");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert_eq!(token.term.as_ref(), "accord");
}

#[test]
fn elision_removes_qu() {
    let f = FrenchElisionFilter::new();
    // "qu'il" → "il"
    let mut token = make_token("qu'il");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert_eq!(token.term.as_ref(), "il");
}

#[test]
fn elision_removes_n_apostrophe() {
    let f = FrenchElisionFilter::new();
    // "n'est" → "est"
    let mut token = make_token("n'est");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert_eq!(token.term.as_ref(), "est");
}

#[test]
fn elision_no_change_without_apostrophe() {
    let f = FrenchElisionFilter::new();
    let mut token = make_token("maison");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert_eq!(token.term.as_ref(), "maison");
}

#[test]
fn elision_empty_string() {
    let f = FrenchElisionFilter::new();
    let mut token = make_token("");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

// ═══════════════════════════════════════════════════════════════════════════════
// FrenchLightStemFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn light_stem_construction() {
    let _f = FrenchLightStemFilter::new();
}

#[test]
fn light_stem_plural() {
    let f = FrenchLightStemFilter::new();
    // "chevaux" (horses) → stem
    let mut token = make_token("chevaux");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert_ne!(token.term.as_ref(), "chevaux");
}

#[test]
fn light_stem_feminine() {
    let f = FrenchLightStemFilter::new();
    // "petite" → stem (small, feminine)
    let mut token = make_token("petite");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn light_stem_verb_conjugation() {
    let f = FrenchLightStemFilter::new();
    let mut token = make_token("mangeons");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn light_stem_short_word() {
    let f = FrenchLightStemFilter::new();
    let mut token = make_token("le");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn light_stem_empty_string() {
    let f = FrenchLightStemFilter::new();
    let mut token = make_token("");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

// ═══════════════════════════════════════════════════════════════════════════════
// FrenchMinimalStemFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn minimal_stem_construction() {
    let _f = FrenchMinimalStemFilter::new();
}

#[test]
fn minimal_stem_plural_s() {
    let f = FrenchMinimalStemFilter::new();
    // "chats" → "chat"
    let mut token = make_token("chats");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert_ne!(token.term.as_ref(), "chats");
}

#[test]
fn minimal_stem_plural_x() {
    let f = FrenchMinimalStemFilter::new();
    // "chevaux" → remove x
    let mut token = make_token("chevaux");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

// ═══════════════════════════════════════════════════════════════════════════════
// FrenchStopFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn stop_construction() {
    let _f = FrenchStopFilter::new();
}

#[test]
fn stop_filters_common_words() {
    let f = FrenchStopFilter::new();
    let stop_words = ["le", "la", "les", "de", "des", "du", "un", "une", "et", "en", "que", "au"];
    for word in &stop_words {
        let mut token = make_token(word);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted, "stop word '{}' should be filtered", word);
    }
}

#[test]
fn stop_keeps_content_words() {
    let f = FrenchStopFilter::new();
    let content_words = ["maison", "livre", "chat", "école"];
    for word in &content_words {
        let mut token = make_token(word);
        let (deleted, _) = f.filter(&mut token);
        assert!(!deleted, "content word '{}' should be kept", word);
    }
}

#[test]
fn stop_empty_string() {
    let f = FrenchStopFilter::new();
    let mut token = make_token("");
    let _ = f.filter(&mut token);
}

// ═══════════════════════════════════════════════════════════════════════════════
// Registration
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn register_all_no_panic() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
}

#[test]
fn register_all_filters_present() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_token_filter("french_elision").is_some());
    assert!(factory.get_token_filter("french_light_stem").is_some());
    assert!(factory.get_token_filter("french_stop").is_some());
}

#[test]
fn register_all_analyzer_present() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_analyzer("french").is_some());
}

#[test]
fn analyzer_pipeline_produces_tokens() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("french").unwrap();
    let mut input = String::from("Le chat est sur la table");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(!tokens.is_empty());
}

#[test]
fn analyzer_pipeline_removes_stops() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("french").unwrap();
    let mut input = String::from("le livre de la maison");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    let terms: Vec<&str> = tokens.iter().map(|t| t.term.as_ref()).collect();
    assert!(!terms.contains(&"le"));
    assert!(!terms.contains(&"de"));
    assert!(!terms.contains(&"la"));
}

#[test]
fn analyzer_pipeline_empty_input() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("french").unwrap();
    let mut input = String::from("");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(tokens.is_empty());
}

#[test]
fn analyzer_pipeline_elision_handling() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("french").unwrap();
    let mut input = String::from("l'école");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(!tokens.is_empty());
}
