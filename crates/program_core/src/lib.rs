use serde::{Deserialize, Serialize};

/// Input of dispute test guest program (witness)
#[derive(Serialize, Deserialize, Debug)]
pub struct PegoutTestProofInput {
    pub claimer_pubkey: [u8; 32],
    pub pegin_txid: [u8; 32],
    pub dummy_data: [u8; 32],
}

/// Output of dispute test guest program (public input)
#[derive(Serialize, Deserialize, Debug)]
pub struct PegoutTestProofOutput {
    pub pegin_txid: [u8; 32],
    pub phi: [u8; 32],
}

pub const CLAIMER_PUBKEY: [u8; 32] = [
    185, 138, 127, 184, 204, 0, 112, 72, 98, 91, 100, 70, 173, 73, 161, 179, 167, 34, 223, 140, 28,
    169, 117, 184, 113, 96, 2, 62, 20, 209, 144, 151,
];
pub const VAULT_OP_RETURN_PREFIX: &[u8] = b"btcvaults";
pub const OP_RETURN: u8 = 0x6a;
pub const OP_CHECKSIG: u8 = 0xac;
pub const CLAIM_OP_RETURN_SIZE: usize = 43; // 2 + 9 + 32, whihch carries 41 bytes data
pub const PROOF_SCRIPT_PUBKEY_SIZE: usize = 22; // 2 + 20, which carries 20 bytes data
pub const PROOF_OP_RETURN_SIZE: usize = 82; // 2 + 80, which carries 80 bytes data
                                            // Two phases:
                                            // a) in test phase, 388 is the general gnark Groth16 vk bytes (untweaked) which contributes for 2 public inputs
                                            // b) in production-ready phase, 356 is the tweaked gnark Groth16 vk bytes which contributes for 1 public inputs
pub const UNTWEAKED_GNARK_GROTH16_VK_BYTES: [u8; 388] = [
    173, 77, 154, 167, 227, 2, 217, 223, 65, 116, 157, 85, 7, 148, 157, 5, 219, 234, 51, 251, 177,
    108, 100, 59, 34, 245, 153, 162, 190, 109, 242, 226, 225, 161, 87, 92, 46, 73, 77, 54, 19, 233,
    94, 67, 182, 34, 49, 141, 146, 37, 200, 32, 228, 106, 205, 8, 232, 201, 135, 180, 64, 81, 25,
    91, 201, 103, 3, 47, 203, 247, 118, 209, 175, 201, 133, 248, 136, 119, 241, 130, 211, 132, 128,
    166, 83, 242, 222, 202, 169, 121, 76, 188, 59, 243, 6, 12, 14, 24, 120, 71, 173, 76, 121, 131,
    116, 208, 214, 115, 43, 245, 1, 132, 125, 214, 139, 192, 224, 113, 36, 30, 2, 19, 188, 127,
    193, 61, 183, 171, 153, 142, 147, 147, 146, 13, 72, 58, 114, 96, 191, 183, 49, 251, 93, 37,
    241, 170, 73, 51, 53, 169, 231, 18, 151, 228, 133, 183, 174, 243, 18, 194, 24, 0, 222, 239, 18,
    31, 30, 118, 66, 106, 0, 102, 94, 92, 68, 121, 103, 67, 34, 212, 247, 94, 218, 221, 70, 222,
    189, 92, 217, 146, 246, 237, 216, 229, 115, 154, 115, 214, 87, 232, 50, 163, 54, 121, 25, 119,
    51, 42, 75, 150, 229, 187, 253, 203, 153, 3, 175, 228, 135, 219, 154, 166, 203, 93, 220, 199,
    203, 141, 231, 21, 103, 95, 33, 240, 30, 204, 155, 70, 210, 54, 224, 134, 94, 12, 192, 32, 2,
    69, 33, 153, 130, 105, 132, 95, 116, 230, 3, 255, 65, 244, 186, 12, 55, 254, 44, 175, 39, 53,
    77, 40, 228, 184, 248, 61, 59, 118, 119, 122, 99, 179, 39, 215, 54, 191, 251, 1, 34, 237, 0, 0,
    0, 3, 166, 9, 30, 28, 175, 176, 173, 138, 78, 160, 166, 148, 205, 55, 67, 235, 245, 36, 119,
    146, 51, 219, 115, 76, 69, 29, 40, 181, 138, 169, 117, 142, 134, 28, 63, 208, 253, 61, 162, 93,
    38, 7, 194, 39, 208, 144, 204, 167, 80, 237, 54, 198, 236, 135, 135, 85, 229, 55, 193, 196,
    137, 81, 251, 76, 132, 234, 178, 65, 56, 138, 121, 129, 127, 224, 224, 226, 234, 208, 178, 236,
    79, 253, 236, 81, 161, 96, 40, 222, 224, 32, 99, 79, 209, 41, 231, 28,
];
