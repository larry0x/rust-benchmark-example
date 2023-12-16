#![no_std]

use k256::ecdsa::{signature::Verifier, Signature, VerifyingKey};

pub fn secp256k1_verify(
    prehash_msg_bytes: &[u8],
    vk_bytes:          &[u8],
    sig_bytes:         &[u8],
) -> Result<bool, k256::ecdsa::Error> {
    let vk = VerifyingKey::from_sec1_bytes(vk_bytes)?;
    let sig = Signature::try_from(sig_bytes)?;

    Ok(vk.verify(prehash_msg_bytes, &sig).is_ok())
}

// ----------------------------------- tests -----------------------------------

#[cfg(test)]
mod tests {
    use {
        super::*,
        k256::ecdsa::{signature::Signer, SigningKey},
    };

    #[test]
    fn verifying_secp256k1() {
        let prehash_msg_bytes = b"This is a test";

        let sk = SigningKey::random(&mut rand::thread_rng());
        let vk = VerifyingKey::from(&sk);
        let sig: Signature = sk.sign(prehash_msg_bytes);

        assert!(secp256k1_verify(prehash_msg_bytes, &vk.to_sec1_bytes(), &sig.to_bytes()).unwrap());
    }
}
