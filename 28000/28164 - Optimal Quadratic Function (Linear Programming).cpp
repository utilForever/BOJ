#include <bits/stdc++.h>

using namespace std;

#define assert_msg(x, y) assert((x && y))
#define lp_debug(x) \
    do              \
    {               \
    } while (0)

class Rng
{
private:
    static std::mt19937 engine;

public:
    static std::mt19937 &get_engine()
    {
        return engine;
    }
    template <typename T>
    static void set_seed(T const &seed)
    {
        engine = std::mt19937(seed);
    }
    static void timebased_seed()
    {
        engine = std::mt19937(std::chrono::duration_cast<std::chrono::nanoseconds>(std::chrono::high_resolution_clock::now().time_since_epoch()).count());
    }
    template <typename T>
    static typename std::enable_if<std::is_integral<T>::value, T>::type uniform(T l, T r)
    {
        return std::uniform_int_distribution<T>(l, r)(engine);
    }
    template <typename T>
    static typename std::enable_if<std::is_floating_point<T>::value, T>::type uniform(T l, T r)
    {
        return std::uniform_real_distribution<T>(l, r)(engine);
    }
};

std::mt19937 Rng::engine(std::chrono::duration_cast<std::chrono::nanoseconds>(std::chrono::high_resolution_clock::now().time_since_epoch()).count());

/**
 *  signed Bigint in base 10^18, used for Input / Output
 *  don't use for computations, doesn't support most operations
 *
 */
class Bigint_base10
{
public:
    static constexpr int64_t BASE = 1e18;
    static constexpr int DIGITS = 18;

private:
    bool is_neg;
    vector<int64_t> data;

public:
    Bigint_base10() : is_neg(false), data(1, 0) {}
    Bigint_base10(int64_t const &val) : is_neg(val < 0)
    {
        int64_t abs_val = abs(val);
        if (abs_val < BASE)
        {
            data = {abs_val};
        }
        else
        {
            data = {abs_val % BASE, abs_val / BASE};
        }
    }

    Bigint_base10 operator+(Bigint_base10 const &o) const
    {
        assert_msg(is_neg == o.is_neg, "Addition operands need to have equal sign");
        Bigint_base10 ret;
        ret.is_neg = is_neg;
        ret.data.assign(1 + max(data.size(), o.data.size()), 0);
        copy(data.begin(), data.end(), ret.data.begin());
        int64_t carry = 0;
        for (unsigned int i = 0; i < o.data.size(); ++i)
        {
            ret.data[i] += o.data[i] + carry;
            carry = 0;
            if (ret.data[i] >= BASE)
            {
                carry = 1;
                ret.data[i] -= BASE;
            }
        }
        for (unsigned int i = o.data.size(); carry; ++i)
        {
            ret.data[i] += carry;
            carry = 0;
            if (ret.data[i] >= BASE)
            {
                carry = 1;
                ret.data[i] -= BASE;
            }
        }
        return ret.trim();
    }

    Bigint_base10 operator*(int64_t const &o) const
    {
        if (o == 0)
        {
            return Bigint_base10(0);
        }
        if (o < 0)
        {
            return operator*(-o).negate();
        }
        if (o & 1)
        {
            return operator+(operator*(o - 1));
        }
        return operator+(*this) * (o / 2);
    }
    Bigint_base10 &operator+=(Bigint_base10 const &o)
    {
        *this = operator+(o);
        return *this;
    }
    Bigint_base10 &operator*=(int64_t const &o)
    {
        *this = operator*(o);
        return *this;
    }

    Bigint_base10 &trim()
    {
        while (data.size() > 1 && data.back() == 0)
        {
            data.pop_back();
        }
        return *this;
    }

    bool is_zero() const
    {
        for (auto const &e : data)
            if (e)
                return false;
        return true;
    }

    Bigint_base10 &negate()
    {
        is_neg = !is_neg;
        if (is_zero())
            is_neg = false;
        return *this;
    }

    friend ostream &operator<<(ostream &o, Bigint_base10 const &b)
    {
        if (b.is_neg)
            o << '-';
        o << b.data.back();
        o << setfill('0');
        for (auto it = next(b.data.rbegin()); it != b.data.rend(); ++it)
        {
            o << setw(9) << *it;
        }
        o << setw(0) << setfill(' ');
        return o;
    }
    friend istream &operator>>(istream &in, Bigint_base10 &b)
    {
        static string tmp;
        in >> tmp;
        assert_msg(in, "input should be readable as a string");
        if (tmp[0] == '-')
        {
            b.is_neg = true;
            tmp = tmp.substr(1, -1);
        }
        else
        {
            b.is_neg = false;
        }
        assert_msg(all_of(tmp.begin(), tmp.end(), [](char const &c)
                          { return '0' <= c && c <= '9'; }),
                   "Input should consist of digits and possibly a '-'");
        assert_msg(!tmp.empty(), "Input should contain at least one digit");

        b.data.resize((tmp.size() + DIGITS - 1) / DIGITS);
        unsigned int i, j;
        for (i = tmp.size() - DIGITS, j = 0; i > 0; i -= DIGITS, ++j)
        {
            b.data[j] = stoll(tmp.substr(i, DIGITS));
        }
        b.data[j] = stoll(tmp.substr(0, i + DIGITS));
        return in;
    }
};

