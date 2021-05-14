use std::fmt::{
    self,
    Debug,
    Display
};

use crate::grammar::{
    NonterminalValue,
    TerminalValue,
    Symbol
};

//
// Interface
//

//
// ErrorKind<Nt, T>: Debug + Copy
//

/// Enumerates kinds of possible errors during expansion.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Copy)]
pub enum ErrorKind<Nt>
    where Nt: NonterminalValue
{
    NontermExpansionFailed(Nt),
    MaxIterationsReached(usize)
}

impl<Nt> Debug for ErrorKind<Nt>
    where Nt: NonterminalValue
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NontermExpansionFailed(_) =>  write!(
                f, "NontermExpansionFailed(_)"
            ),
            Self::MaxIterationsReached(iterations) => write!(
                f, "MaxIterationsReached({})", iterations
            )
        }
    }
}

//
// Error<Nt, T>: Error
//

/// Used as error variant for [`Result`](type.Result.html).
///
/// The reason for the error can be determined via the [`kind`](struct.Error.html#structfield.kind) field.
pub struct Error<Nt, T>
    where Nt: NonterminalValue,
          T:  TerminalValue
{
    pub state: Vec<Symbol<Nt, T>>,
    pub kind:  ErrorKind<Nt>
}

impl<Nt, T> Debug for Error<Nt, T>
    where Nt: NonterminalValue,
          T:  TerminalValue
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error {{ state, kind: {:?} }}", self.kind)
    }
}

impl<Nt, T> Display for Error<Nt, T>
    where Nt: NonterminalValue,
          T:  TerminalValue
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ErrorKind::NontermExpansionFailed(_) => write!(
                f, "no rule to expand nonterminal symbol"
            ),
            ErrorKind::MaxIterationsReached(iterations) => write!(
                f,
                "reached the maximum {} iterations with {} nonterminal symbols still unexpanded",
                iterations,
                self.state.iter()
                    .filter(|symbol| symbol.is_nonterminal())
                    .count()
            )
        }
    }
}

impl<Nt, T> std::error::Error for Error<Nt, T> 
    where Nt: NonterminalValue,
          T:  TerminalValue
{
    // Default
}

impl<Nt, T> Error<Nt, T>
    where Nt: NonterminalValue,
          T:  TerminalValue
{
    //
    // Interface
    //

    pub fn nonterm_expansion_failed(state: Vec<Symbol<Nt, T>>, expanded_nonterm_value: Nt) -> Self {
        Self::new(state, ErrorKind::NontermExpansionFailed(expanded_nonterm_value))
    }

    #[must_use]
    pub fn max_iterations_reached(state: Vec<Symbol<Nt, T>>, iterations: usize) -> Self {
        Self::new(state, ErrorKind::MaxIterationsReached(iterations))
    }

    //
    // Service
    //

    fn new(state: Vec<Symbol<Nt, T>>, kind: ErrorKind<Nt>) -> Self {
        Self{state, kind}
    }
}
