use sp_core::{
    sr25519,
    ed25519,
    Pair,
};
use sp_keyring::AccountKeyring;
use sp_runtime::MultiSignature;

use subxt::{
    Client,
    DefaultConfig,
    PairSigner,
    SubstrateExtrinsicParams, extrinsic::Signer,
};

fn get_from_seed(seed: &str) -> ed25519::Pair {
    ed25519::Pair::from_string(&format!("{}", seed), None)
        .expect("static values are valid; qed")
}

pub fn pair_signer(pair: ed25519::Pair) -> PairSigner<DefaultConfig, ed25519::Pair> {
    PairSigner::new(pair)
}


fn main() {
    let pair = get_from_seed("caution juice atom organ advance problem want pledge someone senior holiday very");
    let pairsigner = pair_signer(pair);
    println!("{}", pairsigner.account_id());
    let sig = pairsigner.sign(format!("{}", pairsigner.account_id()).as_bytes());
    if let MultiSignature::Ed25519(x) = sig.clone() {
        println!("0x{:?}", x);
    }
    
    
}
