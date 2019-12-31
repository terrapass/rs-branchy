use super::*;

use std::{
    cell::RefCell,
    rc::Rc,
};

//
// Tests
//

#[test]
fn expand_input_all_terminal() {
    let input: Vec<Symbol<i32, _>> = vec![Symbol::Terminal(0), Symbol::Terminal(1), Symbol::Terminal(2)];

    let (expansion_result, mock_rule_selector_state, mock_logger_state) = expand_input_with_mocks(
        input,
        &vec![],
        UniformRandomRuleSelector::new(),
        1
    );

    let expansion_result = expansion_result.unwrap();

    assert_eq!(expansion_result, vec![0, 1, 2]);

    assert!(mock_rule_selector_state.borrow().select_rule_calls.is_empty());

    assert!(mock_logger_state.borrow().on_nonterm_expanded_calls.is_empty());
    assert!(mock_logger_state.borrow().on_nonterm_expansion_failed_calls.is_empty());
    assert_eq!(mock_logger_state.borrow().on_input_fully_expanded_calls, vec![expansion_result]);
    assert!(mock_logger_state.borrow().on_max_iterations_reached_calls.is_empty());
}

#[test]
fn expand_input_single_option() {
    let input = vec![Symbol::Nonterminal(100), Symbol::Terminal(0), Symbol::Nonterminal(101)];

    let rules = vec![
        Rule::new(100, vec![Symbol::Terminal(0)]),
        Rule::new(101, vec![Symbol::Terminal(1)])
    ];

    let (expansion_result, mock_rule_selector_state, mock_logger_state) = expand_input_with_mocks(
        input,
        &rules,
        UniformRandomRuleSelector::new(),
        3
    );

    let expansion_result = expansion_result.unwrap();

    assert_eq!(expansion_result, vec![0, 0, 1]);

    assert_eq!(
        mock_rule_selector_state.borrow().select_rule_calls,
        vec![
            (rules.clone(), 100),
            (rules.clone(), 101)
        ]
    );

    assert_eq!(
        mock_logger_state.borrow().on_nonterm_expanded_calls,
        vec![
            (100, rules[0].clone()),
            (101, rules[1].clone())
        ]
    );
    assert!(mock_logger_state.borrow().on_nonterm_expansion_failed_calls.is_empty());
    assert_eq!(mock_logger_state.borrow().on_input_fully_expanded_calls, vec![expansion_result]);
    assert!(mock_logger_state.borrow().on_max_iterations_reached_calls.is_empty());
}

#[test]
fn expand_input_multiple_options() {
    let input = vec![Symbol::Nonterminal("name"), Symbol::Terminal("likes"), Symbol::Nonterminal("food")];

    let rules = vec![
        Rule::new("name", vec![Symbol::Terminal("Susan")]),
        Rule::new("name", vec![Symbol::Terminal("Max")]),
        Rule::new("food", vec![Symbol::Terminal("chocolate")]),
        Rule::new("food", vec![Symbol::Terminal("oranges")])
    ];

    let (expansion_result, mock_rule_selector_state, mock_logger_state) = expand_input_with_mocks(
        input,
        &rules,
        AlwaysFirstRuleSelector::new(),
        3
    );

    let expansion_result = expansion_result.unwrap();

    assert_eq!(expansion_result, vec!["Susan", "likes", "chocolate"]);

    assert_eq!(
        mock_rule_selector_state.borrow().select_rule_calls,
        vec![
            (rules.clone(), "name"),
            (rules.clone(), "food")
        ]
    );

    assert_eq!(
        mock_logger_state.borrow().on_nonterm_expanded_calls,
        vec![
            ("name", rules[0].clone()),
            ("food", rules[2].clone())
        ]
    );
    assert!(mock_logger_state.borrow().on_nonterm_expansion_failed_calls.is_empty());
    assert_eq!(mock_logger_state.borrow().on_input_fully_expanded_calls, vec![expansion_result]);
    assert!(mock_logger_state.borrow().on_max_iterations_reached_calls.is_empty());
}

#[test]
fn expand_input_long_nonterm_chain() {
    const MAX_SYMBOL_VALUE: i32 = 10;

    let input: Vec<Symbol<_, i32>> = vec![Symbol::Nonterminal(1), Symbol::Nonterminal(-1)];

    let rules: Vec<Rule<_, _>> = (1..MAX_SYMBOL_VALUE)
        .map(|n| {
            let symbol_variant = if n + 1 < MAX_SYMBOL_VALUE {
                Symbol::Nonterminal
            } else {
                Symbol::Terminal
            };

            vec![
                Rule::new(n, vec![symbol_variant(n + 1)]),
                Rule::new(-n, vec![symbol_variant(-(n + 1))])
            ]
        })
        .flatten()
        .collect();

    let (expansion_result, mock_rule_selector_state, mock_logger_state) = expand_input_with_mocks(
        input,
        &rules,
        UniformRandomRuleSelector::new(),
        2*(MAX_SYMBOL_VALUE as usize) - 1
    );

    let expansion_result = expansion_result.unwrap();

    assert_eq!(expansion_result, vec![MAX_SYMBOL_VALUE, -MAX_SYMBOL_VALUE]);

    assert_eq!(mock_rule_selector_state.borrow().select_rule_calls.len(), 2*(MAX_SYMBOL_VALUE - 1) as usize);

    assert_eq!(mock_logger_state.borrow().on_nonterm_expanded_calls.len(), 2*(MAX_SYMBOL_VALUE - 1) as usize);
    assert!(mock_logger_state.borrow().on_nonterm_expansion_failed_calls.is_empty());
    assert_eq!(mock_logger_state.borrow().on_input_fully_expanded_calls, vec![expansion_result]);
    assert!(mock_logger_state.borrow().on_max_iterations_reached_calls.is_empty());
}

