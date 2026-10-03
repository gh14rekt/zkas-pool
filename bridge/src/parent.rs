//! Explicit per-port mode selection. Native mining needs no parent.
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MiningMode {
    #[default]
    Native,
    Kaspa,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn native_mode_round_trips() {
        let mode: MiningMode = serde_json::from_str("\"native\"").unwrap();
        assert_eq!(mode, MiningMode::Native);
        assert_eq!(serde_json::to_string(&mode).unwrap(), "\"native\"");
    }
}
