from random import randint
for i in range(1000):
    seed = randint(0, 1 << 63)
    print(seed)
