#include <iostream>

int main()
{
    std::ios::sync_with_stdio(0);
    std::cin.tie(0);
    std::cout.tie(0);

	int t;
	std::cin >> t;

	for (int i = 0; i < t; ++i)
	{
		int y, m;
		std::cin >> y >> m;

		switch (m)
		{
		case 1:
			std::cout << y - 1 << ' ' << 12 << ' ' << 31 << '\n';
			break;
		case 2:
		case 4:
		case 6:
		case 8:
		case 9:
		case 11:
			std::cout << y << ' ' << m - 1 << ' ' << 31 << '\n';
			break;
		case 5:
		case 7:
		case 10:
		case 12:
			std::cout << y << ' ' << m - 1 << ' ' << 30 << '\n';
			break;
		case 3:
			if ((y % 4 == 0 && y % 100 != 0) || y % 400 == 0)
			{
				std::cout << y << ' ' << 2 << ' ' << 29 << '\n';
			}
			else
			{
				std::cout << y << ' ' << 2 << ' ' << 28 << '\n';
			}
			break;
		}
	}

	return 0;
}