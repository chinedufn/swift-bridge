use quote::ToTokens;
use std::fmt::Display;

mod parse_error;
pub(crate) use self::parse_error::ParseError;

pub(crate) struct ParseErrors {
    errors: Vec<ParseError>,
}

impl ParseErrors {
    pub fn new() -> Self {
        Self { errors: vec![] }
    }

    pub fn push(&mut self, error: ParseError) {
        self.errors.push(error);
    }

    pub fn combine_all(mut self) -> Result<(), syn::Error> {
        if self.errors.len() == 0 {
            return Ok(());
        }

        let mut errors = self.errors.drain(..);

        let mut combined_errors: syn::Error = errors.next().unwrap().into();

        while let Some(next) = errors.next() {
            combined_errors.combine(next.into());
        }

        Err(combined_errors)
    }
}

#[cfg(test)]
impl std::ops::Deref for ParseErrors {
    type Target = Vec<ParseError>;

    fn deref(&self) -> &Self::Target {
        &self.errors
    }
}
