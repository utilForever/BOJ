#include <algorithm>
#include <iostream>
#include <vector>

const long long MOD = 998'244'353;
const long long ROOT = 3;

long long Pow(long long a, long long b)
{
    long long ret = 1;

    a %= MOD;

    while (b)
    {
        if (b & 1)
        {
            ret = ret * a % MOD;
        }

        a = a * a % MOD;
        b >>= 1;
    }

    return ret;
}

long long PowInv(long long a)
{
    return Pow(a, MOD - 2);
}

void ProcessNTT(std::vector<int> &a, bool invert)
{
    int n = a.size();

    for (int i = 1, j = 0; i < n; ++i)
    {
        int bit = n >> 1;

        for (; j & bit; bit >>= 1)
        {
            j -= bit;
        }

        j += bit;

        if (i < j)
        {
            std::swap(a[i], a[j]);
        }
    }

    for (int len = 2; len <= n; len <<= 1)
    {
        long long wLen = Pow(ROOT, (MOD - 1) / len);

        if (invert)
        {
            wLen = PowInv(wLen);
        }

        for (int i = 0; i < n; i += len)
        {
            long long w = 1;

            for (int j = 0; j < len / 2; ++j)
            {
                long long u = a[i + j];
                long long v = static_cast<long long>(a[i + j + len / 2]) * w % MOD;

                a[i + j] = (u + v) % MOD;
                a[i + j + len / 2] = (u - v + MOD) % MOD;

                w = static_cast<long long>(w) * wLen % MOD;
            }
        }
    }

    if (invert)
    {
        long long nInv = PowInv(n);

        for (int i = 0; i < n; ++i)
        {
            a[i] = static_cast<long long>(a[i]) * nInv % MOD;
        }
    }
}

std::vector<int> PolyMul(const std::vector<int> &a, const std::vector<int> &b, int maxDegree)
{
    int n = 1;

    while (n < a.size() + b.size() - 1)
    {
        n <<= 1;
    }

    std::vector<int> fa(a.begin(), a.end());
    std::vector<int> fb(b.begin(), b.end());

    fa.resize(n);
    fb.resize(n);

    ProcessNTT(fa, false);
    ProcessNTT(fb, false);

    for (int i = 0; i < n; i++)
    {
        long long val = static_cast<long long>(fa[i]) * fb[i] % MOD;
        fa[i] = val;
    }

    ProcessNTT(fa, true);

    int sizeNew = std::min(maxDegree + 1, static_cast<int>(a.size() + b.size() - 1));

    std::vector<int> ret(sizeNew);

    for (int i = 0; i < sizeNew; i++)
    {
        ret[i] = fa[i] % MOD;
    }

    return ret;
}

std::vector<int> PolyExp(std::vector<int> poly, long long exp, int maxDegree)
{
    std::vector<int> ret = {1};

    while (exp > 0)
    {
        if (exp & 1)
        {
            ret = PolyMul(ret, poly, maxDegree);

            if (ret.size() > maxDegree + 1)
            {
                ret.resize(maxDegree + 1);
            }
        }

        poly = PolyMul(poly, poly, maxDegree);

        if (poly.size() > maxDegree + 1)
        {
            poly.resize(maxDegree + 1);
        }

        exp >>= 1;
    }

    return ret;
}

int main()
{
    std::ios_base::sync_with_stdio(false);
    std::cin.tie(nullptr);

    long long n, m, x, y;
    std::cin >> n >> m >> x >> y;

    long long needHeroDead = (y + x - 1) / x;

    std::vector<int> healths(101, 0);

    for (int i = 0; i < m; ++i)
    {
        int h;
        std::cin >> h;

        long long hit = (h + x - 1) / x;
        healths[hit] += 1;
    }
    
    if (needHeroDead > n)
    {
        std::cout << 0 << '\n';
        return 0;
    }

    std::vector<int> factorial(n + 1, 0);
    std::vector<int> factorialInv(n + 1, 0);

    factorial[0] = 1;

    for (int i = 1; i <= n; ++i)
    {
        factorial[i] = static_cast<long long>(factorial[i - 1]) * i % MOD;
    }

    factorialInv[n] = PowInv(factorial[n]);

    for (int i = n - 1; i >= 0; --i)
    {
        factorialInv[i] = static_cast<long long>(factorialInv[i + 1]) * (i + 1) % MOD;
    }

    std::vector<int> polyQ(1, 1);

    for (int r = 1; r <= 100; ++r)
    {
        if (healths[r] == 0)
        {
            continue;
        }

        std::vector<int> base(r + 1, 0);

        for (int j = 0; j <= r; ++j)
        {
            base[j] = factorialInv[j];
        }

        std::vector<int> polyR = PolyExp(base, healths[r], n - needHeroDead);

        polyQ = PolyMul(polyQ, polyR, n - needHeroDead);
    }

    long long ret = 0;

    for (int i = 0; i <= n - needHeroDead; ++i)
    {
        if (i >= polyQ.size())
        {
            break;
        }

        int valN = needHeroDead + i - 1;
        int valR = i;

        long long binomial = static_cast<long long>(factorial[valN]) * factorialInv[valR] % MOD;
        binomial = (binomial * factorialInv[valN - valR]) % MOD;

        long long ways = static_cast<long long>(factorial[i]) * polyQ[i] % MOD;
        long long term = binomial * ways % MOD;

        ret = (ret + term) % MOD;
    }

    std::cout << ret % MOD << '\n';

    return 0;
}
