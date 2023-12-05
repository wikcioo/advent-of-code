#include <stdint.h>
#include <limits>
#include <sstream>

#include "input.hpp"

std::vector<std::string> input;

struct Map
{
    uint64_t DestStart;
    uint64_t SrcStart;
    uint64_t Range;
};

std::vector<Map> seedToSoilMappings{};
std::vector<Map> soilToFertilizerMappings{};
std::vector<Map> fertilizerToWater{};
std::vector<Map> waterToLight{};
std::vector<Map> lightToTemperature{};
std::vector<Map> temperatureToHumidity{};
std::vector<Map> humidityToLocation{};

void InitMappings(const std::vector<std::string>& data, std::vector<Map>& mapping)
{
    uint64_t destStart, srcStart, range;

    for (const auto& line : data)
    {
        std::istringstream iss(line);
        iss >> destStart >> srcStart >> range;
        mapping.push_back({ destStart, srcStart, range });
    }
}

std::vector<std::string> GetNextMappings()
{
    static int line = 3;
    static int numLines = input.size();

    std::vector<std::string> lines{};

    do
    {
        lines.emplace_back(input[line]);
    } while (line < numLines && !input[++line].empty());

    line += 2;
    return lines;
}

void PopulateMappings()
{
    InitMappings(GetNextMappings(), seedToSoilMappings);
    InitMappings(GetNextMappings(), soilToFertilizerMappings);
    InitMappings(GetNextMappings(), fertilizerToWater);
    InitMappings(GetNextMappings(), waterToLight);
    InitMappings(GetNextMappings(), lightToTemperature);
    InitMappings(GetNextMappings(), temperatureToHumidity);
    InitMappings(GetNextMappings(), humidityToLocation);
}

std::vector<uint64_t> GetSeedsNumbers()
{
    std::string numbers = input[0].substr(input[0].find_first_of(':') + 2);
    std::vector<uint64_t> seeds{};
    uint64_t number;
    std::istringstream iss(numbers);

    while (iss >> number)
        seeds.push_back(number);

    return seeds;
}

uint64_t GetMappingResult(uint64_t number, const std::vector<Map>& mappings)
{
    for (const auto& map : mappings)
    {
        if (number >= map.SrcStart && number < map.SrcStart + map.Range)
            return map.DestStart + (number - map.SrcStart);
    }

    return number;
}

uint64_t GetSeedLocation(uint64_t seed)
{
    uint64_t res = GetMappingResult(seed, seedToSoilMappings);
    res = GetMappingResult(res, soilToFertilizerMappings);
    res = GetMappingResult(res, fertilizerToWater);
    res = GetMappingResult(res, waterToLight);
    res = GetMappingResult(res, lightToTemperature);
    res = GetMappingResult(res, temperatureToHumidity);
    return GetMappingResult(res, humidityToLocation);
}

void part1()
{
    auto seeds = GetSeedsNumbers();

    uint64_t lowestLocation = std::numeric_limits<uint64_t>::max();
    for (uint64_t seed : seeds)
    {
        uint64_t location = GetSeedLocation(seed);
        if (location < lowestLocation)
            lowestLocation = location;
    }

    printf("Result = %llu\n", lowestLocation);
}

struct SeedRange
{
    uint64_t Start;
    uint64_t Range;
};

std::vector<SeedRange> GetSeedRanges(const std::vector<uint64_t> seedNumbers)
{
    std::vector<SeedRange> ranges{};
    for (int i = 0; i < seedNumbers.size(); i += 2)
        ranges.push_back({ seedNumbers[i], seedNumbers[i+1] });

    return ranges;
}

void part2()
{
    auto seedNumbers = GetSeedsNumbers();
    auto seedRanges = GetSeedRanges(seedNumbers);

    uint64_t lowestLocation = std::numeric_limits<uint64_t>::max();
    for (const auto& seedRange : seedRanges)
    {
        for (uint64_t seed = seedRange.Start; seed < seedRange.Start + seedRange.Range; seed++)
        {
            uint64_t location = GetSeedLocation(seed);
            if (location < lowestLocation)
                lowestLocation = location;
        }

        printf("Finished range: [%llu-%llu)\n", seedRange.Start, seedRange.Start + seedRange.Range);
    }

    printf("Result = %llu\n", lowestLocation);
}

int main(int argc, char* argv[])
{
    if (argc < 2)
    {
        printf("No input\n");
        return 1;
    }

    input = ReadFile(argv[1]);
    PopulateMappings();

    printf("===== Part 1 =====\n");
    part1();
    printf("===== Part 2 =====\n");
    part2();

    return 0;
}
