import time

def fib(n):
    if n == 0:
        return 0
    if n == 1:
        return 1

    return fib(n - 1) + fib(n - 2)





start = time.perf_counter()
n = 50
#n = 10

for i in range(n):
    print("Fib({}) = {}".format(i+1, fib(i + 1)))



end = time.perf_counter()
print(f"Totale tijd gespendeerd: {end - start:.6f}")
