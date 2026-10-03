//! Public, read-only connection metadata. No RPC endpoints or payout defaults.
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
    config
        .instances
        .iter()
        .enumerate()
        .filter_map(|(i, instance)| {
            // Legacy env-based modes are not reliably known here. Do not advertise
            // them as native or guess a payout network.
            let mode = instance.mining_mode?;
            if mode == MiningMode::Sedra { return None; }
            let port = instance.stratum_port.rsplit(':').next()?.parse::<u16>().ok()?;
            if port == 0 {
                return None;
            }
            let parent_prefix = match mode {
                MiningMode::Native => None,
                _ => {
                    let parent = instance.parent.as_ref()?;
                    if parent.kind != mode || parent.validate().is_err() {
                        return None;
                    }
                    Some(parent.payout_address.split_once(':')?.0.to_owned())
                }
            };
            Some(MiningConnection { instance: crate::log_colors::LogColors::format_instance_id(i + 1), mode, port, parent_prefix })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn publishes_effective_ports_and_parent_network_without_private_config() {
        let config = BridgeConfig::from_yaml(include_str!("../../ops/multimining/devnet.example.json")).unwrap();
        let options = connections(&config);
        assert_eq!(options.len(), 2);
        assert_eq!(options[0].instance, "[Instance 1]");
        assert_eq!(options[0].mode, MiningMode::Native);
        assert_eq!(options[1].parent_prefix.as_deref(), Some("kaspadev"));
        let json = serde_json::to_string(&options).unwrap();
        assert!(!json.contains("endpoint"));
        assert!(!json.contains("payout_address"));
        assert_eq!(options[1].port, 5556);
    }

    #[test]
    fn does_not_guess_legacy_mode_or_advertise_invalid_parent() {
        let mut config = BridgeConfig::default();
        assert!(connections(&config).is_empty());
        config.instances[0].mining_mode = Some(MiningMode::Sedra);
        assert!(connections(&config).is_empty());
    }
}
