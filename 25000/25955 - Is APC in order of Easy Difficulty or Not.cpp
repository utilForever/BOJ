#include <algorithm>
#include <iostream>
#include <string>
#include <vector>

int main()
{
	std::ios::sync_with_stdio(0);
    std::cin.tie(0);
    std::cout.tie(0);

	int n;
	std::cin >> n;

	std::vector<std::string> problems;
	problems.reserve(n);

	for (int i = 0; i < n; ++i)
	{
		std::string problem;
		std::cin >> problem;

		problems.emplace_back(problem);
	}

	std::vector<std::string> sorted_problems = problems;

	std::sort(sorted_problems.begin(), sorted_problems.end(), [](const std::string& lhs, const std::string& rhs)
	{
		const std::string order = "BSGPD";

		if (order.find(lhs[0]) == order.find(rhs[0]))
		{
			std::string lhs_number = lhs;
			std::string rhs_number = rhs;

			lhs_number.erase(lhs_number.begin());
			rhs_number.erase(rhs_number.begin());

			return std::stoi(lhs_number) > std::stoi(rhs_number);
		}
		else
		{
			return order.find(lhs[0]) < order.find(rhs[0]);
		}

		return false;
	});

	bool is_same = true;
	int idx1 = -1, idx2 = -1;

	for (int i = 0; i < n; ++i)
	{
		if (problems[i] != sorted_problems[i])
		{
			is_same = false;

			if (idx1 == -1)
			{
				idx1 = i;
			}
			else
			{
				idx2 = i;
				break;
			}
		}
	}

	if (is_same)
	{
		std::cout << "OK\n";
	}
	else
	{
		std::cout << "KO\n";
		std::cout << problems[idx2] << " " << problems[idx1] << "\n";
	}

	return 0;
}
