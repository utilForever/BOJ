#include <iostream>
#include <limits>
#include <vector>

// Reference: https://cubelover.tistory.com/8
// Reference: https://blog.myungwoo.kr/117
long long min_total_length(std::vector<int> r, std::vector<int> b)
{
	std::size_t numR = r.size();
	std::size_t numB = b.size();

	std::vector<long long> prefixSumR(numR + numB + 1, 0), prefixSumB(numR + numB + 1, 0);
	std::vector<long long> combined(numR + numB + 1, 0);
	std::vector<std::size_t> cnt(numR + numB + 1, 0);

	std::size_t idxCombined = 1, idxR = 0, idxB = 0;

	cnt[0] = 100000;

	while (idxR < numR && idxB < numB)
	{
		prefixSumR[idxCombined] = prefixSumR[idxCombined - 1];
		prefixSumB[idxCombined] = prefixSumB[idxCombined - 1];
		cnt[idxCombined] = cnt[idxCombined - 1];

		if (r[idxR] < b[idxB])
		{
			prefixSumR[idxCombined] += r[idxR];
			combined[idxCombined] = r[idxR];
			++cnt[idxCombined];

			++idxCombined;
			++idxR;
		}
		else
		{
			prefixSumB[idxCombined] += b[idxB];
			combined[idxCombined] = b[idxB];
			--cnt[idxCombined];

			++idxCombined;
			++idxB;
		}
	}

	while (idxR < numR)
	{
		prefixSumR[idxCombined] = prefixSumR[idxCombined - 1] + r[idxR];
		prefixSumB[idxCombined] = prefixSumB[idxCombined - 1];
		combined[idxCombined] = r[idxR];
		cnt[idxCombined] = cnt[idxCombined - 1] + 1;

		++idxCombined;
		++idxR;
	}

	while (idxB < numB)
	{
		prefixSumR[idxCombined] = prefixSumR[idxCombined - 1];
		prefixSumB[idxCombined] = prefixSumB[idxCombined - 1] + b[idxB];
		combined[idxCombined] = b[idxB];
		cnt[idxCombined] = cnt[idxCombined - 1] - 1;

		++idxCombined;
		++idxB;
	}

	std::vector<std::size_t> last(200001, 0);
	std::vector<long long> sum(idxCombined, 0);
	idxR = 0;
	idxB = 0;

	for (std::size_t i = 1; i < idxCombined; ++i)
	{
		sum[i] = std::numeric_limits<long long>::max();

		if (cnt[i] == cnt[i - 1] + 1)
		{
			idxR += 1;
		}
		else
		{
			idxB += 1;
		}

		long long val = last[cnt[i]];

		if (val > 0)
		{
			sum[i] = sum[val] + std::abs((prefixSumR[i] - prefixSumR[val]) - (prefixSumB[i] - prefixSumB[val]));
		}
		else if (cnt[i] == 100000)
		{
			sum[i] = std::abs(prefixSumR[i] - prefixSumB[i]);
		}

		long long ret = std::numeric_limits<long long>::max();

		if (cnt[i] == cnt[i - 1] + 1)
		{
			if (idxB != 0)
			{
				ret = std::min(ret, std::abs(combined[i] - b[idxB - 1]));
			}

			if (idxB != numB)
			{
				ret = std::min(ret, std::abs(combined[i] - b[idxB]));
			}
		}
		else
		{
			if (idxR != 0)
			{
				ret = std::min(ret, std::abs(combined[i] - r[idxR - 1]));
			}

			if (idxR != numR)
			{
				ret = std::min(ret, std::abs(combined[i] - r[idxR]));
			}
		}

		sum[i] = std::min(sum[i], sum[i - 1] + ret);
		last[cnt[i]] = i;
	}

	return sum[idxCombined - 1];
}
