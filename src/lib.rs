use itertools::Itertools;

mod grammar;
mod expansion;

use grammar::{
    NonterminalValue,
    TerminalValue,
    Symbol,
    Rule
};

use expansion::{
    ExpanderBuilder,
    ExpansionLogger,
    RuleSelector
};

pub fn run() {
    struct StdOutLogger;

    impl<Nt, T> ExpansionLogger<Nt, T> for StdOutLogger
        where Nt: NonterminalValue + std::fmt::Debug,
              T:  TerminalValue + std::fmt::Debug
    {
        fn on_nonterm_expanded(&mut self, expanded_nonterm_value: &Nt, rule: &Rule<Nt, T>) {
            println!("- expanded {:?} to {:?}", expanded_nonterm_value, rule.replacement);
        }
    
        fn on_nonterm_expansion_failed(&mut self, expanded_nonterm_value: &Nt) {
            println!("- failed to expand {:?}", expanded_nonterm_value);
        }
    
        fn on_input_fully_expanded(&mut self, expansion_result: &[T]) {
            println!("- fully expanded to {:?}", expansion_result);
        }
    
        fn on_max_iterations_reached(&mut self, current_state: &[Symbol<Nt, T>], iterations: usize) {
            println!("- reached the maximum {} iterations in state {:?}", iterations, current_state);
        }
    }

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

    const MAX_ITERATIONS: usize = 512;

    use Symbol::{
        Nonterminal,
        Terminal
    };

    let terminal_alternatives = vec![
        (
            "animal",
            vec![
                "cat",
                "dog",
                "dragon",
                "whale",
                "elephant",
                "snake",
                "hamster",
                "rabbit"
            ]
        ),
        (
            "color",
            vec![
                "red",
                "green",
                "blue",
                "cyan",
                "yellow",
                "magenta",
                "black",
                "white",
                "gray"
            ]
        ),
        (
            "biome",
            vec![
                "desert",
                "tundra",
                "forest",
                "wasteland",
                "jungle"
            ]
        ),
        (
            "town_size",
            vec![
                "tiny",
                "small",
                "large"
            ]
        ),
        (
            "city_size",
            vec![
                "small",
                "regular",
                "populous",
                "large",
                "huge"
            ]
        ),
        (
            "name",
            vec![
                "Arjun",
                "Betty",
                "Carl",
                "Daniel",
                "Ethan",
                "Felix",
                "George",
                "Hugh",
                "Ira",
                "Jill",
                "Katherine",
                "Lisa",
                "Malik",
                "Nancy",
                "Otto",
                "Pierre",
                "Quentin",
                "Roland",
                "Sarah",
                "Tracy",
                "Uma",
                "Victoria",
                "Wagner",
                "Xavier",
                "Ysolda",
                "Zina"
            ]
        )
    ];

    let mut rules = vec![
        Rule::new("colored_animal", vec![Nonterminal("color"), Terminal(" "), Nonterminal("animal")]),
        Rule::new("location", vec![Nonterminal("biome")]),
        Rule::new("location", vec![Nonterminal("town_size"), Terminal(" town")]),
        Rule::new("location", vec![Nonterminal("city_size"), Terminal(" city")]),
    ];

    rules.splice(
        rules.len()..rules.len(),
        terminal_alternatives.iter()
            .flat_map(|(non_term, terms)| {
                terms.iter().map(move |term| Rule::new(*non_term, vec![Terminal(*term)]))
            })
    );

    let input: Vec<Symbol<_, _>> = vec![
        Terminal("The "),
        Nonterminal("colored_animal"),
        Terminal(" of the "),
        Nonterminal("location"),
        Terminal(" is called "),
        Nonterminal("name"),
        Terminal(".")
    ];

    let mut expander = ExpanderBuilder::from(rules)
        .with_rule_selector(AlwaysFirstRuleSelector)
        .with_logger(StdOutLogger)
        .with_max_iterations(MAX_ITERATIONS)
        .build();

    let expansion_result = expander.expand(input);

    match expansion_result {
        Ok(result) => println!("{}", result.into_iter().join("")),
        Err(state) => eprintln!("Failed to fully expand input.\nLast state: {:?}", state)
    }
}
