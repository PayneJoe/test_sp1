#[cfg(test)]
mod tests {
    use vault_prover_guest::DISPUTE_ELF;
    use sp1_sdk::ProverClient;
    use sp1_sdk::HashableKey;

    #[test]
    fn it_works() {
        sp1_sdk::utils::setup_logger();

        let prover_client = ProverClient::from_env();
        let (_, vk) = prover_client.setup(DISPUTE_ELF);
        println!("#### {:?}", vk.bytes32());
    }
}
