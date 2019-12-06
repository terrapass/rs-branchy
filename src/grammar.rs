//
// Interface traits
//

//
// NonterminalValue: Copy + PartialEq
//

pub trait NonterminalValue: Copy + PartialEq {
    // Empty
}

impl<Nt> NonterminalValue for Nt
    where Nt: Copy + PartialEq {
    // Empty
}

//
// TerminalValue: Copy
//

pub trait TerminalValue: Copy {
    // Empty
}

impl<T> TerminalValue for T
    where T: Copy {
    // Empty
}

//
// Interface types
//

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
// struct Rule<Nt, T>
//

pub struct Rule<Nt, T> {
    pub pattern:     Nt,
    pub replacement: Vec<Symbol<Nt, T>>
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