/**
 *  Biginteger with fixed precision
 *  Has 31*len bits, first bit is sign
 *
 *  Is quite fast
 */
template <size_t len>
struct Bigint_Fixedsize
{
    unsigned int data[len];
    static constexpr unsigned int bits = 31;
    Bigint_Fixedsize() { memset(data, 0, sizeof(data)); }
    Bigint_Fixedsize(long long const &_a)
    {
        memset(data, 0, sizeof(data));
        unsigned long long a = _a;
        data[0] = a & ((1u << bits) - 1);
        data[1] = a >> bits;
        data[1] &= ~(1u << bits);
        if (a > ~a)
        { // negative number, use complement
            for (size_t i = 2; i < len; ++i)
            {
                data[i] = (1u << bits) - 1;
            }
        }
    }
    //__attribute__((optimize("unroll-loops")))
    Bigint_Fixedsize &operator+=(Bigint_Fixedsize const &o)
    {
        unsigned int carry = 0;
        for (size_t i = 0; i < len; ++i)
        {
            data[i] += o.data[i] + carry;
            carry = data[i] >> bits;
            data[i] &= ~(1u << bits);
        }
        return *this;
    }
    //__attribute__((optimize("unroll-loops")))
    Bigint_Fixedsize &operator-=(Bigint_Fixedsize const &o)
    {
        unsigned int carry = 0;
        for (size_t i = 0; i < len; ++i)
        {
            data[i] -= o.data[i] + carry;
            carry = data[i] >> bits;
            data[i] &= ~(1u << bits);
        }
        return *this;
    }
    Bigint_Fixedsize operator+(Bigint_Fixedsize const &o) const
    {
        Bigint_Fixedsize ret(*this);
        ret += o;
        return ret;
    }
    Bigint_Fixedsize operator-(Bigint_Fixedsize const &o) const
    {
        Bigint_Fixedsize ret(*this);
        ret -= o;
        return ret;
    }
    //__attribute__((optimize("unroll-loops")))
    void multo(Bigint_Fixedsize const &o, Bigint_Fixedsize &ret) const
    {
        static unsigned int tmp[len + 1];
        memset(tmp, 0, sizeof(tmp));
        for (size_t i = 0; i < len; ++i)
        {
            unsigned long long val = 0;
            for (size_t j = 0; j < len - i; ++j)
            {
                val += data[i] * (unsigned long long)o.data[j] + tmp[i + j];
                tmp[i + j] = val & ((1u << bits) - 1);
                val >>= bits;
            }
        }
        memcpy(ret.data, tmp, sizeof(ret.data));
    }
    Bigint_Fixedsize &operator*=(Bigint_Fixedsize const &o)
    {
        multo(o, *this);
        return *this;
    }
    Bigint_Fixedsize operator*(Bigint_Fixedsize const &o) const
    {
        Bigint_Fixedsize ret;
        multo(o, ret);
        return ret;
    }
    Bigint_Fixedsize &negate()
    {
        unsigned int carry = 0;
        for (size_t i = 0; i < len; ++i)
        {
            data[i] = ~data[i] + !carry;
            carry = (data[i] >> bits);
            data[i] &= ~(1u << bits);
        }
        return *this;
    }

