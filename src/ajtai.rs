use qfall_math::{
    integer::{MatZ, Z},
    integer_mod_q::MatZq,
    rational::{MatQ, Q},
    traits::GetNumRows,
};

// FIXME: I want this to be a usize
const LATTICE_DIMENSION: u32 = 500;
const SHORT: u32 = 50;

pub struct Commitment {
    com: MatZq,
}

pub struct PublicKey {
    /// Columns are the base vectors of the lattice
    lattice_base: MatQ,
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
        let max_length: Q = Q::from(SHORT);
        if length_with_base(&value, &public_key.lattice_base) >= max_length {
            return false;
        }
        if length_with_base(&randomness, &public_key.lattice_base) >= max_length {
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

        let random_base =
            MatZ::sample_uniform(LATTICE_DIMENSION, LATTICE_DIMENSION, -1024, 1024).unwrap();
        let lattice_base = MatQ::from(&random_base).gso();

        // In theory, A_1 and A_2 could have a different number of columns, but lets not worry about
        // that.
        Self {
            lattice_base,
            a_1: MatZq::sample_uniform(&n, &m, &q),
            a_2: MatZq::sample_uniform(&n, &m, &q),
        }
    }
}

#[must_use]
fn length_with_base(vector: &MatZq, base: &MatQ) -> Q {
    debug_assert!(vector.is_column_vector());

    let point = base * MatQ::from(&MatZ::from(vector));
    point.norm_eucl_sqrd().unwrap()
}
