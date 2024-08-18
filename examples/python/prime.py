def is_prime(k): 
    if k < 2:
        return False
    i = 2;
    while i * i <= k:
        if k % i == 0:
            return False
        i += 1
    return True

is_prime(29996224275833)