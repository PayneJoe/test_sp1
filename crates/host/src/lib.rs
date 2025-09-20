#[cfg(test)]
mod tests {
    use sp1_sdk::{HashableKey, ProverClient};
    use vault_prover_guest::{DISPUTE_ELF, PEGOUT_ELF};

    #[test]
    fn it_works() {
        sp1_sdk::utils::setup_logger();

        let prover_client = ProverClient::from_env();
        let (_, dispute_vk) = prover_client.setup(DISPUTE_ELF);
        let (_, pegout_vk) = prover_client.setup(PEGOUT_ELF);
        println!(
            "#### dispute VK: {:?} \n pegout VK: {:?}",
            dispute_vk.bytes32(),
            pegout_vk.bytes32()
        );
    }
}
