use {
    criterion::{criterion_group, criterion_main, BatchSize, Criterion},
    crypto::secp256k1_verify,
    k256::ecdsa::{signature::Signer, Signature, SigningKey, VerifyingKey},
    rand::{CryptoRng, Rng},
    random_string::charsets::ALPHANUMERIC,
};

const MSG_LEN: usize = 20;

struct TestCase {
    prehash_msg_bytes: Vec<u8>,
    vk_bytes:          Vec<u8>,
    sig_bytes:         Vec<u8>,
    valid:             bool,
}

fn create_random_test_case<R: Rng + CryptoRng>(rng: &mut R) -> TestCase {
    let prehash_msg = random_string::generate(MSG_LEN, ALPHANUMERIC);
    let sk = SigningKey::random(rng);
    let vk = VerifyingKey::from(&sk);
    let sig: Signature = sk.sign(prehash_msg.as_bytes());

    let valid = rng.gen();
    let prehash_msg = if valid {
        prehash_msg
    } else {
        let mut msg;
        loop {
            msg = random_string::generate(MSG_LEN, ALPHANUMERIC);
            if prehash_msg != msg {
                break;
            }
        }
        msg
    };

    TestCase {
        prehash_msg_bytes: prehash_msg.into_bytes(),
        vk_bytes:          vk.to_sec1_bytes().to_vec(),
        sig_bytes:         sig.to_bytes().to_vec(),
        valid,
    }
}

fn bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("secp256k1_verify");

    g.bench_function("overhead included", |b| {
        b.iter(|| {
            let tc = create_random_test_case(&mut rand::thread_rng());
            let valid = secp256k1_verify(&tc.prehash_msg_bytes, &tc.vk_bytes, &tc.sig_bytes).unwrap();
            assert_eq!(valid, tc.valid);
        });
    });

    g.bench_function("overhead excluded", |b| {
        b.iter_batched(
            || create_random_test_case(&mut rand::thread_rng()),
            |tc| {
                let valid = secp256k1_verify(&tc.prehash_msg_bytes, &tc.vk_bytes, &tc.sig_bytes).unwrap();
                assert_eq!(valid, tc.valid);
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
