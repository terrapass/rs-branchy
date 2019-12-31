use crate::grammar::{
    NonterminalValue,
    Symbol,
    Rule
};

pub mod builtin;

//
// Interface traits
//

/// Implement this trait if you want to provide custom rule selection logic.
///
/// See [crate docs](index.html#using-a-custom-rule-selector) for an example.
pub trait RuleSelector<Nt, T>
{
    fn select_rule<'a>(&self, all_rules: &'a [Rule<Nt, T>], nonterm_value: &Nt) -> Option<&'a Rule<Nt,T>>
        where Nt: NonterminalValue
    {
        self.select_matching_rule(
            self.find_matching_rules(all_rules, nonterm_value).as_slice()
        )
    }

    fn find_matching_rules<'a>(&self, all_rules: &'a [Rule<Nt, T>], nonterm_value: &Nt) -> Vec<&'a Rule<Nt, T>>
        where Nt: NonterminalValue
    {
        all_rules.iter()
            .filter(|rule| rule.pattern == *nonterm_value)
            .collect()
    }

    fn select_matching_rule<'a>(&self, matching_rules: &[&'a Rule<Nt, T>]) -> Option<&'a Rule<Nt, T>>;
}

/// Implement this trait if you want to log or otherwise handle individual steps during expansion.
///
/// See [crate docs](index.html#logging) for an example.
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
