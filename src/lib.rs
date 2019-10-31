use itertools::Itertools;

mod grammar;

use grammar::{
    Symbol,
    Rule
};

pub fn run() {
    const MAX_ITERATIONS: usize = 1024;

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

    let expansion_result = grammar::expand_input(
        input,
        &rules,
        &grammar::UniformRandomRuleSelector{},
        &mut grammar::NullExpansionLogger{},
        MAX_ITERATIONS
    );

    match expansion_result {
        Ok(result) => println!("{}", result.into_iter().join("")),
        Err(state) => eprintln!("Failed to fully expand input.\nLast state: {:?}", state)
    }
}
