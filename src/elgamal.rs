use curve25519_dalek::{constants, ristretto::RistrettoPoint, scalar::Scalar};

/// `(g^y, mh^y)`
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Commitment(RistrettoPoint, RistrettoPoint);

#[derive(Clone, Copy, Debug)]
pub struct OpeningInfo {
    value: RistrettoPoint,
    randomness: Scalar,
}

#[derive(Clone, Copy, Debug)]
pub struct PublicKey {
    h: RistrettoPoint,
}

impl Commitment {
    #[must_use]
    pub fn create<T: rand::RngCore + rand::CryptoRng>(value: RistrettoPoint, public_key: PublicKey, mut rng: T) -> (Self, OpeningInfo) {
        let randomness = Scalar::random(&mut rng);
        let opening_info = OpeningInfo {value, randomness};

        let randomness_pin = constants::RISTRETTO_BASEPOINT_POINT * randomness;
        let value_term = value + public_key.h * randomness;

        let commitment = Self(randomness_pin, value_term);

        (commitment, opening_info)
    }

    #[must_use]
    pub fn is_valid(&self, public_key: PublicKey, opening_info: OpeningInfo) -> bool {
        // Recompute commitment and check for equality
        let randomness_pin = constants::RISTRETTO_BASEPOINT_POINT * opening_info.randomness;
        if randomness_pin != self.0 {
            return false;
        }

        let value_term = opening_info.value + public_key.h * opening_info.randomness;

        value_term == self.1
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