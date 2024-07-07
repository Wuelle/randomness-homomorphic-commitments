use qfall_math::{
    integer::Z, integer_mod_q::MatZq, traits::GetNumRows
};

// FIXME: I want this to be a usize
const LATTICE_DIMENSION: u32 = 500;
const SHORT: u32 = 50;

pub struct Commitment {
    com: MatZq,
}

pub struct PublicKey {
    a_1: MatZq,
    a_2: MatZq,
}

impl Commitment {
    #[must_use]
    pub fn create(value: &MatZq, public_key: &PublicKey) -> (Self, MatZq) {
        assert!(value.is_column_vector());

        let randomness = MatZq::new(public_key.a_2.get_num_rows(), 1, LATTICE_DIMENSION);

        let commitment = Self {
            com: &public_key.a_1 * value + &public_key.a_2 * &randomness,
        };

        (commitment, randomness)
    }

    #[must_use]
    pub fn is_valid(&self, value: &MatZq, randomness: &MatZq, public_key: &PublicKey) -> bool {
        if !value.is_column_vector() || !randomness.is_column_vector() {
            return false;
        }

        // Verify that the vector is short
        let max_length: Z = Z::from(SHORT);
        if value.norm_eucl_sqrd().unwrap() >= max_length {
            return false;
        }
        if value.norm_eucl_sqrd().unwrap() >= max_length {
            return false;
        }

        let recomputed_commitment = &public_key.a_1 * value + &public_key.a_2 * randomness;

        self.com == recomputed_commitment
    }
}

impl PublicKey {
    #[must_use]
    pub fn random<N, M, Q>(n: N, m: M, q: Q) -> Self
    where
        N: Into<Z>,
        M: Into<Z>,
        Q: Into<Z>,
    {
        let n = n.into();
        let m = m.into();
        let q = q.into();

        // In theory, A_1 and A_2 could have a different number of columns, but lets not worry about
        // that.
        Self {
            a_1: MatZq::sample_uniform(&n, &m, &q),
            a_2: MatZq::sample_uniform(&n, &m, &q),
        }
    }
}