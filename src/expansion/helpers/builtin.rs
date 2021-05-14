use rand::Rng;

use crate::grammar::Rule;
use super::{
    RuleSelector,
    ExpansionLogger
};

//
// Interface types
//

//
// UniformRandomRuleSelector: RuleSelector<Nt, T> + Default
//

/// Default rule selector. Randomly selects one of the matching rules
/// for every encountered non-terminal symbol.
pub struct UniformRandomRuleSelector; // TODO: Replace with RandomRuleSelector<RNG>

impl Default for UniformRandomRuleSelector {
    fn default() -> Self {
        Self::new()
    }
}

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

impl UniformRandomRuleSelector {
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

//
// NullExpansionLogger: ExpansionLogger<Nt, T>
//

/// Default logger. Does nothing.
pub struct NullExpansionLogger;

impl<Nt, T> ExpansionLogger<Nt, T> for NullExpansionLogger {
    // Default
}