#[test]
fn expand_input_err_nonterm_expansion_failed() {
    let input: Vec<Symbol<_, i32>> = vec![Symbol::Nonterminal(0)];

    let (expansion_result, mock_rule_selector_state, mock_logger_state) = expand_input_with_mocks(
        input,
        &vec![],
        UniformRandomRuleSelector::new(),
        1
    );

    if let Err(Error{kind: ErrorKind::NontermExpansionFailed(0), state}) = expansion_result {
        assert_eq!(state, vec![Symbol::Nonterminal(0)]);

        assert_eq!(
            mock_rule_selector_state.borrow().select_rule_calls,
            vec![
                (vec![], 0)
            ]
        );

        assert!(mock_logger_state.borrow().on_nonterm_expanded_calls.is_empty());
        assert_eq!(mock_logger_state.borrow().on_nonterm_expansion_failed_calls, vec![0]);
        assert!(mock_logger_state.borrow().on_input_fully_expanded_calls.is_empty());
        assert!(mock_logger_state.borrow().on_max_iterations_reached_calls.is_empty());
    } else {
        panic!("expected Error with kind NontermExpansionFailed(0)");
    }
}

#[test]
fn expand_input_err_max_iterations_reached() {
    const MAX_ITERATIONS: usize = 1024;

    let input: Vec<Symbol<_, i32>> = vec![Symbol::Nonterminal(0)];

    let rules = vec![
        Rule::new(0, vec![Symbol::Nonterminal(0)])
    ];

    let (expansion_result, mock_rule_selector_state, mock_logger_state) = expand_input_with_mocks(
        input,
        &rules,
        UniformRandomRuleSelector::new(),
        MAX_ITERATIONS
    );

    if let Err(Error{kind: ErrorKind::MaxIterationsReached(MAX_ITERATIONS), state}) = expansion_result {
        assert_eq!(state, vec![Symbol::Nonterminal(0)]);

        assert_eq!(
            mock_rule_selector_state.borrow().select_rule_calls,
            (0..MAX_ITERATIONS)
                .map(|_| (rules.clone(), 0))
                .collect::<Vec<(Vec<Rule<_, _>>, _)>>()
        );

        assert_eq!(
            mock_logger_state.borrow().on_nonterm_expanded_calls,
            (0..MAX_ITERATIONS)
                .map(|_| (0, rules[0].clone()))
                .collect::<Vec<(_, Rule<_, _>)>>()
        );
        assert!(mock_logger_state.borrow().on_nonterm_expansion_failed_calls.is_empty());
        assert!(mock_logger_state.borrow().on_input_fully_expanded_calls.is_empty());
        assert_eq!(
            mock_logger_state.borrow().on_max_iterations_reached_calls,
            vec![
                (vec![Symbol::Nonterminal(0)], MAX_ITERATIONS)
            ]
        );
    } else {
        panic!("expected Error with kind MaxIterationsReached(MAX_ITERATIONS)");
    }
}

//
// Service
//

fn expand_input_with_mocks<Nt, T, RS>(
    input:               Vec<Symbol<Nt, T>>,
    rules:               &[Rule<Nt, T>],
    inner_rule_selector: RS,
    max_iterations:      usize
) -> (Result<Nt, T>, Rc<RefCell<MockRuleSelectorState<Nt, T>>>, Rc<RefCell<MockLoggerState<Nt, T>>>)
    where Nt: NonterminalValue,
          T:  TerminalValue,
          RS: RuleSelector<Nt, T>
{
    let (mock_rule_selector_state, mock_logger_state) = make_helper_mock_states();

    let expansion_result = expand_input(
        input,
        rules,
        &MockRuleSelector::new(inner_rule_selector, Rc::clone(&mock_rule_selector_state)),
        &mut MockLogger::new(Rc::clone(&mock_logger_state)),
        max_iterations
    );

    (expansion_result, mock_rule_selector_state, mock_logger_state)
}

fn make_helper_mock_states<Nt, T>() -> (Rc<RefCell<MockRuleSelectorState<Nt, T>>>, Rc<RefCell<MockLoggerState<Nt, T>>>) {
    (
        Rc::new(RefCell::new(MockRuleSelectorState::new())),
        Rc::new(RefCell::new(MockLoggerState::new()))
    )
}

