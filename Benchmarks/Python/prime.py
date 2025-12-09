import time

def isprime(n):
    if n < 2:
        return False
    if n%2==0:
        return False
    i = 3
    while (i*i <= n):
        if n % i == 0:
            return False;
        else:
            i+=2
    return True



start = time.perf_counter()   
#x = 20000
x = 20000000

ans = 1
for i in range(x):
    if isprime(i) == True:
        ans += 1;


print(ans)


end = time.perf_counter()
print(f"Runtime: {end - start:.6f} seconds")