    Bigint_Fixedsize operator-() const
    {
        Bigint_Fixedsize ret(*this);
        ret.negate();
        return ret;
    }
    bool operator<(Bigint_Fixedsize const &o) const
    {
        // treat sign bit
        if (data[len - 1] >> (bits - 1) != o.data[len - 1] >> (bits - 1))
        {
            return data[len - 1] >> (bits - 1);
        }
        for (size_t i = len - 1; ~i; --i)
        {
            if (data[i] != o.data[i])
                return data[i] < o.data[i];
        }
        return false;
    }
    bool operator>(Bigint_Fixedsize const &o) const
    {
        return o < *this;
    }
    bool operator<=(Bigint_Fixedsize const &o) const
    {
        return !(operator>(o));
    }
    bool operator>=(Bigint_Fixedsize const &o) const
    {
        return !(operator<(o));
    }
    bool operator==(Bigint_Fixedsize const &o) const
    {
        for (size_t i = 0; i < len; ++i)
        {
            if (data[i] != o.data[i])
                return false;
        }
        return true;
    }
    bool operator!=(Bigint_Fixedsize const &o) const
    {
        return !operator==(o);
    }
    bool operator!() const
    {
        for (size_t i = 0; i < len; ++i)
        {
            if (data[i])
                return false;
        }
        return true;
    }
    bool is_negative() const
    {
        return data[len - 1] >> (bits - 1);
    }
    void print_binary(ostream &o, Bigint_Fixedsize const &b)
    {
        o << "[";
        for (size_t i = len; i > 0; --i)
        {
            o << bitset<bits>(b.data[i - 1]);
        }
        o << "]";
    }
    friend ostream &operator<<(ostream &o, Bigint_Fixedsize const &b)
    {
        if (b.is_negative())
        {
            return o << '-' << -b << "\n";
        }
        Bigint_base10 ret(0);
        int64_t base = 1u << bits;
        for (int i = len - 1; i >= 0; --i)
        {
            ret *= base;
            ret += Bigint_base10(b.data[i]);
        }
        o << ret;

        return o;
    }
    explicit operator long double() const
    {
        if (is_negative())
        {
            return (long double)operator-();
        }
        long double ret = 0.0;
        long double base = 1u << bits;
        for (int i = len - 1; i >= 0; --i)
        {
            ret = ret * base + data[i];
        }
        return ret;
    }
    /// TODO: implement for larger inputs
    friend istream &operator>>(istream &i, Bigint_Fixedsize &b)
    {
        int64_t tmp;
        i >> tmp;
        b = Bigint_Fixedsize(tmp);
        return i;
    }
};

/**
 *  Biginteger with fixed precision
 *  Has 32*len bits, is signed
 *
 *  Trying out more optimizations.
 */