//
// Service types
//

//
// AlwaysFirstRuleSelector: RuleSelector<Nt, T>
//

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

impl Default for AlwaysFirstRuleSelector {
    fn default() -> Self {
        Self::new()
    }
}

impl AlwaysFirstRuleSelector {
    fn new() -> Self {
        Self
    }
}

//
// MockRuleSelector<Nt, T>: RuleSelector<Nt, T>
//

struct MockRuleSelector<Nt, T, RS>
    where Nt: NonterminalValue,
          T:  TerminalValue,
          RS: RuleSelector<Nt, T>
{
    inner: RS,
    state: Rc<RefCell<MockRuleSelectorState<Nt, T>>>
}

impl<Nt, T, RS> RuleSelector<Nt, T> for MockRuleSelector<Nt, T, RS>
    where Nt: NonterminalValue,
          T:  TerminalValue,
          RS: RuleSelector<Nt, T>
{
    fn select_rule<'a>(&self, all_rules: &'a [Rule<Nt, T>], nonterm_value: &Nt) -> Option<&'a Rule<Nt,T>>
    {
        self.state.borrow_mut().select_rule_calls.push((Vec::from(all_rules), *nonterm_value));

        self.inner.select_rule(all_rules, nonterm_value)
    }

    fn find_matching_rules<'a>(&self, _all_rules: &'a [Rule<Nt, T>], _nonterm_value: &Nt) -> Vec<&'a Rule<Nt, T>>
    {
        panic!("did not expect find_matching_rules() to be called directly")
    }

    fn select_matching_rule<'a>(&self, _matching_rules: &[&'a Rule<Nt, T>]) -> Option<&'a Rule<Nt, T>> {
        panic!("did not expect select_matching_rule() to be called directly")
    }
}

impl<Nt, T, RS> MockRuleSelector<Nt, T, RS>
    where Nt: NonterminalValue,
          T:  TerminalValue,
          RS: RuleSelector<Nt, T>
{
    fn new(inner: RS, state: Rc<RefCell<MockRuleSelectorState<Nt, T>>>) -> Self {
        Self{inner, state}
    }
}

//
// MockRuleSelectorState<Nt, T>
//

struct MockRuleSelectorState<Nt, T>
{
    select_rule_calls: Vec<(Vec<Rule<Nt, T>>, Nt)>
}

impl<Nt, T> Default for MockRuleSelectorState<Nt, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Nt, T> MockRuleSelectorState<Nt, T> {
    fn new() -> Self {
        Self{
            select_rule_calls: Vec::new()
        }
    }
}

//
// MockLogger<Nt, T>: ExpansionLogger<Nt, T>
//

struct MockLogger<Nt, T>
{
    state: Rc<RefCell<MockLoggerState<Nt, T>>>
}

impl<Nt, T> ExpansionLogger<Nt, T> for MockLogger<Nt, T>
    where Nt: NonterminalValue,
          T:  TerminalValue
{
    fn on_nonterm_expanded(&mut self, expanded_nonterm_value: &Nt, rule: &Rule<Nt, T>) {
        self.state.borrow_mut().on_nonterm_expanded_calls.push((*expanded_nonterm_value, rule.clone()));
    }

    fn on_nonterm_expansion_failed(&mut self, expanded_nonterm_value: &Nt) {
        self.state.borrow_mut().on_nonterm_expansion_failed_calls.push(*expanded_nonterm_value);
    }

    fn on_input_fully_expanded(&mut self, expansion_result: &[T]) {
        self.state.borrow_mut().on_input_fully_expanded_calls.push(Vec::from(expansion_result));
    }

    fn on_max_iterations_reached(&mut self, current_state: &[Symbol<Nt, T>], iterations: usize) {
        self.state.borrow_mut().on_max_iterations_reached_calls.push((Vec::from(current_state), iterations));
    }
}

impl<Nt, T> MockLogger<Nt, T> {
    fn new(state: Rc<RefCell<MockLoggerState<Nt, T>>>) -> Self {
        Self{state}
    }
}

//
// MockLoggerState<Nt, T>
//

struct MockLoggerState<Nt, T>
{
    on_nonterm_expanded_calls:         Vec<(Nt, Rule<Nt, T>)>,
    on_nonterm_expansion_failed_calls: Vec<Nt>,
    on_input_fully_expanded_calls:     Vec<Vec<T>>,
    on_max_iterations_reached_calls:   Vec<(Vec<Symbol<Nt, T>>, usize)>
}

impl<Nt, T> Default for MockLoggerState<Nt, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Nt, T> MockLoggerState<Nt, T> {
    fn new() -> Self {
        Self{
            on_nonterm_expanded_calls:         Vec::new(),
            on_nonterm_expansion_failed_calls: Vec::new(),
            on_input_fully_expanded_calls:     Vec::new(),
            on_max_iterations_reached_calls:   Vec::new()
        }
    }
}
