#include <fstream>
#include <vector>
#include <algorithm>

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

size_t split(const std::string &txt, std::vector<std::string> &strs, char ch)
{
    size_t pos = txt.find( ch );
    size_t initialPos = 0;
    strs.clear();

    while( pos != std::string::npos ) {
        strs.push_back( txt.substr( initialPos, pos - initialPos ) );
        initialPos = pos + 1;

        pos = txt.find( ch, initialPos );
    }

    strs.push_back( txt.substr( initialPos, std::min( pos, txt.size() ) - initialPos + 1 ) );

    return strs.size();
}

double get_cpu_temperature_non_msr() {
	const char * cmd = "sensors | grep \"Package id 0:\"";
	char buffer[128];
    string result = "";
    FILE* pipe = popen(cmd, "r");
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
	
	vector<string> v;
    split( result, v, ' ' );

	v[4].erase(std::remove(v[4].begin(), v[4].end(), '+'), v[4].end());
	v[4].pop_back();
	v[4].pop_back();
	return stof(v[4]);
}