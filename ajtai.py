# Implicit in https://dl.acm.org/doi/pdf/10.1145/237814.237838
# Explicitly defined in https://link.springer.com/chapter/10.1007/978-3-031-15979-4_3

import numpy as np
from timeit import timeit

Q = 655_360_001

# Higher message/randomness dimensions weaken security
# Higher commitment dimension strengthens security
MESSAGE_DIM = 200
RANDOMNESS_DIM = 200
COMMITMENT_DIM = 500

# SIS parameter, messages & randomness longer than this are rejected
BETA = 16

def random_short_vector(size):
    vector = np.random.randint(low=0, high=BETA, size=size)

    # Hack to generate sufficiently short vectors
    while np.linalg.norm(vector) > BETA:
        position = np.random.random_integers(size - 1)
        vector[position] /= 2
    return vector


# Setup
A_1 = np.random.randint(low=0, high=Q, size=[COMMITMENT_DIM, MESSAGE_DIM])
A_2 = np.random.randint(low=0, high=Q, size=[COMMITMENT_DIM, RANDOMNESS_DIM])

# Commit
message = random_short_vector(MESSAGE_DIM)
randomness = random_short_vector(RANDOMNESS_DIM) 

def compute_commitment():
    return A_1 * message + A_2 * randomness


N_REPETITIONS = 128
result = (
    timeit(
        "compute_commitment()",
        setup="from __main__ import compute_commitment",
        number=N_REPETITIONS,
    )
    / N_REPETITIONS
)
print(f"Took {result:.6f} seconds")
