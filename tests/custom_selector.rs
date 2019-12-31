use branchy::{
    Symbol,
    Rule,
    ExpanderBuilder,
    RuleSelector
};

#[test]
fn custom_selector()
{
    struct AlwaysFirstRuleSelector;
 
    impl<Nt, T> RuleSelector<Nt, T> for AlwaysFirstRuleSelector {
        fn select_matching_rule<'a>(&self, matching_rules: &[&'a Rule<Nt, T>]) -> Option<&'a Rule<Nt, T>> {
            if matching_rules.is_empty() {
                None
            } else {
                Some(matching_rules[0])
            }
        }
    }

    let input = vec![
        Symbol::Terminal("There is a"),
        Symbol::Nonterminal("site_description"),
        Symbol::Terminal("to the"),
        Symbol::Nonterminal("direction"),
        Symbol::Terminal("of the town.")
    ];

    let rules = vec![
        Rule::new("site_description", vec![Symbol::Nonterminal("adjective"), Symbol::Nonterminal("site")]),
        Rule::new("adjective", vec![Symbol::Terminal("huge")]),
        Rule::new("adjective", vec![Symbol::Terminal("dark")]),
        Rule::new("site", vec![Symbol::Terminal("forest")]),
        Rule::new("site", vec![Symbol::Terminal("cave")]),
        Rule::new("direction", vec![Symbol::Terminal("north")]),
        Rule::new("direction", vec![Symbol::Terminal("east")])
    ];

    let mut expander = ExpanderBuilder::from(rules)
        .with_rule_selector(AlwaysFirstRuleSelector)
        .build();

    let expansion_result = expander.expand(input).unwrap();

    assert_eq!(
        expansion_result,
        vec!["There is a", "huge", "forest", "to the", "north", "of the town."]
    );
}