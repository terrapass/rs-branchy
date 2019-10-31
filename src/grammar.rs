use rand::Rng;

//
// Interface
//

//
// Utilities
//

pub fn expand_input<Nt, T, RS, EL>(
    mut input:      Vec<Symbol<Nt, T>>,
    rules:          &[Rule<Nt, T>],
    rule_selector:  &RS,
    logger:         &mut EL,
    max_iterations: usize
) -> Result<Vec<T>, Vec<Symbol<Nt, T>>>
    where T:  Clone + Copy, // TODO: See if we can dispose of Copy bound for Nt and T
          Nt: Clone + Copy + PartialEq,
          RS: RuleSelector<Nt, T>,
          EL: ExpansionLogger<Nt, T>
{
    #[allow(clippy::find_map)]
    for _ in 0..max_iterations {
        let maybe_first_nonterm = input.iter()
            .enumerate()
            .find(|(_, &symbol)| symbol.is_nonterminal())
            .map(|(idx, &symbol)| (idx, symbol.unwrap_nonterm()));

        if let Some((first_nonterm_idx, first_nonterm_value)) = maybe_first_nonterm {
            let maybe_selected_rule = rule_selector.select_rule(rules, &first_nonterm_value);

            // TODO: See if this can be rewritten without another nested if let
            if let Some(selected_rule) = maybe_selected_rule {
                input.splice(
                    first_nonterm_idx..=first_nonterm_idx,
                    selected_rule.replacement.iter().cloned()
                );
            } else {
                logger.on_nonterm_expansion_failed(&first_nonterm_value);

                return Err(input);
            }
        } else {
            let expansion_result: Vec<_> = input.into_iter().map(Symbol::unwrap_term).collect();

            logger.on_input_fully_expanded(&expansion_result);

            return Ok(expansion_result);
        }
    }

    logger.on_max_iterations_reached(&input, max_iterations);

    // TODO: Use Err value to distinguish between reaching max iterations and failing to expand a nonterm. symbol.
    Err(input)
}

//
// Traits // TODO: Move into a (sub)module?
//

pub trait RuleSelector<Nt, T>
{
    fn select_rule<'a>(&self, all_rules: &'a [Rule<Nt, T>], nonterm_value: &Nt) -> Option<&'a Rule<Nt,T>>
        where Nt: PartialEq
    {
        self.select_matching_rule(
            self.find_matching_rules(all_rules, nonterm_value).as_slice()
        )
    }

    fn find_matching_rules<'a>(&self, all_rules: &'a [Rule<Nt, T>], nonterm_value: &Nt) -> Vec<&'a Rule<Nt, T>>
        where Nt: PartialEq
    {
        all_rules.iter()
            .filter(|rule| rule.pattern == *nonterm_value)
            .collect()
    }

    fn select_matching_rule<'a>(&self, matching_rules: &[&'a Rule<Nt, T>]) -> Option<&'a Rule<Nt, T>>;
}

pub trait ExpansionLogger<Nt, T> {
    fn on_nonterm_expanded(&mut self, _expanded_nonterm_value: &Nt, _rule: &Rule<Nt, T>) {
        // Empty
    }

    fn on_nonterm_expansion_failed(&mut self, _expanded_nonterm_value: &Nt) {
        // Empty
    }

    fn on_input_fully_expanded(&mut self, _expansion_result: &[T]) {
        // Empty
    }

    fn on_max_iterations_reached(&mut self, _current_state: &[Symbol<Nt, T>], _iterations: usize) {
        // Empty
    }
}

//
// Types
//

// TODO: Extract RuleSelector and ExpansionLogger implementations into a (sub)module

pub struct UniformRandomRuleSelector;

impl<Nt, T> RuleSelector<Nt, T> for UniformRandomRuleSelector {
    fn select_matching_rule<'a>(&self, matching_rules: &[&'a Rule<Nt, T>]) -> Option<&'a Rule<Nt, T>> {
        if matching_rules.is_empty() {
            None
        } else {
            let selected_rule_idx = rand::thread_rng().gen_range(0, matching_rules.len());

            Some(matching_rules[selected_rule_idx])
        }
    }
}

pub struct NullExpansionLogger;

impl<Nt, T> ExpansionLogger<Nt, T> for NullExpansionLogger {
    // Default
}

//
// enum Symbol<Nt, T>
//

#[derive(Debug, Clone, Copy)]
pub enum Symbol<Nt, T> {
    Nonterminal(Nt),
    Terminal(T)
}

//
// Methods
//

impl<Nt, T> Symbol<Nt, T> {
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

    fn unwrap_nonterm(self) -> Nt {
        self.expect_nonterm(
            "unwrap_nonterm() must be used only on non-terminal symbols"
        )
    }

    fn expect_nonterm(self, message: &'static str) -> Nt {
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

//
// struct Rule<Nt, T>
//

pub struct Rule<Nt, T> {
    pattern: Nt,
    replacement: Vec<Symbol<Nt, T>>
}

//
// Methods
//

impl<Nt, T> Rule<Nt, T> {
    pub fn new(pattern: Nt, replacement: Vec<Symbol<Nt, T>>) -> Self {
        Self{
            pattern,
            replacement
        }
    }
}
