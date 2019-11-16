mod error;
mod helpers;

use crate::grammar::{
    NonterminalValue,
    TerminalValue,
    Symbol,
    Rule
};

//
// Constants
//

const DEFAULT_MAX_ITERATIONS: usize = 1024;

//
// Interface traits
//

pub use helpers::RuleSelector;
pub use helpers::ExpansionLogger;

//
// Interface types
//

pub use error::Error;
pub use helpers::builtin::UniformRandomRuleSelector;
pub use helpers::builtin::NullExpansionLogger;

pub type Result<Nt, T> = std::result::Result<Vec<T>, Error<Nt, T>>;

//
// Expander<Nt, T, RS, EL>
//

pub struct Expander<Nt, T, RS, EL>
    where RS: RuleSelector<Nt, T>,
          EL: ExpansionLogger<Nt, T>
{
    config: ExpanderConfig<Nt, T, RS, EL>
}

impl<Nt, T, RS, EL> Expander<Nt, T, RS, EL>
    where Nt: NonterminalValue,
          T:  TerminalValue,
          RS: RuleSelector<Nt, T>,
          EL: ExpansionLogger<Nt, T>
{
    //
    // Interface
    //

    pub fn expand(&mut self, input: Vec<Symbol<Nt, T>>) -> Result<Nt, T> {
        expand_input(
            input,
            &self.config.rules,
            &self.config.rule_selector,
            &mut self.config.logger,
            self.config.max_iterations
        )
    }

    //
    // Friend interface
    //

    fn new(config: ExpanderConfig<Nt, T, RS, EL>) -> Self {
        Self{config}
    }
}

//
// ExpanderBuilder<Nt, T, RS, EL>
//

pub struct ExpanderBuilder<Nt, T, RS, EL>
    where RS: RuleSelector<Nt, T>,
          EL: ExpansionLogger<Nt, T>
{
    config: ExpanderConfig<Nt, T, RS, EL>
}

impl<Nt, T> ExpanderBuilder<Nt, T, UniformRandomRuleSelector, NullExpansionLogger>
    where Nt: NonterminalValue,
          T:  TerminalValue
{
    pub fn new() -> Self {
        Self::from(Vec::new())
    }

    pub fn from<Rs>(rules: Rs) -> Self
        where Rs: IntoIterator<Item = Rule<Nt, T>>
    {
        Self{
            config: ExpanderConfig{
                rules:          rules.into_iter().collect(),
                rule_selector:  UniformRandomRuleSelector,
                logger:         NullExpansionLogger,
                max_iterations: DEFAULT_MAX_ITERATIONS
            }
        }
    }
}

#[allow(clippy::use_self)]
impl<Nt, T, RS, EL> ExpanderBuilder<Nt, T, RS, EL>
    where Nt: NonterminalValue,
          T:  TerminalValue,
          RS: RuleSelector<Nt, T>,
          EL: ExpansionLogger<Nt, T>
{
    pub fn build(self) -> Expander<Nt, T, RS, EL> {
        Expander::new(self.config)
    }

    pub fn with_new_rule<Ss>(mut self, pattern: Nt, replacement: Ss) -> Self
        where Ss: IntoIterator<Item = Symbol<Nt, T>>
    {
        self.config.rules.push(
            Rule::new(pattern, replacement.into_iter().collect())
        );

        self
    }

    pub fn with_rule(mut self, rule: Rule<Nt, T>) -> Self {
        self.config.rules.push(rule);

        self
    }

    pub fn with_rules<Rs>(mut self, rules: Rs) -> Self
        where Rs: IntoIterator<Item = Rule<Nt, T>>
    {
        let my_rules = &mut self.config.rules;

        my_rules.splice(
            my_rules.len()..my_rules.len(),
            rules.into_iter()
        );

        self
    }

    pub fn with_rule_selector<NewRS>(self, rule_selector: NewRS) -> ExpanderBuilder<Nt, T, NewRS, EL>
        where NewRS: RuleSelector<Nt, T>
    {
        ExpanderBuilder{
            config: ExpanderConfig{
                rules:          self.config.rules,
                rule_selector,
                logger:         self.config.logger,
                max_iterations: self.config.max_iterations
            }
        }
    }

    pub fn with_logger<NewEL>(self, logger: NewEL) -> ExpanderBuilder<Nt, T, RS, NewEL>
        where NewEL: ExpansionLogger<Nt, T>
    {
        ExpanderBuilder{
            config: ExpanderConfig{
                rules:          self.config.rules,
                rule_selector:  self.config.rule_selector,
                logger,
                max_iterations: self.config.max_iterations
            }
        }
    }

    pub fn with_max_iterations(self, max_iterations: usize) -> Self {
        Self{
            config: ExpanderConfig {
                max_iterations,
                ..self.config
            }
        }
    }
}

//
// Service types
//

//
// ExpanderConfig<Nt, T, RS, EL>
//

pub struct ExpanderConfig<Nt, T, RS, EL>
    where RS: RuleSelector<Nt, T>,
          EL: ExpansionLogger<Nt, T>
{
    rules:          Vec<Rule<Nt, T>>,
    rule_selector:  RS,
    logger:         EL,
    max_iterations: usize
}

//
// Service
//

fn expand_input<Nt, T, RS, EL>(
    mut input:      Vec<Symbol<Nt, T>>,
    rules:          &[Rule<Nt, T>],
    rule_selector:  &RS,
    logger:         &mut EL,
    max_iterations: usize
) -> Result<Nt, T>
    where Nt: NonterminalValue,
          T:  TerminalValue,
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

                logger.on_nonterm_expanded(&first_nonterm_value, &selected_rule);
            } else {
                logger.on_nonterm_expansion_failed(&first_nonterm_value);

                return Err(Error::nonterm_expansion_failed(input, first_nonterm_value));
            }
        } else {
            let expansion_result: Vec<_> = input.into_iter().map(Symbol::unwrap_term).collect();

            logger.on_input_fully_expanded(&expansion_result);

            return Ok(expansion_result);
        }
    }

    logger.on_max_iterations_reached(&input, max_iterations);

    Err(Error::max_iterations_reached(input, max_iterations))
}
