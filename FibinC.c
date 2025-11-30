#include <stdio.h>
#include <time.h>



long long fib(long long n){

    if(n == 0)
        return 0;
    if(n == 1)
        return 1;
    return fib(n - 1) + fib(n - 2);

}


int main() {
    clock_t begin = clock();

    const long long N = 45;
    for(long long i = 0; i <= N; i++){
        printf("Fib(%lli) : %lli\n", i,fib(i)); 
    }

    clock_t end = clock();
    double time_spent = (double)(end - begin)  / CLOCKS_PER_SEC;
    printf("Totale tijd gespendeerd is: %f\n", time_spent);
    return 0;
}


