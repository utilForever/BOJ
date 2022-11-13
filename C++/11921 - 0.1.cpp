#include <cstdio>

char input[50'000'000];

int main()
{
	int n = 0;
	int i = 0;
	long long ret = 0;

	fread(input, 1, sizeof(input), stdin);

	for (i = 0; input[i] != '\n'; ++i)
	{
		n = n * 10 + input[i] - '0';
	}

	for (int j = 0; j < n; ++j)
	{
		int num = 0;

		for (++i; input[i] != '\n'; ++i)
		{
			num = num * 10 + input[i] - '0';
		}

		ret += num;
	}

	printf("%d\n%lld", n, ret);
}
