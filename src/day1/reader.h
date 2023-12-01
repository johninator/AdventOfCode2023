#ifndef READER_H
#define READER_H

#include <string>
#include <fstream>
#include <sstream>
#include <vector>

class Reader
{
public:
    Reader(const std::string &fileName_)
        : mFileName(fileName_)
    {
        mInputStream.open(mFileName);
    }

    std::vector<std::string> readFile()
    {
        std::cout << "\n** read file **";

        std::string line;
        std::istringstream lineStream;
	std::vector<std::string> lines;

        while (std::getline(mInputStream, line))
        {
          lines.push_back(line);
	}
        return lines;
    }

private:
    std::string mFileName;
    std::ifstream mInputStream;
};

#endif // READER_H
