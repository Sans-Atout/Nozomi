use std::fmt;
use error_stack::{self, Context};

#[derive(Debug, Clone, Copy)]
pub struct InputError;

impl fmt::Display for InputError{
    fn fmt(&self, fmt : &mut fmt::Formatter<'_>) -> fmt::Result{
        fmt.write_str("Input Error : invalid given input")
    }
}

impl Context for InputError {}


#[derive(Debug, Clone, Copy)]
pub struct  ProcessError;

impl fmt::Display for ProcessError{
    fn fmt(&self, fmt : &mut fmt::Formatter<'_>) -> fmt::Result{
        fmt.write_str("Process Error : overwritte method fail")
    }
}

impl Context for ProcessError {}
