#include <stdio.h>
#include <stdbool.h>
#include <time.h>
#include <stdlib.h>
#include <string.h>



void revString(char* s){
    int len =  strlen(s) ;
    char* rev = malloc(len);
    for(int i = 0; i < len; i++){
        *(rev+i) = *(s + (len - 1 -i));
    }
    rev[len+1] = '\0';
    int c = len%3;
    int ind = 0;
    for(int i = 0; i < c; i++){
        ind++;
        printf("%c",*(rev+i));
    }
    if(c != 0 && len > 3){
        printf(".");
        
    }
    for(int i = c; i < len; i++){
        if((i-c)%3==2 && i != len - 1){
            
            printf("%c.",*(rev+i));
        }else{
            printf("%c",*(rev+i));
        }

    }

    free(rev);
}



void IntToString(int n){
    
    char* ans = malloc(10 * sizeof(char));
    int i  = 0;

    while(n > 0){
        *(ans + i) = (n % 10) + '0';
        i++;
        n/=10;
    }
    ans[i] = '\0';        
    revString(ans);
    free(ans);
}



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
    const int N = 20000000;
    printf("Aantal priemgetallen onder ");
    IntToString(N);
    printf("\n");
    for(int i = 0; i <= N; i++){
        if(isPrime(i)){
            IntToString(i); 
            printf(" is een priem getal\n");
            totaal++;
        }
    }

    IntToString(totaal);
    printf("\n");
    clock_t end = clock();
    double time_spent = (double)(end - begin)  / CLOCKS_PER_SEC;
    printf("Totale tijd gespendeerd is: %f\n", time_spent);
    return 0;
}


