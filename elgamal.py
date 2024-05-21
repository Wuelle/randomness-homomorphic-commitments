import random

class CyclicGroup:
    def __init__(self, g=5, p=2333, s=42):
        self.g = g
        self.q = 2 * p + 1
        self.h = pow(self.g, s, self.q)

    def random_element(self):
        return random.randint(0, self.q)


def generate_commitment(message, randomness, group):
    return (
        pow(group.g, message, group.q),
        (message * group.h ** randomness) % group.q
    )

def verify_commitment(commitment, message, randomness, group):
    regenerated_commitment = generate_commitment(message, randomness, group)
    return commitment == regenerated_commitment

group = CyclicGroup()

randomness = group.random_element()
message = group.random_element()
commitment = generate_commitment(message, randomness, group)

assert verify_commitment(commitment, message, randomness, group)

# Try and open the commitment to another message
bogus_message = (message + 1) % group.q
assert not verify_commitment(commitment, bogus_message, randomness, group)
