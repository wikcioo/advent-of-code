#include <stdio.h>
#include <cmath>
#include <string>
#include <sstream>
#include <algorithm>
#include <numeric>

#include "input.hpp"

std::vector<std::string> input;

std::vector<int> ExtractNumbersFromStr(const std::string& data)
{
    std::vector<int> numbers{};
    std::istringstream iss(data);
    int number;

    while (iss >> number)
        numbers.push_back(number);

    return numbers;
}

int GetMatchesCount(const std::string& line)
{
    size_t colonIndex = line.find_first_of(':');
    size_t barIndex = line.find_first_of('|');

    auto winningNumbers = ExtractNumbersFromStr(line.substr(colonIndex + 2, barIndex - colonIndex - 2));
    auto myNumbers = ExtractNumbersFromStr(line.substr(barIndex + 2));

    int matchCount = 0;
    for (int number : myNumbers)
    {
        if (std::find(winningNumbers.begin(), winningNumbers.end(), number) != winningNumbers.end())
            matchCount++;
    }

    return matchCount;
}

void part1()
{
    unsigned int sum = 0;

    for (const auto& line : input)
    {
        int matches = GetMatchesCount(line);
        if (matches > 0)
            sum += std::pow(2, matches - 1);
    }

    printf("Sum = %ld\n", sum);
}

void part2()
{
    std::vector<int> scratchCardCount(input.size(), 1);

    for (size_t i = 0; i < input.size(); i++)
    {
        const auto& line = input[i];

        int matches = GetMatchesCount(line);
        for (size_t j = 0; j < scratchCardCount[i]; j++)
        {
            for (size_t k = i + 1; k < i + 1 + matches; k++)
                scratchCardCount[k]++;
        }
    }

    int sum = std::accumulate(scratchCardCount.begin(), scratchCardCount.end(), 0, std::plus<int>());
    printf("Sum = %d\n", sum);
}

int main(int argc, char* argv[])
{
    if (argc < 2)
    {
        printf("No input\n");
        return 1;
    }

    input = ReadFile(argv[1]);

    printf("===== Part 1 =====\n");
    part1();
    printf("===== Part 2 =====\n");
    part2();

    return 0;
}
