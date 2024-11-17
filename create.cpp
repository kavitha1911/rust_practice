#include <iostream>
using namespace std;
int sum(int a, int b);
int main()
{
    int res = sum(3, 4);
    cout << res;
}

int sum(int a, int b)
{
    return a+b;
}