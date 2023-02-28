#include <fstream>
#include <fcntl.h>
#include <errno.h>
#include <unistd.h>
#include <string.h>
#include <time.h>

#define MSR_PKG_ENERGY_STATUS 0x611
#define MSR_POWER_UNIT 0x606
#define MSR_VOLTAGE 0x198
#define MSR_TEMPERATURE_STATUS 0x19c
#define MSR_TEMPERATURE_TARGET 0x1a2
#define MSR_MISC_ENABLE 0x1a0

using namespace std;

static int open_msr(int core)
{

	char msr_filename[BUFSIZ];
	int fd;

	sprintf(msr_filename, "/dev/cpu/%d/msr", core);
	fd = open(msr_filename, O_RDONLY);
	if (fd < 0)
	{
		if (errno == ENXIO)
		{
			fprintf(stderr, "rdmsr: No CPU %d\n", core);
			exit(2);
		}
		else if (errno == EIO)
		{
			fprintf(stderr, "rdmsr: CPU %d doesn't support MSRs\n",
					core);
			exit(3);
		}
		else
		{
			perror("rdmsr:open");
			fprintf(stderr, "Trying to open %s\n", msr_filename);
			exit(127);
		}
	}

	return fd;
}

static long long read_msr(int fd, int which)
{

	uint64_t data;

	if (pread(fd, &data, sizeof data, which) != sizeof data)
	{
		perror("rdmsr:pread");
		exit(127);
	}

	return (long long)data;
}

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

double get_cpu_voltage(int fd)
{
	long long result = read_msr(fd, MSR_VOLTAGE);
	double voltage = (double)(result >> 32);
	return voltage / 8192;
}

double get_package_power_before(int fd, double cpu_energy_units)
{
	long long result = read_msr(fd, MSR_PKG_ENERGY_STATUS);
	double package_before = (double)result * cpu_energy_units;
	return package_before;
}

double get_package_power_after(int fd, double cpu_energy_units)
{
	long long result = read_msr(fd, MSR_PKG_ENERGY_STATUS);
	double package_after = (double)result * cpu_energy_units;
	return package_after;
}

double get_cpu_temperature(int fd)
{
	long long result;

	result = read_msr(fd, MSR_TEMPERATURE_STATUS);
	double t1 = (double)((result >> 16) & ((1 << 6) - 1));

	result = read_msr(fd, MSR_TEMPERATURE_TARGET);
	double t2 = (double)((result >> 16) & ((1 << 7) - 1));

	return t2 - t1;
}

bool get_cpu_ht(int fd)
{
	long long result = read_msr(fd, MSR_MISC_ENABLE);
	bool ht = (bool)(result & 24);
	return ht;
}