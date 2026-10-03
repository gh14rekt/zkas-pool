//! Public native-port connection metadata. No RPC endpoints or payout defaults.
use crate::{BridgeConfig, parent::MiningMode};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MiningConnection {
    pub instance: String,
    pub mode: MiningMode,
    pub port: u16,
    pub parent_prefix: Option<String>,
}

pub fn connections(config: &BridgeConfig) -> Vec<MiningConnection> {
    config.instances.iter().enumerate().filter_map(|(i, instance)| {
        let mode = instance.mining_mode?;
        if mode != MiningMode::Native { return None; }
        let port = instance.stratum_port.rsplit(':').next()?.parse::<u16>().ok()?;
        if port == 0 { return None; }
        Some(MiningConnection {
            instance: crate::log_colors::LogColors::format_instance_id(i + 1),
            mode, port, parent_prefix: None,
        })
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn publishes_native_port_without_private_configuration() {
        let mut config = BridgeConfig::default();
        assert!(connections(&config).is_empty());
        config.instances[0].mining_mode = Some(MiningMode::Native);
        config.instances[0].stratum_port = "127.0.0.1:5555".into();
        let options = connections(&config);
        assert_eq!(options.len(), 1);
        assert_eq!(options[0].port, 5555);
        assert!(options[0].parent_prefix.is_none());
        let json = serde_json::to_string(&options).unwrap();
        assert!(!json.contains("endpoint"));
        assert!(!json.contains("payout_address"));
    }
}
