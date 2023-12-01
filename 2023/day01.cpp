#include <stdio.h>
#include <unordered_map>

#include "input.hpp"

inline bool IsDigit(char c)
{
    return '0' <= c && c <= '9';
}

void part1(const std::vector<std::string>& input)
{
    unsigned int sum = 0;
    for (const auto& line : input)
    {
        int firstDigit = 0, lastDigit = 0;
        for (int i = 0; i < line.size(); i++)
        {
            if (IsDigit(line[i]))
            {
                firstDigit = line[i] - '0';
                break;
            }
        }

        for (int i = line.size() - 1; i >= 0; i--)
        {
            if (IsDigit(line[i]))
            {
                lastDigit = line[i] - '0';
                break;
            }
        }

        sum += firstDigit * 10 + lastDigit;
    }

    printf("Sum = %ld\n", sum);
}

const std::unordered_map<std::string, int> numberMapping = {
    { "zero",  0 },
    { "one",   1 },
    { "two",   2 },
    { "three", 3 },
    { "four",  4 },
    { "five",  5 },
    { "six",   6 },
    { "seven", 7 },
    { "eight", 8 },
    { "nine",  9 }
};

int GetNumberFromStr(const std::string& str)
{
    auto it = numberMapping.find(str);
    if (it != numberMapping.end())
        return it->second;
    return -1;
}

void part2(const std::vector<std::string>& input)
{
    unsigned int sum = 0;
    for (const auto& line : input)
    {
        int firstDigit = 0, lastDigit = 0;
        for (int i = 0; i < line.size(); i++)
        {
            if (IsDigit(line[i]))
            {
                firstDigit = line[i] - '0';
                goto foundFirstDigit;
            }

            for (int j = 3; j < 6; j++)
            {
                int n = GetNumberFromStr(line.substr(i, j));
                if (n != -1)
                {
                    firstDigit = n;
                    goto foundFirstDigit;
                }
            }
        }

        foundFirstDigit:

        for (int i = line.size() - 1; i >= 0; i--)
        {
            if (IsDigit(line[i]))
            {
                lastDigit = line[i] - '0';
                goto foundSecondDigit;
            }

            for (int j = 3; j < 6; j++)
            {
                if (i >= j - 1)
                {
                    std::string substr = line.substr(i-j+1, j);
                    int n = GetNumberFromStr(substr);
                    if (n != -1)
                    {
                        lastDigit = n;
                        goto foundSecondDigit;
                    }
                }
            }
        }

        foundSecondDigit:

        sum += firstDigit * 10 + lastDigit;
    }

    printf("Sum = %ld\n", sum);
}

int main(int argc, char* argv[])
{
    if (argc < 2)
    {
        printf("No input\n");
        return 1;
    }

    auto input = ReadFile(argv[1]);

    printf("===== Part 1 =====\n");
    part1(input);
    printf("===== Part 2 =====\n");
    part2(input);

    return 0;
}
