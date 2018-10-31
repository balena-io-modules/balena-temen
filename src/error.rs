use crate::parser::Rule;
pub use error_chain::bail;
use error_chain::*;

error_chain! {
    errors {}

    foreign_links {
        Json(serde_json::Error) #[doc = "JSON (de)serialization error"];
        Pest(pest::error::Error<Rule>) #[doc = "Parser error"];
    }
}