template <size_t len>
struct Bigint_Fixedsize_Fast
{
    unsigned int data[len];
    uint16_t siz;
    bool sign;
    static constexpr unsigned int bits = 32;
    Bigint_Fixedsize_Fast()
    {
        data[0] = 0;
        siz = 1;
        sign = false;
    }
    Bigint_Fixedsize_Fast(long long a)
    {
        sign = false;
        if (a < 0)
        {
            sign = true;
            a = -a;
        }
        siz = 0;
        do
        {
            long long b = a >> bits;
            data[siz] = a - (b << bits);
            a = b;
            ++siz;
        } while (a);
    }
    void trim()
    {
        while (siz > 1 && !data[siz - 1])
            --siz;
        if (siz == 1 && data[0] == 0)
            sign = false;
    }
    int comp_unsigned(Bigint_Fixedsize_Fast const &o) const
    {
        uint16_t lim = min(siz, o.siz);
        for (unsigned int i = lim; i < siz; ++i)
        {
            if (data[i])
            {
                return 1;
            }
        }
        for (unsigned int i = lim; i < o.siz; ++i)
        {
            if (o.data[i])
            {
                return -1;
            }
        }
        for (unsigned int i = lim - 1; i + 1; --i)
        {
            if (data[i] != o.data[i])
            {
                return data[i] < o.data[i] ? -1 : 1;
            }
        }
        return 0;
    }
    int comp(Bigint_Fixedsize_Fast const &o) const
    {
        int sign_mul = 1 - 2 * sign;
        if (sign != o.sign)
        {
            return sign_mul;
        }
        return sign_mul * comp_unsigned(o);
    }
    bool operator<(Bigint_Fixedsize_Fast const &o) const
    {
        return comp(o) < 0;
    }
    bool operator>(Bigint_Fixedsize_Fast const &o) const
    {
        return comp(o) > 0;
    }
    bool operator<=(Bigint_Fixedsize_Fast const &o) const
    {
        return comp(o) <= 0;
    }
    bool operator>=(Bigint_Fixedsize_Fast const &o) const
    {
        return comp(o) >= 0;
    }
    bool operator==(Bigint_Fixedsize_Fast const &o) const
    {
        return comp(o) == 0;
    }
    bool operator!=(Bigint_Fixedsize_Fast const &o) const
    {
        return comp(o) != 0;
    }
    bool operator!() const
    {
        return operator==(ZERO);
    }
    Bigint_Fixedsize_Fast operator-() const
    {
        Bigint_Fixedsize_Fast ret(*this);
        if (!!ret)
        {
            ret.sign ^= 1;
        }
        return ret;
    }
    Bigint_Fixedsize_Fast operator*(Bigint_Fixedsize_Fast const &o) const
    {
        Bigint_Fixedsize_Fast ret;
        ret.siz = min(siz + o.siz, (int)len);
        ret.sign = (sign != o.sign);
        fill(ret.data, ret.data + ret.siz, 0);
        for (unsigned int i = 0; i < siz; ++i)
        {
            unsigned long long carry = 0, carry_2;
            for (unsigned int j = 0; j < o.siz; ++j)
            {
                carry += data[i] * (unsigned long long)o.data[j] + ret.data[i + j];
                carry_2 = carry >> bits;
                ret.data[i + j] = carry - (carry_2 << bits);
                carry = carry_2;
            }
            for (unsigned int j = i + o.siz; carry; ++j)
            {
                carry += ret.data[j];
                carry_2 = carry >> bits;
                ret.data[j] = carry - (carry_2 << bits);
                carry = carry_2;
            }
        }
        ret.trim();
        return ret;
    }
    Bigint_Fixedsize_Fast &operator*=(Bigint_Fixedsize_Fast const &o)
    {
        *this = operator*(o);
        return *this;
    }
    static void unsigned_add(Bigint_Fixedsize_Fast &ret, Bigint_Fixedsize_Fast const &A, Bigint_Fixedsize_Fast const &B)
    {
        const Bigint_Fixedsize_Fast *a = &A, *b = &B;
        if (a->siz < b->siz)
            swap(a, b);
        ret.sign = A.sign;
        unsigned long long carry = 0, carry_2;
        unsigned int j;
        for (j = 0; j < b->siz; ++j)
        {
            carry += (unsigned long long)a->data[j] + (unsigned long long)b->data[j];
            carry_2 = carry >> bits;
            ret.data[j] = carry - (carry_2 << bits);
            carry = carry_2;
        }
        for (; j < a->siz; ++j)
        {
            carry += a->data[j];
            carry_2 = carry >> bits;
            ret.data[j] = carry - (carry_2 << bits);
            carry = carry_2;
        }
        if (carry)
        {
            ret.data[j++] = carry;
        }
        ret.siz = j;
        ret.trim();
    }
    static void unsigned_subtract(Bigint_Fixedsize_Fast &ret, Bigint_Fixedsize_Fast const &A, Bigint_Fixedsize_Fast const &B)
    {
        int com = A.comp_unsigned(B);
        if (com == 0)
        {
            ret.sign = false;
            ret.siz = 1;
            ret.data[0] = 0;
            return;
        }
        ret.sign = A.sign;
        const Bigint_Fixedsize_Fast *a = &A, *b = &B;
        if (com < 0)
        {
            ret.sign ^= true;
            swap(a, b);
        }
        // deal with case then o is not trimed.
        unsigned int min_siz = min(A.siz, B.siz);
        unsigned long long carry = 0, carry_2;
        unsigned int j;
        for (j = 0; j < min_siz; ++j)
        {
            carry += (unsigned long long)a->data[j] - (unsigned long long)b->data[j];
            carry_2 = carry >> bits;
            ret.data[j] = carry - (carry_2 << bits);
            carry = -(carry_2 & 1u);
        }
        for (; carry; ++j)
        {
            assert(j < a->siz);
            carry += a->data[j];
            carry_2 = carry >> bits;
            ret.data[j] = carry - (carry_2 << bits);
            carry = -(carry_2 & 1u);
        }
        copy(a->data + j, a->data + a->siz, ret.data + j);
        ret.siz = a->siz;
        ret.trim();
    }
    static void add(Bigint_Fixedsize_Fast &ret, Bigint_Fixedsize_Fast const &A, Bigint_Fixedsize_Fast const &B)
    {
        if (A.sign == B.sign)
        {
            unsigned_add(ret, A, B);
        }
        else
        {
            unsigned_subtract(ret, A, B);
        }
    }
    static void sub(Bigint_Fixedsize_Fast &ret, Bigint_Fixedsize_Fast const &A, Bigint_Fixedsize_Fast const &B)
    {
        if (A.sign != B.sign)
        {
            unsigned_add(ret, A, B);
        }
        else
        {
            unsigned_subtract(ret, A, B);
        }
    }
    Bigint_Fixedsize_Fast operator+(Bigint_Fixedsize_Fast const &o) const
    {
        Bigint_Fixedsize_Fast ret;
        add(ret, *this, o);
        return ret;
    }
    Bigint_Fixedsize_Fast &operator+=(Bigint_Fixedsize_Fast const &o)
    {
        add(*this, *this, o);
        return *this;
    }
    Bigint_Fixedsize_Fast operator-(Bigint_Fixedsize_Fast const &o) const
    {
        Bigint_Fixedsize_Fast ret;
        sub(ret, *this, o);
        return ret;
    }
    Bigint_Fixedsize_Fast operator-=(Bigint_Fixedsize_Fast const &o)
    {
        sub(*this, *this, o);
        return *this;
    }
    /// TODO: more operators
    void print_binary(ostream &o, Bigint_Fixedsize_Fast const &b)
    {
        o << "[";
        o << sign << "/" << len << "/";
        for (size_t i = siz; i > 0; --i)
        {
            o << bitset<bits>(b.data[i - 1]);
        }
        o << "]";
    }
    friend ostream &operator<<(ostream &o, Bigint_Fixedsize_Fast const &b)
    {
        if (b.sign)
        {
            return o << '-' << -b;
        }
        Bigint_base10 ret(0);
        int64_t base = 1ll << bits;
        for (int i = b.siz - 1; i >= 0; --i)
        {
            ret *= base;
            ret += Bigint_base10(b.data[i]);
        }
        o << ret;

        return o;
    }
    explicit operator long double() const
    {
        if (sign)
        {
            return (long double)operator-();
        }
        long double ret = 0.0;
        long double base = 1ll << bits;
        for (int i = siz - 1; i >= 0; --i)
        {
            ret = ret * base + data[i];
        }
        return ret;
    }
    /// TODO: implement for larger inputs
    friend istream &operator>>(istream &i, Bigint_Fixedsize_Fast &b)
    {
        int64_t tmp;
        i >> tmp;
        b = Bigint_Fixedsize_Fast(tmp);
        return i;
    }
    static const Bigint_Fixedsize_Fast ZERO;
};
template <size_t len>
const Bigint_Fixedsize_Fast<len> Bigint_Fixedsize_Fast<len>::ZERO(0);

