//
// Interface traits
//

//
// NonterminalValue: Clone + PartialEq
//

/// Describes requirements for types of non-terminal symbol values.
///
/// Any type that is cloneable and comparable by `==` automatically satisfies `NonterminalValue`.
pub trait NonterminalValue: Clone + PartialEq {
    // Empty
}

impl<Nt> NonterminalValue for Nt
    where Nt: Clone + PartialEq {
    // Empty
}

//
// TerminalValue: Clone
//

/// Describes requirements for types of terminal symbol values.
///
/// Any cloneable type automatically satisfies `TerminalValue`.
pub trait TerminalValue: Clone {
    // Empty
}

impl<T> TerminalValue for T
    where T: Clone {
    // Empty
}

//
// Interface types
//

//
// enum Symbol<Nt, T>: Debug + Clone + Copy + PartialEq
//

/// Used to describe non-terminal and terminal symbols in [`Rule`](struct.Rule.html)s
/// and grammar input sequences for [`Expander::expand()`](struct.Expander.html#method.expand).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Symbol<Nt, T> {
    /// Variant for non-terminal symbols - ones which can be further expanded,
    /// can appear on left-hand side of rules and cannot appear in a successful expansion result.
    Nonterminal(Nt),
    /// Variant for terminal symbols - ones which will not be replaced during expansion,
    /// cannot appear on left-hand side of rules and will be the only ones in a successful expansion result.
    Terminal(T)
}

impl<Nt, T> Symbol<Nt, T> {
    pub fn is_terminal(&self) -> bool {
        if let Self::Terminal(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_nonterminal(&self) -> bool {
        !self.is_terminal()
    }

    pub fn unwrap_nonterm(self) -> Nt {
        self.expect_nonterm(
            "unwrap_nonterm() must be used only on non-terminal symbols"
        )
    }

    pub fn expect_nonterm(self, message: &'static str) -> Nt {
        if let Self::Nonterminal(value) = self {
            value
        } else {
            panic!(message);
        }
    }

    pub fn unwrap_term(self) -> T {
        self.expect_term(
            "unwrap_term() must be used only on terminal symbols"
        )
    }

    pub fn expect_term(self, message: &'static str) -> T {
        if let Self::Terminal(value) = self {
            value
        } else {
            panic!(message);
        }
    }
}

//
// struct Rule<Nt, T>: Debug + Clone + PartialEq
//

/// Describes a rule (or production) of a context-free grammar.
#[derive(Debug, Clone, PartialEq)]
pub struct Rule<Nt, T> {
    /// Left-hand side of the rule (a single non-terminal symbol value).
    pub pattern:     Nt,
    /// Right-hand side of the rule (any sequence of symbols,
    /// with which to replace the encountered `pattern`).
    pub replacement: Vec<Symbol<Nt, T>>
}

impl<Nt, T> Rule<Nt, T> {
    pub fn new(pattern: Nt, replacement: Vec<Symbol<Nt, T>>) -> Self {
        Self{
            pattern,
            replacement
        }
    }
}
