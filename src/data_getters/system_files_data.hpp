#include <fstream>
#include <vector>
#include <algorithm>
#include "../algorithms/split.hpp"

static unsigned long long lastTotalUser, lastTotalUserLow, lastTotalSys, lastTotalIdle;

double get_cpu_usage()
{
	double percent;
	FILE *file;
	unsigned long long totalUser, totalUserLow, totalSys, totalIdle, total;

	file = fopen("/proc/stat", "r");
	fscanf(file, "cpu %llu %llu %llu %llu", &totalUser, &totalUserLow,
		   &totalSys, &totalIdle);
	fclose(file);

	if (totalUser < lastTotalUser || totalUserLow < lastTotalUserLow ||
		totalSys < lastTotalSys || totalIdle < lastTotalIdle)
		percent = -1.0;
	else
	{
		total = (totalUser - lastTotalUser) + (totalUserLow - lastTotalUserLow) +
				(totalSys - lastTotalSys);
		percent = total;
		total += (totalIdle - lastTotalIdle);
		percent /= total;
		percent *= 100;
	}

	lastTotalUser = totalUser;
	lastTotalUserLow = totalUserLow;
	lastTotalSys = totalSys;
	lastTotalIdle = totalIdle;

	return percent;
}

double get_cpu_temperature_non_msr() {
	const char * cmd = "sensors | grep \"Package id 0:\"";
	char buffer[128];
    std::string result = "";
    std::FILE* pipe = popen(cmd, "r");
    if (!pipe) throw std::runtime_error("popen() failed!");
    try {
        while (fgets(buffer, sizeof buffer, pipe) != NULL) {
            result += buffer;
        }
    } catch (...) {
        pclose(pipe);
        return -1;
    }
    pclose(pipe);
	
	std::vector<std::string> v;
    split( result, v, ' ' );

	v[4].erase(std::remove(v[4].begin(), v[4].end(), '+'), v[4].end());
	v[4].pop_back();
	v[4].pop_back();
	return std::stof(v[4]);
}