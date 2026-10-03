//! Private-devnet integration probe: real CPU PoW, unmodified node validators.
use anyhow::{Result, ensure};
use kaspa_consensus_core::block::Block;
use kaspa_pow::State;
use kaspa_stratum_bridge::{KaspaApi, parent::ParentConfig};
use num_bigint::BigUint;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let args: Vec<String> = std::env::args().collect();
    ensure!(
        args.len() == 4 || (args.len() == 5 && matches!(args[4].as_str(), "--expect-fallback" | "--lifecycle")),
        "usage: multimining_probe POOL_CONFIG_JSON ZKAS_TEST_ADDRESS MODE [--expect-fallback|--lifecycle]"
    );
    let lifecycle = args.get(4).is_some_and(|a| a == "--lifecycle");
    let fallback = args.get(4).is_some_and(|a| a == "--expect-fallback");
    ensure!(args[2].starts_with("zkasdev:"), "probe requires private devnet wallet");
    let config: serde_json::Value = serde_json::from_slice(&std::fs::read(&args[1])?)?;
    let instance = config["instances"].as_array().unwrap().iter().find(|i| i["mining_mode"] == args[3]).unwrap();
    let parent: Option<ParentConfig> = instance.get("parent").map(|p| serde_json::from_value(p.clone())).transpose()?;
    let (_tx, rx) = tokio::sync::watch::channel(false);
    let api = KaspaApi::new_with_parent(config["kaspad_address"].as_str().unwrap().to_owned(), None, rx, None, parent.clone()).await?;
    let payee = parent.map(|p| p.payout_address);
    let block = api.get_block_template(&args[2], "probe", "", 9001, 1, payee.clone(), 42).await?;
    let auxiliary = api.merged_fc_target(&block);
    ensure!(auxiliary.is_some() == (args[3] != "native" && !fallback), "unexpected native/MM job type");
    if lifecycle {
        println!("PAUSE_PARENT");
        std::io::stdin().read_line(&mut String::new())?;
        let native = api.get_block_template(&args[2], "probe", "", 9001, 2, payee.clone(), 42).await?;
        ensure!(api.merged_fc_target(&native).is_none() && native.header.aux_pow.is_none(), "outage did not return native work");
        println!("NATIVE_FALLBACK_VERIFIED");
        ensure!(api.merged_fc_target(&block).is_some(), "old MM job lost its identity");
    }
    let target = auxiliary.unwrap_or_else(|| kaspa_stratum_bridge::hasher::calculate_target(block.header.bits as u64));
    println!("mode={} bits={:08x} target={:x}", args[3], block.header.bits, target);
    let state = State::new(&block.header);
    let deadline = Instant::now() + Duration::from_secs(90);
    let mut header = (*block.header).clone();
    for nonce in 0..u64::MAX {
        if nonce % 4096 == 0 {
            ensure!(Instant::now() < deadline, "CPU proof deadline exceeded");
        }
        let pow = state.calculate_pow(nonce);
        if BigUint::from_bytes_be(&pow.to_be_bytes()) <= target {
            header.nonce = nonce;
            header.finalize();
            let solved = Block { header: Arc::new(header), transactions: block.transactions.clone() };
            println!("nonce={nonce} parent={:?}", api.submit_merged_parent_if_solved(&solved).await);
            let result = api.submit_block(solved).await?;
            println!("zkas={result:?}");
            ensure!(matches!(result, kaspa_stratum_bridge::kaspaapi::BlockSubmitOutcome::Accepted(_)), "ZKas did not accept proof");
            if lifecycle {
                println!("RESUME_PARENT");
                std::io::stdin().read_line(&mut String::new())?;
                tokio::time::sleep(Duration::from_secs(4)).await;
                let recovered = api.get_block_template(&args[2], "probe", "", 9001, 3, payee.clone(), 42).await?;
                ensure!(api.merged_fc_target(&recovered).is_some(), "parent did not recover");
                println!("LATE_MM_AND_RECOVERY_VERIFIED");
            }
            return Ok(());
        }
    }
    anyhow::bail!("nonce space exhausted")
}
