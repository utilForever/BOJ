#include <iostream>

int main()
{
	int t;
	std::cin >> t;

	for (int i = 0; i < t; ++i)
	{
		int height, weight;
		std::cin >> height >> weight;

		const double bmi = weight / (height * height / 10000.0);

		if (height >= 204)
		{
			std::cout << "4\n";
		}
		else if (height >= 161)
		{
			if (bmi >= 20.0 && bmi < 25.0)
			{
				std::cout << "1\n";
			}
			else if ((bmi >= 18.5 && bmi < 20.0) || (bmi >= 25.0 && bmi < 30.0))
			{
				std::cout << "2\n";
			}
			else if ((bmi >= 16.0 && bmi < 18.5) || (bmi >= 30.0 && bmi < 35.0))
			{
				std::cout << "3\n";
			}
			else
			{
				std::cout << "4\n";
			}
		}
		else if (height >= 159)
		{
			if (bmi >= 16.0 && bmi < 35.0)
			{
				std::cout << "3\n";
			} 
			else
			{
				std::cout << "4\n";
			}
		}
		else if (height >= 146)
		{
			std::cout << "4\n";
		}
		else if (height >= 141)
		{
			std::cout << "5\n";
		}
		else
		{
			std::cout << "6\n";
		}
	}

	return 0;
}