/**
 *  Represents an integer of the form
 *  <val> + <inf_part> * inf
 *  where inf is a symbol bigger than any integer
 */
template <typename INT>
class Barrier_Int
{
private:
    Barrier_Int(INT const &_val, INT const &_inf_part) : val(_val), inf_part(_inf_part) {}

public:
    Barrier_Int() : val(0), inf_part(0) {}
    explicit Barrier_Int(INT const &_val) : val(_val), inf_part(0) {}
    static Barrier_Int infinity()
    {
        return Barrier_Int(0, 1);
    }
    static Barrier_Int negative_infinity()
    {
        return Barrier_Int(0, -1);
    }

    Barrier_Int operator-() const
    {
        return Barrier_Int(-val, -inf_part);
    }
    Barrier_Int &operator+=(Barrier_Int const &o)
    {
        val += o.val;
        inf_part += o.inf_part;
        return *this;
    }
    Barrier_Int operator+(Barrier_Int const &o) const
    {
        return Barrier_Int(val + o.val, inf_part + o.inf_part);
    }
    Barrier_Int &operator-=(Barrier_Int const &o)
    {
        val -= o.val;
        inf_part -= o.inf_part;
        return *this;
    }
    Barrier_Int operator-(Barrier_Int const &o) const
    {
        return Barrier_Int(val - o.val, inf_part - o.inf_part);
    }
    Barrier_Int &operator*=(INT const &o)
    {
        val *= o;
        inf_part *= o;
        return *this;
    }
    Barrier_Int operator*(INT const &o) const
    {
        return Barrier_Int(val * o, inf_part * o);
    }
    bool operator<(Barrier_Int const &o) const
    {
        if (inf_part != o.inf_part)
            return inf_part < o.inf_part;
        return val < o.val;
    }
    bool operator>(Barrier_Int const &o) const
    {
        return o < *this;
    }
    bool operator>=(Barrier_Int const &o) const
    {
        return !operator<(o);
    }
    bool operator<=(Barrier_Int const &o) const
    {
        return !operator>(o);
    }
    bool operator==(Barrier_Int const &o) const
    {
        return val == o.val && inf_part == o.inf_part;
    }
    bool operator!=(Barrier_Int const &o) const
    {
        return val != o.val || inf_part != o.inf_part;
    }
    friend ostream &operator<<(ostream &o, Barrier_Int const &b)
    {
        if (!b.inf_part)
        {
            return o << b.val;
        }
        o << b.inf_part << numeric_limits<double>::infinity() << "+" << b.val;
        return o;
    }
    explicit operator long double() const
    {
        if (inf_part != INT(0))
        {
            return inf_part < INT(0) ? -numeric_limits<long double>::infinity() : numeric_limits<long double>::infinity();
        }
        return (long double)val;
    }

public:
    INT val;
    INT inf_part;
};

/**
 *  Randomized LP in expected
 *  O(d! 4^d n)
 *  Does exact calculations.
 *  Does not need a barrier bound for dealing with unbounded parts
 */
