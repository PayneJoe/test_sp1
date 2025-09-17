#[cfg(test)]
mod tests {
    use sp1_sdk::{HashableKey, ProverClient, SP1ProofWithPublicValues, SP1Stdin};
    use sp1_verifier::Groth16Verifier;
    use sp1_verifier::compress_groth16_proof_from_bytes;
    use sp1_verifier::decode_sp1_vkey_hash;
    use sp1_verifier::hash_public_inputs;
    use vault_prover_program_core::UNTWEAKED_GNARK_GROTH16_VK_BYTES;
    use vault_prover_program_core::{CLAIMER_PUBKEY, PegoutTestProofInput};
    use vault_prover_programs::PEGOUT_TEST_PROGRAM_ELF;

    #[test]
    fn test_pegout_snark_proof() {
        sp1_sdk::utils::setup_logger();

        let prover_client = ProverClient::from_env();
        let (pegout_pk, pegout_vk) = prover_client.setup(PEGOUT_TEST_PROGRAM_ELF);
        let mut stdin = SP1Stdin::new();

        // Generate some example keys (in real usage, use actual keys)
        let pegout_proof_input = PegoutTestProofInput {
            claimer_pubkey: CLAIMER_PUBKEY,
            dummy_data: [1u8; 32],
            pegin_txid: [8u8; 32],
        };
        stdin.write(&pegout_proof_input);

        let sp1_proof = prover_client
            .prove(&pegout_pk, &stdin)
            .groth16()
            .run()
            .expect("Dispute proving failed");

        let workspace_dir = std::env::current_dir().expect("get current dir failed");
        let groth16_proof_file = format!(
            "{0}/pegout-sp1-groth16-proof.bin",
            workspace_dir.to_str().unwrap()
        );
        println!("proof is saved in {:?}", groth16_proof_file);
        sp1_proof
            .save(groth16_proof_file)
            .expect("Save groth16 proof failed.");

        let pegout_snark_proof = sp1_proof
            .clone()
            .proof
            .try_as_groth_16()
            .expect("parse groth16 proof failed");
        let tmp_vk_hash = pegout_snark_proof.groth16_vkey_hash;

        let (vk_pegout_hash, public_inputs, raw_snark_proof) = {
            (
                pegout_vk.bytes32().as_bytes().to_vec(),
                sp1_proof.public_values.to_vec(),
                sp1_proof.bytes()[4..260].to_vec(),
            )
        };
        println!(
            "vk_pegout_hash: {:?} ---- {:?} \n\n public input: {:?} \n\n snark proof: {} - {:?}",
            vk_pegout_hash,
            tmp_vk_hash,
            public_inputs,
            raw_snark_proof.len(),
            raw_snark_proof
        );
    }

    #[test]
    fn test_compress_snark_proof() {
        sp1_sdk::utils::setup_logger();

        let prover_client = ProverClient::from_env();
        let (_, pegout_vk) = prover_client.setup(PEGOUT_TEST_PROGRAM_ELF);
        let workspace_dir = std::env::current_dir().expect("get current dir failed");
        let groth16_proof_file = format!(
            "{0}/pegout-sp1-groth16-proof.bin",
            workspace_dir.to_str().unwrap()
        );
        let sp1_proof = SP1ProofWithPublicValues::load(groth16_proof_file).unwrap();
        let (vk_pegout_hash, sp1_public_inputs, raw_snark_proof) = {
            (
                decode_sp1_vkey_hash(&pegout_vk.bytes32()).unwrap(),
                sp1_proof.public_values.to_vec(),
                sp1_proof.bytes()[4..260].to_vec(),
            )
        };
        let compressed_groth16_proof = compress_groth16_proof_from_bytes(&raw_snark_proof).unwrap();
        let result = Groth16Verifier::verify_compressed_gnark_proof(
            &compressed_groth16_proof[..128],
            &[vk_pegout_hash, hash_public_inputs(&sp1_public_inputs)],
            &UNTWEAKED_GNARK_GROTH16_VK_BYTES,
        );
        if result.is_err() {
            println!("verify_compressed_gnark_proof failed: {:?}", result.err());
        } else {
            println!("verify_compressed_gnark_proof success");
        }

        println!(
            "Compressed groth16 proof: {:?}",
            compressed_groth16_proof[..128].to_vec()
        );
    }
}
