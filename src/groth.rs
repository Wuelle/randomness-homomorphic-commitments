use std::ops;

use bn::Group;

pub type Message<const N: usize> = [bn::G2; N];

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Commitment(bn::Gt, bn::Gt);

#[derive(Clone, Copy)]
pub struct OpeningInfo<const N: usize> {
    message: [bn::G2; N],
    r: bn::G2,
    s: bn::G2,
}

#[derive(Clone, Copy)]
pub struct PublicKey<const N: usize> {
    g: [bn::G1; N],
    h: [bn::G1; N],
    /// `g_s, g_r, h_s, h_r`
    randomness_coefficients: [bn::G1; 4],
}

impl Commitment {
    #[must_use]
    pub fn create<T: ancient_rand::Rng, const N: usize>(
        message: Message<N>,
        public_key: PublicKey<N>,
        mut rng: T,
    ) -> (Self, OpeningInfo<N>) {
        let r = bn::G2::one() * bn::Fr::random(&mut rng);
        let s = bn::G2::one() * bn::Fr::random(&mut rng);

        let opening_info = OpeningInfo { message, s, r };

        let commitment = Self::compute_inner(message, public_key, r, s);

        (commitment, opening_info)
    }

    #[must_use]
    fn compute_inner<const N: usize>(message: Message<N>, public_key: PublicKey<N>, r: bn::G2, s: bn::G2) -> Self {
        let c = public_key
            .g
            .iter()
            .zip(message)
            .map(|(g, m)| bn::pairing(*g, m))
            .fold(bn::Gt::one(), ops::Mul::mul)
            * bn::pairing(public_key.randomness_coefficients[0], r)
            * bn::pairing(public_key.randomness_coefficients[1], s);

        let d = public_key
            .h
            .iter()
            .zip(message)
            .map(|(h, m)| bn::pairing(*h, m))
            .fold(bn::Gt::one(), ops::Mul::mul)
            * bn::pairing(public_key.randomness_coefficients[2], r)
            * bn::pairing(public_key.randomness_coefficients[3], s);

        Self(c, d)
    }

    #[must_use]
    pub fn is_valid<const N: usize>(
        &self,
        public_key: PublicKey<N>,
        opening_info: OpeningInfo<N>,
    ) -> bool {
        let recomputed = Self::compute_inner(opening_info.message, public_key, opening_info.r, opening_info.s);
        *self == recomputed
    }
}

impl<const N: usize> PublicKey<N> {
    #[must_use]
    pub fn random<T: ancient_rand::Rng>(mut rng: T) -> Self {
        let mut sample = |_| bn::G1::one() * bn::Fr::random(&mut rng);

        Self {
            g: std::array::from_fn(&mut sample),
            h: std::array::from_fn(&mut sample),
            randomness_coefficients: std::array::from_fn(&mut sample),
        }
    }
}
