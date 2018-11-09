use error_chain::*;
pub(crate) use error_chain::bail;

use crate::parser::Rule;

error_chain! {
    errors {}

    foreign_links {
        Json(serde_json::Error) #[doc = "JSON (de)serialization error"];
        Pest(pest::error::Error<Rule>) #[doc = "Parser error"];
        ParseIntError(std::num::ParseIntError) #[doc = "Integer parser error"];
        ParseFloatError(std::num::ParseFloatError) #[doc = "Float parser error"];
    }
}
