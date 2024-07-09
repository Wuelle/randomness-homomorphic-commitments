# Implicit in https://dl.acm.org/doi/pdf/10.1145/237814.237838
# Explicitly defined in https://link.springer.com/chapter/10.1007/978-3-031-15979-4_3

import numpy as np
from timeit import timeit

Q = 655_360_001
LATTICE_DIM = 500
MESSAGE_DIM = 10
COMMITMENT_DIM = 15


# Setup
A_1 = np.random.randint(low=0, high=Q, size=[COMMITMENT_DIM, MESSAGE_DIM, LATTICE_DIM])
A_2 = np.random.randint(low=0, high=Q, size=[COMMITMENT_DIM, MESSAGE_DIM, LATTICE_DIM])

# Commit
message = np.random.randint(low=0, high=Q, size=[MESSAGE_DIM, LATTICE_DIM])
randomness = np.random.randint(low=0, high=Q, size=[MESSAGE_DIM, LATTICE_DIM])


def compute_commitment():
    return A_1 * message + A_2 * randomness


N_REPETITIONS = 128
result = timeit(
    "compute_commitment()", setup="from __main__ import compute_commitment", number=N_REPETITIONS
) /  N_REPETITIONS
print(f"Took {result:.3f} seconds")
