#include <cmath>
#include <algorithm>
#include <unordered_map>

#include "input.hpp"

std::vector<std::string> input;

inline bool IsDigit(char c)
{
    return '0' <= c && c <= '9';
}

bool HasNeighboringSymbol(int xPos, int yPos)
{
    static int width = input[0].size() - 1;
    static int height = input.size() - 1;

    int yStart = std::max(yPos - 1, 0);
    int yEnd = std::min(yPos + 1, height);
    int xStart = std::max(xPos - 1, 0);
    int xEnd = std::min(xPos + 1, width);

    for (int y = yStart; y <= yEnd; y++)
    {
        for (int x = xStart; x <= xEnd; x++)
        {
            if (!IsDigit(input[y][x]) && input[y][x] != '.')
                return true;
        }
    }

    return false;
}

void part1()
{
    unsigned int sum = 0;
    for (int y = 0; y < input.size(); y++)
    {
        int numberStartIndex = -1;
        for (int x = 0; x < input[y].size(); x++)
        {
            char c = input[y][x];
            if (IsDigit(c) && numberStartIndex == -1)
            {
                numberStartIndex = x;
            }
            else
            {
                if ((!IsDigit(c) || x == input[y].size() - 1) && numberStartIndex != -1)
                {
                    for (int xIdx = numberStartIndex; xIdx < x; xIdx++)
                    {
                        if (HasNeighboringSymbol(xIdx, y))
                        {
                            int lastIndex = x - numberStartIndex;
                            if (x == input[y].size() - 1) lastIndex++;
                            int number = std::stoi(input[y].substr(numberStartIndex, lastIndex));
                            sum += number;
                            break;
                        }
                    }

                    numberStartIndex = -1;
                }
            }
        }
    }

    printf("Sum = %ld\n", sum);
}

struct Coords
{
    int x, y;

    bool operator==(const Coords& other) const
    {
        return x == other.x && y == other.y;
    }
};

std::string getUniqueCoordsId(Coords coords)
{
    return std::to_string(coords.x) + '-' + std::to_string(coords.y);
}

std::vector<Coords> GetNeighboringStars(int xPos, int yPos)
{
    static int width = input[0].size() - 1;
    static int height = input.size() - 1;

    int yStart = std::max(yPos - 1, 0);
    int yEnd = std::min(yPos + 1, height);
    int xStart = std::max(xPos - 1, 0);
    int xEnd = std::min(xPos + 1, width);

    std::vector<Coords> coords{};
    for (int y = yStart; y <= yEnd; y++)
    {
        for (int x = xStart; x <= xEnd; x++)
        {
            if (input[y][x] == '*')
                coords.push_back({ x, y });
        }
    }

    return coords;
}

void part2()
{
    std::unordered_map<std::string, std::vector<int>> m{};
    for (int y = 0; y < input.size(); y++)
    {
        int numberStartIndex = -1;
        for (int x = 0; x < input[y].size(); x++)
        {
            char c = input[y][x];
            if (IsDigit(c) && numberStartIndex == -1)
            {
                numberStartIndex = x;
            }
            if ((!IsDigit(c) || x == input[y].size() - 1) && numberStartIndex != -1)
            {
                std::vector<Coords> visited{};
                int xIdx = numberStartIndex;
                do
                {
                    auto coords = GetNeighboringStars(xIdx, y);
                    if (!coords.empty())
                    {
                        int lastIndex = x - numberStartIndex;
                        if (x == input[y].size() - 1) lastIndex += 1;
                        int number = std::stoi(input[y].substr(numberStartIndex, lastIndex));
                        for (const auto& coord : coords)
                        {
                            if (std::find(visited.begin(), visited.end(), coord) == visited.end())
                            {
                                m[getUniqueCoordsId(coord)].push_back(number);
                                visited.push_back(coord);
                            }
                        }
                    }

                    xIdx++;
                } while (xIdx < x);

                numberStartIndex = -1;
            }
        }
    }

    unsigned int sum = 0;
    for (const auto& i : m)
    {
        if (i.second.size() == 2)
            sum += i.second[0] * i.second[1];
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

    input = ReadFile(argv[1]);

    printf("===== Part 1 =====\n");
    part1();
    printf("===== Part 2 =====\n");
    part2();

    return 0;
}
