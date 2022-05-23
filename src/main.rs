/* ==========================================
     __  ___  _____        _____  ___    __
  /\ \ \/___\/__   \/\ /\  \_   \/___\/\ \ \
 /  \/ //  //  / /\/ / \ \  / /\//  //  \/ /
/ /\  / \_//  / /  \ \_/ /\/ /_/ \_// /\  /
\_\ \/\___/   \/    \___/\____/\___/\_\ \/
========================================== */

use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct MyConfig {
    version: String,
    api_key: String,
}

/// `MyConfig` implements `Default`
impl ::std::default::Default for MyConfig {
    fn default() -> Self { Self { version: "0".into(), api_key: "".into() } }
}

fn main() -> Result<(), confy::ConfyError> {
    let cfg = confy::load("notuion")?;
    dbg!(cfg);
    Ok(())
}
