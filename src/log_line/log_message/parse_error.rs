use std::error::Error as ErrorTrait;
use std::fmt;

#[derive(Debug)]
pub struct ParseError;

impl ErrorTrait for ParseError {}
impl fmt::Display for ParseError {
    //This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self)
    }
}
