//! This crate provides basic tools for generation of text strings and other sequences
//! using [context-free grammars](https://www.cs.rochester.edu/~nelson/courses/csc_173/grammars/cfg.html).
//!
//! ## Text generation example
//! 
//! The following example demonstrates random generation of a short sentence based on a sequence of input symbols
//! and a set of rules, which may be applied to the symbol sequence until it is fully expanded.
//! 
//! ```
//! use branchy::{
//!     Symbol,
//!     Rule,
//!     ExpanderBuilder
//! };
//!
//! let input = vec![Symbol::Nonterminal("person"), Symbol::Terminal("comes from a"), Symbol::Nonterminal("location")];
//!
//! let rules = vec![
//!     Rule::new("person", vec![Symbol::Nonterminal("name")]),
//!     Rule::new(
//!         "person",
//!         vec![Symbol::Nonterminal("name"), Symbol::Terminal("the"), Symbol::Nonterminal("occupation")]
//!     ),
//!     Rule::new("name", vec![Symbol::Terminal("Alice")]),
//!     Rule::new("name", vec![Symbol::Terminal("Bob")]),
//!     Rule::new("occupation", vec![Symbol::Terminal("blacksmith")]),
//!     Rule::new("occupation", vec![Symbol::Terminal("baker")]),
//!     Rule::new("location", vec![Symbol::Nonterminal("size"), Symbol::Nonterminal("settlement_type")]),
//!     Rule::new("size", vec![Symbol::Terminal("small")]),
//!     Rule::new("size", vec![Symbol::Terminal("big")]),
//!     Rule::new("settlement_type", vec![Symbol::Terminal("village")]),
//!     Rule::new("settlement_type", vec![Symbol::Terminal("town")])
//! ];
//!
//! let mut expander = ExpanderBuilder::from(rules).build();
//!
//! let expansion_result = expander.expand(input);
//! assert!(expansion_result.is_ok());
//!
//! let expanded_string = expansion_result.unwrap().join(" ");
//! println!("{}", expanded_string);
//! ```
//! 
//! When run, this example prints out a sentence, similar to the ones below:
//! 
//! >    Alice the blacksmith comes from a big village
//! >
//! >    Bob comes from a small town
//! >
//! >    Bob the baker comes from a big town
//! 
//! As you can see, both the input sequence and the rules of the grammar are described in terms of
//! [`Nonterminal`](enum.Symbol.html#variant.Nonterminal) (ones that can be further expanded)
//! and [`Terminal`](enum.Symbol.html#variant.Terminal) symbols.
//! All of the rules have a non-terminal symbol value on their left-hand side and a sequence which
//! may contain both [`Nonterminal`](enum.Symbol.html#variant.Nonterminal) and
//! [`Terminal`](enum.Symbol.html#variant.Terminal) symbols on their right-hand side.
//! 
//! The "magic" happens in [`Expander`](struct.Expander.html)'s [`expand()`](struct.Expander.html#method.expand) method,
//! which repeatedly selects and applies matching rules until the sequence is fully expanded
//! (i.e contains only terminal symbols).
//! 
//! By default, [`UniformRandomRuleSelector`](struct.UniformRandomRuleSelector.html) is used
//! to select rules while expanding, therefore the result is randomized. As we'll see below,
//! this can be changed, if needed, via [`ExpanderBuilder`](struct.ExpanderBuilder.html).
//! 
//! ## Using a custom rule selector
//! 
//! When constructing an [`Expander`](struct.Expander.html), you can provide your own
//! rule selector via [`ExpanderBuilder`](struct.ExpanderBuilder.html)'s
//! [`with_rule_selector()`](struct.ExpanderBuilder.html#method.with_rule_selector) method.
//!
//! The following example defines a custom rule selector, which always chooses the first
//! matching rule, and then  uses it in generation of a short phrase.
//! As you can see, rule selectors need to implement at least the
//! [`select_matching_rule()`](trait.RuleSelector.html#method.select_matching_rule) method
//! from the [`RuleSelector`](trait.RuleSelector.html) trait.
//! 
//! ```
//! use branchy::{
//!     Symbol,
//!     Rule,
//!     ExpanderBuilder,
//!     RuleSelector
//! };
//! 
//! struct AlwaysFirstRuleSelector;
//! 
//! impl<Nt, T> RuleSelector<Nt, T> for AlwaysFirstRuleSelector {
//!     fn select_matching_rule<'a>(&self, matching_rules: &[&'a Rule<Nt, T>]) -> Option<&'a Rule<Nt, T>> {
//!         if matching_rules.is_empty() {
//!             None
//!         } else {
//!             Some(matching_rules[0])
//!         }
//!     }
//! }
//! 
//! let input = vec![Symbol::Terminal("Have a"), Symbol::Nonterminal("adjective"), Symbol::Nonterminal("time_of_day")];
//! 
//! let mut expander = ExpanderBuilder::new()
//!     .with_new_rule("adjective", vec![Symbol::Terminal("wonderful")])
//!     .with_new_rule("adjective", vec![Symbol::Terminal("great")])
//!     .with_new_rule("time_of_day", vec![Symbol::Terminal("afternoon")])
//!     .with_new_rule("time_of_day", vec![Symbol::Terminal("evening")])
//!     .with_rule_selector(AlwaysFirstRuleSelector)
//!     .build();
//! 
//! let expanded_string = expander.expand(input).unwrap().join(" ");
//! 
//! assert_eq!(expanded_string, "Have a wonderful afternoon");
//! ```
//! This example also sets the rules of the grammar directly on [`ExpanderBuilder`](struct.ExpanderBuilder.html)
//! via the [`with_new_rule()`](struct.ExpanderBuilder.html#method.with_new_rule) method. See the documentation
//! of [`ExpanderBuilder`](struct.ExpanderBuilder.html) for more helper methods.
//! 
//! ## Logging
//! 
//! To help you debug your grammars, `branchy` provides the [`ExpansionLogger`](trait.ExpansionLogger.html) trait,
//! which you can implement in order to be notified of the progress of the expansion and the steps it takes.
//! 
//! For example, in order to log the rules selected at each step of the expansion, you can implement the
//! [`on_nonterm_expanded()`](trait.ExpansionLogger.html#method.on_nonterm_expanded) method. The following example
//! writes a message via `println!()` on every step.
//! 
//! ```
//! use branchy::{
//!     Symbol,
//!     Rule,
//!     ExpanderBuilder,
//!     ExpansionLogger,
//!     TerminalValue,
//!     NonterminalValue
//! };
//! 
//! struct StdOutLogger;
//! 
//! impl<Nt, T> ExpansionLogger<Nt, T> for StdOutLogger
//!     where Nt: NonterminalValue + std::fmt::Debug,
//!           T:  TerminalValue + std::fmt::Debug
//! {
//!     fn on_nonterm_expanded(&mut self, expanded_nonterm_value: &Nt, rule: &Rule<Nt, T>) {
//!         println!("expanded {:?} to {:?}", expanded_nonterm_value, rule.replacement);
//!     }
//! }
//! 
//! let input = vec![
//!     Symbol::Terminal("There is a"),
//!     Symbol::Nonterminal("site_description"),
//!     Symbol::Terminal("to the"),
//!     Symbol::Nonterminal("direction"),
//!     Symbol::Terminal("of the town.")
//! ];
//! 
//! let rules = vec![
//!     Rule::new("site_description", vec![Symbol::Nonterminal("adjective"), Symbol::Nonterminal("site")]),
//!     Rule::new("adjective", vec![Symbol::Terminal("huge")]),
//!     Rule::new("adjective", vec![Symbol::Terminal("dark")]),
//!     Rule::new("site", vec![Symbol::Terminal("forest")]),
//!     Rule::new("site", vec![Symbol::Terminal("cave")]),
//!     Rule::new("direction", vec![Symbol::Terminal("north")]),
//!     Rule::new("direction", vec![Symbol::Terminal("east")])
//! ];
//! 
//! let mut expander = ExpanderBuilder::from(rules)
//!     .with_logger(StdOutLogger)
//!     .build();
//! 
//! expander.expand(input).unwrap();
//! ```
//! This example produces output similar to the following:
//! ```txt
//! expanded "site_description" to [Nonterminal("adjective"), Nonterminal("site")]
//! expanded "adjective" to [Terminal("dark")]
//! expanded "site" to [Terminal("cave")]
//! expanded "direction" to [Terminal("east")]
//! ```
//! 
//! ## Generating non-text sequences
//! 
//! Even though the primary use-case for `branchy` is generating text strings, it can be used for
//! grammars producing other kinds of sequences. Any type implementing `Copy + PartialEq` can be
//! used for values of non-terminal symbols and any type implementing `Copy` can be used for
//! terminals. See [`NonterminalValue`](trait.NonterminalValue.html) and
//! [`TerminalValue`](trait.TerminalValue.html) traits.

mod grammar;
mod expansion;

pub use grammar::{
    NonterminalValue,
    TerminalValue,
    Symbol,
    Rule
};

pub use expansion::{
    Expander,
    ExpanderBuilder,
    RuleSelector,
    ExpansionLogger,
    UniformRandomRuleSelector,
    NullExpansionLogger,
    Error,
    Result
};
