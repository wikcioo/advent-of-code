#pragma once

#include <vector>
#include <string>
#include <fstream>

std::vector<std::string> ReadFile(const std::string& filename)
{
    std::ifstream ifs(filename);
    if (!ifs)
        return {};
    
    std::string line;
    std::vector<std::string> lines;
    while (std::getline(ifs, line))
        lines.push_back(line);

    return lines;
}
