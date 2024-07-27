import numpy as np
from timeit import timeit

Q = 655_360_001

N = 100
K = 500 # randomness length
L = 100 # message length

# l_\infty norm bound for randomness
beta = 100


# Setup
A_1 = np.concatenate((np.eye(N), np.random.randint(low=0, high=Q, size=[N, K - N])), axis=1)
A_2 = np.concatenate((np.zeros((L, N)), np.eye(L), np.random.randint(low=0, high=Q, size=[L, K -  N - L])), axis=1)
A = np.concatenate((A_1, A_2), axis=0)

# Commit
message = np.random.randint(low=0, high=Q, size=L)
randomness = np.random.randint(low=0, high=beta, size=K)

def compute_commitment():
    return np.remainder(A.dot(randomness), Q) + np.concatenate((np.zeros(N), message), axis=0)


N_REPETITIONS = 1
result = timeit(
    "compute_commitment()", globals=locals(), number=N_REPETITIONS
) /  N_REPETITIONS
print(f"Took {result} seconds")
