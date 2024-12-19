#include <bits/stdc++.h>

using namespace std;

const long double PI = acosl(-1);

static void fft_opt(vector<complex<long double>> &a, bool invert)
{
	int n = (int)a.size();
	int j = 0;

	for (int i = 1; i < n; i++)
	{
		int bit = n >> 1;

		for (; (j & bit) != 0; bit >>= 1)
		{
			j ^= bit;
		}

		j ^= bit;

		if (i < j)
		{
			swap(a[i], a[j]);
		}
	}

	for (int len = 2; len <= n; len <<= 1)
	{
		long double angle = 2 * PI / len * (invert ? -1 : 1);
		complex<long double> wlen(cos(angle), sin(angle));

		for (int i = 0; i < n; i += len)
		{
			complex<long double> w(1, 0);

			for (int j = 0; j < len / 2; j++)
			{
				complex<long double> u = a[i + j];
				complex<long double> v = a[i + j + len / 2] * w;

				a[i + j] = u + v;
				a[i + j + len / 2] = u - v;
				w = w * wlen;
			}
		}
	}

	if (invert)
	{
		for (int i = 0; i < n; i++)
		{
			a[i] /= n;
		}
	}
}

static vector<long long> polymul(const vector<long long> &A, const vector<long long> &B)
{
	int n = 1;

	while (n < (int)(A.size() + B.size()))
	{
		n <<= 1;
	}

	vector<complex<long double>> fA(n), fB(n);

	for (int i = 0; i < (int)A.size(); i++)
	{
		fA[i] = complex<long double>(A[i], 0);
	}

	for (int i = 0; i < (int)B.size(); i++)
	{
		fB[i] = complex<long double>(B[i], 0);
	}

	fft_opt(fA, false);
	fft_opt(fB, false);

	for (int i = 0; i < n; i++)
	{
		fA[i] = fA[i] * fB[i];
	}

	fft_opt(fA, true);

	vector<long long> res(n);

	for (int i = 0; i < n; i++)
	{
		res[i] = round(fA[i].real());
	}

	return res;
}

int main()
{
	ios::sync_with_stdio(false);
	cin.tie(nullptr);

	int64_t n;
	cin >> n;

	string a, b;
	cin >> a >> b;

	if (a == "!" || b == "!")
	{
		cout << "!\n";
		return 0;
	}

	bool is_minus = false;

	if (n > 0 && !a.empty() && a[0] == '~')
	{
		is_minus = !is_minus;
		a.erase(a.begin());
	}

	if (n > 0 && !b.empty() && b[0] == '~')
	{
		is_minus = !is_minus;
		b.erase(b.begin());
	}

	vector<long long> a_converted, b_converted;

	for (int i = (int)a.size() - 1; i >= 0; i--)
	{
		a_converted.push_back((unsigned char)a[i] - (unsigned char)'!');
	}

	for (int i = (int)b.size() - 1; i >= 0; i--)
	{
		b_converted.push_back((unsigned char)b[i] - (unsigned char)'!');
	}

	auto conv = polymul(a_converted, b_converted);
	conv.push_back(0);

	vector<long long int> stack;
	long long int prev = 0;

	for (int i = 0; i < (int)conv.size(); i++)
	{
		long long int curr = conv[i] + prev;
		long long int mod_val = curr % n;
		long long int div_val = curr / n;

		if (mod_val < 0)
		{
			mod_val -= n;
			div_val += 1;
		}

		stack.push_back(mod_val);
		prev = div_val;
	}

	while (stack.size() > 1 && stack.back() == 0)
	{
		stack.pop_back();
	}

	if (is_minus && stack.back() != 0)
	{
		cout << "~";
	}

	for (int i = (int)stack.size() - 1; i >= 0; i--)
	{
		char c = (char)((stack[i] + (int64_t)'!'));
		cout << c;
	}

	cout << "\n";

	return 0;
}
