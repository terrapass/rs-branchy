use branchy::{
    Symbol,
    Rule,
    ExpanderBuilder
};

#[test]
fn short_sentence()
{
    let input = vec![Symbol::Nonterminal("person"), Symbol::Terminal("comes from a"), Symbol::Nonterminal("location")];

    let rules = vec![
        Rule::new("person", vec![Symbol::Nonterminal("name")]),
        Rule::new(
            "person",
            vec![Symbol::Nonterminal("name"), Symbol::Terminal("the"), Symbol::Nonterminal("occupation")]
        ),
        Rule::new("name", vec![Symbol::Terminal("Alice")]),
        Rule::new("name", vec![Symbol::Terminal("Bob")]),
        Rule::new("occupation", vec![Symbol::Terminal("blacksmith")]),
        Rule::new("occupation", vec![Symbol::Terminal("baker")]),
        Rule::new("location", vec![Symbol::Nonterminal("size"), Symbol::Nonterminal("settlement_type")]),
        Rule::new("size", vec![Symbol::Terminal("small")]),
        Rule::new("size", vec![Symbol::Terminal("big")]),
        Rule::new("settlement_type", vec![Symbol::Terminal("village")]),
        Rule::new("settlement_type", vec![Symbol::Terminal("town")])
    ];

    let mut expander = ExpanderBuilder::from(rules).build();

    let expansion_result: Vec<_> = expander.expand(input).unwrap();

    assert!(expansion_result.contains(&"comes from a"));
    assert!(!expansion_result.contains(&"person"));
    assert!(!expansion_result.contains(&"location"));
    assert!(
        expansion_result[0] == "Alice" || expansion_result[0] == "Bob"
    );
    assert!(
        expansion_result.iter()
            .skip(1)
            .find(|symbol| *symbol == &"Alice" || *symbol == &"Bob")
            .is_none()
    );
}
