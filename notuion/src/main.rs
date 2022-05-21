// =============================================
//                   NOTUION
// =============================================
//
// lets start by just cURLing the Notion API

use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct MyConfig {
    version: u8,
    api_key: String,
}

/// `MyConfig` implements `Default`
impl ::std::default::Default for MyConfig {
    fn default() -> Self { Self { version: 0, api_key: "".into() } }
}

fn main() -> Result<(), confy::ConfyError> {
    let cfg = confy::load("notuion")?;
    dbg!(cfg);
    Ok(())
}
