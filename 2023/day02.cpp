#include <sstream>

#include "input.hpp"

struct Subset
{
    int Red;
    int Green;
    int Blue;
};

struct Game
{
    int Id;
    std::vector<Subset> Subsets;
};

std::vector<std::string> Tokenize(const std::string& data, char separator)
{
    std::istringstream iss(data);
    std::vector<std::string> tokens{};
    std::string token;

    while (std::getline(iss, token, separator))
        tokens.push_back(token);

    return tokens;
}

std::vector<Game> ParseInput(const std::vector<std::string>& input)
{
    std::vector<Game> games{};
    for (const auto& gameData : input)
    {
        Game game{};

        size_t firstSpaceIdx = gameData.find(' ');
        size_t colonIdx = gameData.find(':');
        int gameId = std::stoi(gameData.substr(firstSpaceIdx + 1, colonIdx - firstSpaceIdx));

        game.Id = gameId;

        auto subsets = Tokenize(gameData.substr(colonIdx + 2), ';');
        for (const auto& subset : subsets)
        {
            Subset ss{};
            auto colors = Tokenize(subset, ',');
            for (const auto& colorData : colors)
            {
                int count;
                std::string color;
                std::istringstream iss(colorData);
                iss >> count >> color;

                if (color == "red")
                    ss.Red = count;
                else if (color == "green")
                    ss.Green = count;
                else if (color == "blue")
                    ss.Blue = count;
                else
                    printf("Invalid color value");
            }

            game.Subsets.push_back(ss);
        }

        games.push_back(game);
    }

    return games;
}

void part1(const std::vector<std::string>& input)
{
    unsigned int sum = 0;
    for (const auto& game : ParseInput(input))
    {
        bool possible = true;
        for (const auto& subset : game.Subsets)
        {
            if (subset.Red > 12 || subset.Green > 13 || subset.Blue > 14)
            {
                possible = false;
                break;
            }
        }

        if (possible)
            sum += game.Id;
    }

    printf("Sum = %ld\n", sum);
}

void part2(const std::vector<std::string>& input)
{
    unsigned int sum = 0;
    for (const auto& game : ParseInput(input))
    {
        Subset minimum{};
        for (const auto& subset : game.Subsets)
        {
            if (subset.Red > minimum.Red)
                minimum.Red = subset.Red;
            if (subset.Green > minimum.Green)
                minimum.Green = subset.Green;
            if (subset.Blue > minimum.Blue)
                minimum.Blue = subset.Blue;
        }

        sum += minimum.Red * minimum.Green * minimum.Blue;
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