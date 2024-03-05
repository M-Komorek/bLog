use std::error::Error;
use std::result::Result;

pub type AppResult<T> = Result<T, Box<dyn Error>>;
