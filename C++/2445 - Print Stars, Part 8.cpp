/*************************************************************************
> File Name: 2445 - Print Stars, Part 8.cpp
> Author: Chan-Ho Chris Ohk
> Created Time: 2017/01/30
> Problem Link: https://www.acmicpc.net/problem/2445
> Copyright (c) 2017, Chan-Ho Chris Ohk
*************************************************************************/
#include <iostream>

int main(int argc, char* argv[])
{
	int num = 0;
	std::cin >> num;

	for (int i = 1; i <= num; ++i)
	{
		for (int j = 1; j <= i; ++j)
		{
			std::cout << "*";
		}

		for (int j = (num - i); j >= 1; --j)
		{
			std::cout << "  ";
		}

		for (int j = 1; j <= i; ++j)
		{
			std::cout << "*";
		}

		std::cout << '\n';
	}

	for (int i = num - 1; i >= 1; --i)
	{
		for (int j = i; j >= 1; --j)
		{
			std::cout << "*";
		}

		for (int j = 1; j <= (num - i); ++j)
		{
			std::cout << "  ";
		}

		for (int j = i; j >= 1; --j)
		{
			std::cout << "*";
		}

		std::cout << '\n';
	}
}