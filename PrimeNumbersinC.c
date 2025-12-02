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


int main() {

    clock_t begin = clock();
    int totaal = 1;

    //const int N = 10000000;
    const int N = 20000000;
    for(int i = 0; i <= N; i++){
        if(isPrime(i)){
            printf("%d is een priem getal\n", i);
            totaal++;
        }
    }

    printf("\n");
    printf("Er zijn in totaal %d priemgetallen onder %d\n", totaal, N);
    clock_t end = clock();
    double time_spent = (double)(end - begin)  / CLOCKS_PER_SEC;
    printf("Totale tijd gespendeerd is: %f\n", time_spent);
    return 0;
}


