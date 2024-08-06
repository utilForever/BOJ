from sys import stdin
from decimal import Decimal, getcontext

getcontext().prec = 100


def calculate(x, a, b, c, d) -> Decimal:
    return a * x**3 + b * x**2 + c * x + d


def binary_search(left, right, a, b, c, d) -> Decimal:
    while right - left > Decimal(1e-20):
        mid = (left + right) / 2

        if calculate(mid, a, b, c, d) * calculate(left, a, b, c, d) <= 0:
            right = mid
        else:
            left = mid

    return left


N = int(stdin.readline())

for i in range(N):
    A, B, C, D = map(Decimal, stdin.readline().split())
    d_of_prime = B**2 - 3 * A * C

    if d_of_prime > 0:
        x1 = (-B - d_of_prime.sqrt()) / (3 * A)
        x2 = (-B + d_of_prime.sqrt()) / (3 * A)

        if x1 > x2:
            x1, x2 = x2, x1

        val_x1 = round(calculate(x1, A, B, C, D), 20)
        val_x2 = round(calculate(x2, A, B, C, D), 20)

        if val_x1 * val_x2 > 0:
            if A > 0:
                if val_x1 > 0:
                    ret = binary_search(Decimal(-1e6), x1, A, B, C, D)
                    print(f"{ret:.20f}")
                else:
                    ret = binary_search(x2, Decimal(1e6), A, B, C, D)
                    print(f"{ret:.20f}")
            else:
                if val_x2 > 0:
                    ret = binary_search(x2, Decimal(1e6), A, B, C, D)
                    print(f"{ret:.20f}")
                else:
                    ret = binary_search(Decimal(-1e6), x1, A, B, C, D)
                    print(f"{ret:.20f}")
        elif val_x1 == 0:
            ret = binary_search(x2, Decimal(1e6), A, B, C, D)
            print(f"{x1:.20f} {ret:.20f}")
        elif val_x2 == 0:
            ret = binary_search(Decimal(-1e6), x1, A, B, C, D)
            print(f"{ret:.20f} {x2:.20f}")
        else:
            ret1 = binary_search(Decimal(-1e6), x1, A, B, C, D)
            ret2 = binary_search(x1, x2, A, B, C, D)
            ret3 = binary_search(x2, Decimal(1e6), A, B, C, D)

            print(f"{ret1:.20f} {ret2:.20f} {ret3:.20f}")
    else:
        x = binary_search(Decimal(-1e6), Decimal(1e6), A, B, C, D)
        print(f"{x:.20f}")
