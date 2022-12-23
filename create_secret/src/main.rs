use bip39::{Language, Mnemonic, MnemonicType};
fn main() {
    let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
    let phrase = mnemonic.phrase();

    println!("phrase: {}", phrase);

    assert_eq!(phrase.split(" ").count(), 12);
}