template <typename FLOAT>
class Lp_Seidel_Barierless
{
private:
    // orthogonal projection of 'vec' into 'plane'
    vector<FLOAT> proj_down(vector<FLOAT> const &vec, vector<FLOAT> const &plane, size_t j)
    {
        assert(vec.size() <= plane.size() && plane.size() <= vec.size() + 1);
        assert(j + 1 < plane.size());
        assert(!!plane[j]);
        vector<FLOAT> ret(vec.size() - 1);
        // FLOAT tmp;
        if (plane[j] < FLOAT(0))
        {
            for (size_t i = 0; i + 1 < vec.size(); ++i)
            {
                ret[i] = vec[j] * plane[i + (i >= j)] - vec[i + (i >= j)] * plane[j];
            }
        }
        else
        {
            for (size_t i = 0; i + 1 < vec.size(); ++i)
            {
                ret[i] = vec[i + (i >= j)] * plane[j] - vec[j] * plane[i + (i >= j)];
            }
        }
        return ret;
    }

    // orthogonal projection of 'vec' out of 'plane'
    vector<Barrier_Int<FLOAT>> proj_up(vector<Barrier_Int<FLOAT>> const &vec, vector<FLOAT> const &plane, size_t j)
    {
        assert(vec.size() + 1 == plane.size());
        assert(j + 1 < plane.size());
        assert(!!plane[j]);
        vector<Barrier_Int<FLOAT>> ret(vec.size() + 1);
        copy(vec.begin(), vec.begin() + j, ret.begin());
        copy(vec.begin() + j, vec.end(), ret.begin() + j + 1);
        for (size_t i = 0; i < vec.size(); ++i)
        {
            ret[j] += vec[i] * plane[i + (i >= j)];
        }
        FLOAT denom = plane[j];
        if (denom < FLOAT(0))
        {
            denom = -denom;
        }
        for (size_t i = 0; i < vec.size(); ++i)
        {
            ret[i + (i >= j)] *= denom;
        }
        if (plane[j] >= FLOAT(0))
        {
            ret[j] = -ret[j];
        }
        return ret;
    }
    Barrier_Int<FLOAT> planescal(vector<Barrier_Int<FLOAT>> const &x, vector<FLOAT> const &a)
    {
        assert(x.size() == a.size());
        Barrier_Int<FLOAT> ret(0);
        for (size_t i = 0; i < x.size(); ++i)
        {
            ret += x[i] * a[i];
        }
        return ret;
    }

    // solve lp recursively
    vector<Barrier_Int<FLOAT>> solve(vector<vector<FLOAT>> const &A, vector<FLOAT> const &c, int d)
    {
        int n = A.size();
        if (d == 1)
        { // base case: single dimension
            vector<Barrier_Int<FLOAT>> ret(2);
            if (c[0] != FLOAT(0))
            {
                ret[0] = (c[0] < FLOAT(0) ? Barrier_Int<FLOAT>::negative_infinity() : Barrier_Int<FLOAT>::infinity());
            }
            ret[1].val = FLOAT(1ull);
            for (int i = 0; i < n; ++i)
            {
                if (ret[0] * A[i][0] + ret[1] * A[i].back() > Barrier_Int<FLOAT>(0))
                {
                    if (!A[i][0])
                    {
                        lp_debug("infeasible single\n");
                        return vector<Barrier_Int<FLOAT>>();
                    }
                    ret[0] = Barrier_Int<FLOAT>(-A[i].back());
                    ret[1] = Barrier_Int<FLOAT>(A[i][0]);
                    if (ret[1] < Barrier_Int<FLOAT>(0))
                    {
                        ret[1] = -ret[1];
                        ret[0] = -ret[0];
                    }
                    lp_debug(" -> " << i << " " << ret[0] << " " << ret[1] << "\n");
                }
            }
            for (int i = 0; i < n; ++i)
            {
                if (ret[0] * A[i][0] + ret[1] * A[i].back() > Barrier_Int<FLOAT>(0))
                {
                    lp_debug("infeasible\n");
                    return vector<Barrier_Int<FLOAT>>();
                }
            }
            return ret;
        }
        // initial solution
        vector<Barrier_Int<FLOAT>> x(d + 1);
        for (int i = 0; i < d; ++i)
        {
            if (c[i] != FLOAT(0))
            {
                x[i] = (c[i] < FLOAT(0) ? Barrier_Int<FLOAT>::negative_infinity() : Barrier_Int<FLOAT>::infinity());
            }
        }
        x.back() = Barrier_Int<FLOAT>(1);
        for (size_t i = 0; i < A.size(); ++i)
        {
            if (planescal(x, A[i]) > Barrier_Int<FLOAT>(0))
            {
                int k = 0;
                while (k < d && !A[i][k])
                    ++k;
                // recurse
                if (k == d)
                {
                    lp_debug("what?\n");
                    return vector<Barrier_Int<FLOAT>>();
                } // degenerate failing plane??????
                vector<vector<FLOAT>> A2(i);
                for (size_t j = 0; j < A2.size(); ++j)
                {
                    A2[j] = proj_down(A[j], A[i], k);
                }
                shuffle(A2.begin(), A2.end(), Rng::get_engine());
                lp_debug(string(2 * d, ' ') << i << "\n");
                vector<FLOAT> c2 = proj_down(c, A[i], k);
                vector<Barrier_Int<FLOAT>> x2 = solve(A2, c2, d - 1);
                if (x2.empty())
                    return x2; // infeasible
                x = proj_up(x2, A[i], k);
                lp_debug(string(2 * d, ' ') << ":");
                lp_debug(""; for (auto const &e : x) lp_debug(" " << e));
                lp_debug("\n");
            }
        }
        return x;
    }

public:
    vector<Barrier_Int<FLOAT>> solve(vector<vector<FLOAT>> const &A, vector<FLOAT> const &c)
    {
        assert(A.empty() || A[0].size() == c.size() + 1);
        return solve(A, c, c.size());
    }
    /**
     *  Maximize c^T x
     *  subject to Ax <= b
     *
     *  Returns empty vector if infeasible
     */
    vector<Barrier_Int<FLOAT>> solve(vector<vector<FLOAT>> A, vector<FLOAT> const &b, vector<FLOAT> const &c)
    {
        assert(A.size() == b.size());
        for (unsigned int i = 0; i < A.size(); ++i)
        {
            A[i].push_back(-b[i]);
        }
        return solve(A, c);
    }
};

