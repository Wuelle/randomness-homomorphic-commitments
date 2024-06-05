use curve25519_dalek::{constants, ristretto::RistrettoPoint, scalar::Scalar, traits::MultiscalarMul};

#[derive(Clone, Copy, Debug)]
pub struct Commitment(RistrettoPoint);

#[derive(Clone, Copy, Debug)]
pub struct OpeningInfo {
    value: Scalar,
    randomness: Scalar,
}

#[derive(Clone, Copy, Debug)]
pub struct PublicKey {
    h: RistrettoPoint,
}

impl Commitment {
    #[must_use]
    pub fn create<T: rand::RngCore + rand::CryptoRng>(value: Scalar, public_key: PublicKey, mut rng: T) -> (Self, OpeningInfo) {
        let randomness = Scalar::random(&mut rng);
        let c = RistrettoPoint::multiscalar_mul(
            &[value, randomness],
            &[constants::RISTRETTO_BASEPOINT_POINT, public_key.h],
        );
        let opening_info = OpeningInfo {value, randomness};

        (Commitment(c), opening_info)
    }

    #[must_use]
    pub fn is_valid(&self, public_key: PublicKey, opening_info: OpeningInfo) -> bool {
        // Recompute commitment and check for equality
        let c = RistrettoPoint::multiscalar_mul(
            &[opening_info.value, opening_info.randomness],
            &[constants::RISTRETTO_BASEPOINT_POINT, public_key.h],
        );

        c == self.0
    }
}

impl PublicKey {
    #[must_use]
    pub fn random<T: rand::RngCore + rand::CryptoRng>(mut rng: T) -> Self {
        Self {
            h: constants::RISTRETTO_BASEPOINT_POINT * Scalar::random(&mut rng)
        }
    }
}