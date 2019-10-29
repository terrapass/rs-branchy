use rand::{
    self,
    Rng
};
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum Symbol<NT, T> {
    Nonterminal(NT),
    Terminal(T)
}

impl<NT, T> Symbol<NT, T> {
    fn is_terminal(&self) -> bool {
        if let Self::Terminal(_) = self {
            true
        } else {
            false
        }
    }

    fn is_nonterminal(&self) -> bool {
        !self.is_terminal()
    }

    fn unwrap_nonterm(self) -> NT {
        self.expect_nonterm(
            "unwrap_nonterm() must be used only on non-terminal symbols"
        )
    }

    fn expect_nonterm(self, message: &'static str) -> NT {
        if let Self::Nonterminal(value) = self {
            value
        } else {
            panic!(message);
        }
    }

    fn unwrap_term(self) -> T {
        self.expect_term(
            "unwrap_term() must be used only on terminal symbols"
        )
    }

    fn expect_term(self, message: &'static str) -> T {
        if let Self::Terminal(value) = self {
            value
        } else {
            panic!(message);
        }
    }
}

struct Rule<NT, T> {
    pattern: NT,
    replacement: Vec<Symbol<NT, T>>
}

impl<NT, T> Rule<NT, T> {
    pub fn new(pattern: NT, replacement: Vec<Symbol<NT, T>>) -> Self {
        Self{
            pattern,
            replacement
        }
    }
}

struct Grammar<NT, T> {
    rules: Vec<Rule<NT, T>>
}

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

    let grammar = Grammar{rules};

    let mut input: Vec<Symbol<_, _>> = vec![
        Terminal("The "),
        Nonterminal("colored_animal"),
        Terminal(" of the "),
        Nonterminal("location"),
        Terminal(" is called "),
        Nonterminal("name"),
        Terminal(".")
    ];

    let mut is_fully_expanded = false;

    #[allow(clippy::find_map)]
    for _ in 0..MAX_ITERATIONS {
        let maybe_first_nonterm = input.iter()
            .enumerate()
            .find(|(_, &symbol)| symbol.is_nonterminal())
            .map(|(idx, &symbol)| (idx, symbol.unwrap_nonterm()));

        if let Some((first_nonterm_idx, first_nonterm_value)) = maybe_first_nonterm {
            let matching_rules: Vec<_> = grammar.rules.iter()
                .filter(|rule| rule.pattern == first_nonterm_value)
                .collect();

            if matching_rules.is_empty() {
                eprintln!("No expansion for non-terminal symbol {:?}", first_nonterm_value);

                break;
            }

            let selected_rule_idx = rand::thread_rng().gen_range(0, matching_rules.len());
            let selected_rule     = matching_rules[selected_rule_idx];

            input.splice(
                first_nonterm_idx..=first_nonterm_idx,
                selected_rule.replacement.iter().cloned()
            );
        } else {
            is_fully_expanded = true;

            break;
        }
    }

    if is_fully_expanded {
        let result = input.into_iter()
            .map(Symbol::unwrap_term)
            .join("");

        println!("{}", result);
    } else {
        eprintln!(
            "Failed to fully expand input.\nLast input: {:?}",
            input
        );
    }
}
