//! Register French analysis components into [`AnalysisFactory`].

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

use pizza_engine::analysis::{
    Analyzer, AnalysisFactory, LowercaseNormalizer, Normalizer, StandardTokenizer, TokenFilter,
    Tokenizer,
};

use crate::{FrenchElisionFilter, FrenchLightStemFilter, FrenchStopFilter};

/// Register French token filters and the `"french"` analyzer.
pub fn register_all(factory: &mut AnalysisFactory) {
    factory.register_token_filter("french_elision", Box::new(FrenchElisionFilter::new()));
    factory.register_token_filter("french_light_stem", Box::new(FrenchLightStemFilter::new()));
    factory.register_token_filter("french_stop", Box::new(FrenchStopFilter::new()));

    let normalizers: Vec<Box<dyn Normalizer>> = vec![Box::new(LowercaseNormalizer::new())];
    let tokenizer: Box<dyn Tokenizer> = Box::new(StandardTokenizer::new());
    let filters: Vec<Box<dyn TokenFilter>> = vec![
        Box::new(FrenchElisionFilter::new()),
        Box::new(FrenchStopFilter::new()),
        Box::new(FrenchLightStemFilter::new()),
    ];
    factory.register_analyzer("french", Analyzer::new(normalizers, tokenizer, filters));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_all_no_panic() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
    }

    #[test]
    fn test_filters_registered() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(factory.get_token_filter("french_elision").is_some());
        assert!(factory.get_token_filter("french_light_stem").is_some());
        assert!(factory.get_token_filter("french_stop").is_some());
    }

    #[test]
    fn test_analyzer_registered() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(factory.get_analyzer("french").is_some());
    }

    #[test]
    fn test_analyzer_pipeline() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        let analyzer = factory.get_analyzer("french").unwrap();
        let mut input = String::from("L'homme est dans la maison");
        let tokens = analyzer.analyze_and_return_tokens(&mut input);
        // "la" is stop word, "l'" should be elided
        assert!(!tokens.iter().any(|t| t.term == "la"));
        assert!(tokens.len() >= 2);
    }
}