template <typename Big_Int, bool use_two_phase = true>
class Lp_Clarkson_Barrierless
{
public:
    using Solution_Int = Barrier_Int<Big_Int>;

private:
    /**
     *  Returns a sub-multiset of size siz uniformly at random
     *  out of the set where i is present weight[i] times.
     *
     *  Runs in O(|weight| + siz^2) expected time.
     *  Could be optimized
     */
    vector<int> sample_subset(vector<int64_t> const &weight, unsigned int siz)
    {
        int64_t total_weight = accumulate(weight.begin(), weight.end(), 0ll);
        vector<int64_t> samples;
        while (samples.size() < siz)
        {
            int64_t new_sample = Rng::uniform<int64_t>(0, total_weight - 1);
            if (find(samples.begin(), samples.end(), new_sample) == samples.end())
            {
                samples.push_back(new_sample);
            }
        }
        sort(samples.begin(), samples.end());
        vector<int> ret;
        int64_t left_weight = 0;
        for (unsigned int i = 0, j = 0; i < weight.size() && j < samples.size();)
        {
            if (samples[j] < left_weight + weight[i])
            {
                ret.push_back(i);
                ++j;
            }
            else
            {
                left_weight += weight[i];
                ++i;
            }
        }
        return ret;
    }
    /// violation check
    bool is_satisfied(vector<Solution_Int> const &x, vector<Big_Int> const &a)
    {
        assert(x.size() == a.size());
        Solution_Int ret(0);
        for (size_t i = 0; i < x.size(); ++i)
        {
            ret += x[i] * a[i];
        }
        return ret <= Solution_Int(0);
    }
    vector<Solution_Int> solve_two(vector<vector<Big_Int>> const &A, vector<Big_Int> const &c)
    {
        const unsigned int sample_size = c.size() * c.size() * 4;
        Lp_Seidel_Barierless<Big_Int> sub_lp;
        // to few constrains -> use other solver
        if (A.size() < sample_size)
        {
            return sub_lp.solve(A, c);
        }
        else
        {
            int constraints = A.size();
            int variables = c.size();
            vector<int64_t> weight(constraints, 1);
            vector<Solution_Int> x;
            vector<vector<Big_Int>> subproblem_A;
            vector<char> is_violated(constraints, 0);
            for (unsigned int iteration = 1;; ++iteration)
            {
                subproblem_A.clear();
                vector<int> subspace = sample_subset(weight, sample_size);
                for (int const &e : subspace)
                {
                    subproblem_A.push_back(A[e]);
                }

                x = sub_lp.solve(subproblem_A, c);
                // infeasible case
                if (x.empty())
                {
                    return x;
                }

                int64_t total_violated = 0;
                for (int i = 0; i < constraints; ++i)
                {
                    is_violated[i] = !is_satisfied(x, A[i]);
                    if (is_violated[i])
                    {
                        total_violated += weight[i];
                    }
                }
                if (total_violated == 0)
                {
                    // cerr << "Iterations: " << iteration;
                    // cerr << ", max weight: " << *max_element(weight.begin(), weight.end()) << "\n";
                    break;
                }
                if (total_violated * 3 * variables <= accumulate(weight.begin(), weight.end(), 0ll))
                {
                    for (int i = 0; i < constraints; ++i)
                    {
                        if (is_violated[i])
                        {
                            weight[i] *= 2;
                        }
                    }
                    assert_msg(accumulate(weight.begin(), weight.end(), 0ll) < (1ll << 62), "Weight overflow");
                }
            }
            return x;
        }
    }
    vector<Solution_Int> solve_one(vector<vector<Big_Int>> const &A, vector<Big_Int> const &c)
    {
        const unsigned int constraints = A.size(), variables = c.size();
        if (constraints <= variables * variables * 6)
        {
            return solve_two(A, c);
        }
        else
        {
            const unsigned int sqrt_constraints = sqrt(constraints);
            const unsigned int sample_size = variables * sqrt(constraints);
            vector<Solution_Int> x;
            vector<vector<Big_Int>> subproblem_A;
            vector<int> violations;
            for (unsigned int iteration = 1;; ++iteration)
            {
                // function<vector<int>()> samp = [&, this](){return sample_subset(vector<int64_t>(constraints, 1), sample_size);};
                // vector<int> subspace = Timer::execute_timed<vector<int>>(samp, "Sampling");
                vector<int> subspace = sample_subset(vector<int64_t>(constraints, 1), sample_size);
                for (int const &e : subspace)
                {
                    subproblem_A.push_back(A[e]);
                }

                x = solve_two(subproblem_A, c);
                // infeasible case
                if (x.empty())
                {
                    return x;
                }
                violations.clear();
                for (unsigned int i = 0; i < constraints; ++i)
                {
                    if (!is_satisfied(x, A[i]))
                    {
                        violations.push_back(i);
                    }
                }
                // cerr << "Violations: " << violations.size() << " / " << 2 * sqrt_constraints << "\n";
                if (violations.empty())
                {
                    // cerr << "Iterations: " << iteration;
                    // cerr << ", used constraints:" << subproblem_A.size() << "\n";
                    break;
                }
                subproblem_A.erase(subproblem_A.end() - sample_size, subproblem_A.end());
                if (violations.size() <= 2 * sqrt_constraints)
                {
                    for (int const &e : violations)
                    {
                        subproblem_A.push_back(A[e]);
                    }
                }
            }
            return x;
        }
    }

public:
    vector<Solution_Int> solve(vector<vector<Big_Int>> const &A, vector<Big_Int> const &c)
    {
        if (use_two_phase)
        {
            return solve_one(A, c);
        }
        else
        {
            return solve_two(A, c);
        }
    }

