#include <iostream>
#include <algorithm>
#include "reader.h"


std::pair<int, int> findNumbers(std::vector<int> vals, int sum);

int main()
{
    // file reading
    Reader csvReader("src/day1/input.txt");
    std::vector<std::string> lines = csvReader.readFile();

	int sum = 0;
	for(const auto& line : lines) {
			std::vector<int> digits;
			std::for_each(line.begin(), line.end(),
				[&digits](char character) {
				if(std::isdigit(character)) {
					digits.push_back(character - '0');	
				}
			});
			int add = 10*digits.front() + digits.back();
			sum += add;
	}
	std::cout << "sum: " << sum << std::endl;

    return 0;
}

