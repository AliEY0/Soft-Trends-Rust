#include <stdio.h>
#include <stdbool.h>
#include <time.h>



bool isPrime(int n){
    if(n < 2)
        return false;
    if(n % 2 == 0)
        return false;

    for(int i = 3; i * i <= n; i+=2){
        if(n % i == 0)
            return false;
    }
    
    return true;
}

void checkPrime(int n){
    clock_t begin = clock();
    int totaal = 1;
    printf("Aantal priemgetallen onder %d\n", n);
    for(int i = 0; i <= n; i++){
        if(isPrime(i)){
            //printf(" is een priem getal\n");
            totaal++;
        }
    }

    printf("Er zijn %d priemgetallen onder %d\n", totaal, n);
    clock_t end = clock();
    double time_spent = (double)(end - begin)  / CLOCKS_PER_SEC;
    printf("Totale tijd gespendeerd is: %f\n", time_spent);
    printf("\n");
}

int main() {
    checkPrime(10);
    checkPrime(100);
    checkPrime(10000000);
    checkPrime(20000000);

    return 0;
}