    /**
     *  Maximize c^T x
     *  Subject to Ax <= b
     *
     *  Returns empty vector if infeasible
     */
    vector<Solution_Int> solve(vector<vector<Big_Int>> A, vector<Big_Int> const &b, vector<Big_Int> const &c)
    {
        assert(A.size() == b.size());
        for (unsigned int i = 0; i < A.size(); ++i)
        {
            A[i].push_back(-b[i]);
        }
        return solve(A, c);
    }
};

using DOUBLE = Bigint_Fixedsize_Fast<12>;

// Reference: https://github.com/dacin21/dacin21_codebook
// Reference: "2022-2023 Winter Petrozavodsk Camp, Day 2: GP of ainta" Editorial
int main()
{
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);

    Rng::set_seed(45453);
    Lp_Clarkson_Barrierless<DOUBLE> solver;

    int t;
    cin >> t;

    /*
        Minimize E
        Subject to  ax^2 + bx + c - E <= y
                   -ax^2 - bx - c - E <= -y
                    E >= 0
    */

    while (t--)
    {
        int n;
        cin >> n;

        vector<vector<DOUBLE>> A;
        vector<DOUBLE> b;
        vector<DOUBLE> c = {-1, 0, 0, 0}; // [E, a, b, c]

        for (int i = 0; i < n; ++i)
        {
            long long x, y;
            cin >> x >> y;

            A.push_back({-1, x * x, x, 1});
            b.push_back(y);
            A.push_back({-1, -x * x, -x, -1});
            b.push_back(-y);
        }

        auto ret = solver.solve(A, b, c);

        if (!ret.empty())
        {
            long double val = static_cast<long double>(ret[0]) / static_cast<long double>(ret[4]);
            cout << setprecision(12) << fixed << val * val << '\n';
        }
    }

    return 0;
}
