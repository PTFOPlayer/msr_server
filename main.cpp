#include <fstream>
#include <stdlib.h>
#include <fcntl.h>
#include <errno.h>
#include <unistd.h>
#include <math.h>
#include <string.h>
#include <time.h>
#include <iostream>
#include <dirent.h>
#include <stdio.h>

#define MSR_PKG_ENERGY_STATUS 0x611
#define MSR_POWER_UNIT 0x606
#define MSR_VOLTAGE 0x198
#define MSR_TEMPERATURE_STATUS 0x19c
#define MSR_TEMPERATURE_TARGET 0x1a2
#define MSR_MISC_ENABLE 0x1a0

#define TIME_MUL 10

using namespace std;

int msleep(long msec)
{
	struct timespec ts;
	int res;

	if (msec < 0)
	{
		errno = EINVAL;
		return -1;
	}

	ts.tv_sec = msec / 1000;
	ts.tv_nsec = (msec % 1000) * 1000000;

	do
	{
		res = nanosleep(&ts, &ts);
	} while (res && errno == EINTR);

	return res;
}

void cpuID(unsigned i, unsigned regs[4])
{
	asm volatile("cpuid"
				 : "=a"(regs[0]), "=b"(regs[1]), "=c"(regs[2]), "=d"(regs[3])
				 : "a"(i), "c"(0));
}

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

int get_cpu_threads()
{
	DIR *d;
	struct dirent *dir;
	d = opendir("/dev/cpu/");
	unsigned int t = 0;
	if (d)
	{
		while ((dir = readdir(d)) != NULL)
		{
			if (strchr(dir->d_name, '.') == NULL)
				t += 1;
		}
		closedir(d);
	}
	return t;
}

bool get_cpu_ht(int fd)
{
	long long result = read_msr(fd, MSR_MISC_ENABLE);
	bool ht = (bool)(result & 24);
	return ht;
}

unsigned regs[4];
string get_cpu_vendor()
{
	char vendor[12];
	cpuID(0, regs);
	((unsigned *)vendor)[0] = regs[1]; // EBX
	((unsigned *)vendor)[1] = regs[3]; // EDX
	((unsigned *)vendor)[2] = regs[2]; // ECX
	return string(vendor, 12);
}

typedef struct cpu_cores
{
	unsigned logical;
	unsigned physical;
} cpu_cores;

cpu_cores get_cpu_cores()
{
	string cpuVendor = get_cpu_vendor();

	cpuID(1, regs);
	unsigned cpuFeatures = regs[3];
	cpuID(1, regs);
	unsigned logical = ((regs[1] >> 16) & 0xff) / 2;
	unsigned cores = logical;

	if (cpuVendor == "GenuineIntel")
	{
		// Get DCP cache info
		cpuID(4, regs);
		cores = (((regs[0] >> 26) & 0x3f) + 1) / 2;
	}
	else if (cpuVendor == "AuthenticAMD")
	{
		cpuID(0x80000008, regs);
		cores = (((unsigned)(regs[2] & 0xff)) + 1) / 2;
	}
	return {
		logical,
		cores};
}

int main(int argc, char const *argv[])
{

	int fd;
	long long result;
	double cpu_energy_units, package_before, package_after;
	double package_power;
	fd = open_msr(0);

	cout << get_cpu_vendor() << endl;

	while (true)
	{
		result = read_msr(fd, MSR_POWER_UNIT);
		cpu_energy_units = pow(0.5, (double)((result >> 8) & 0x1f));

		package_before = get_package_power_before(fd, cpu_energy_units);
		msleep(1000 / TIME_MUL);
		package_after = get_package_power_after(fd, cpu_energy_units);

		package_power = (package_after - package_before) * TIME_MUL;

		ofstream file;
		string fielpath = "/msr_data.toml";

		file.open(fielpath);
		file << "[cpu]"
			 << "\n"
			 << "power = "
			 << package_power << "\n"
			 << "voltage = "
			 << get_cpu_voltage(fd) << "\n"
			 << "usage = "
			 << get_cpu_usage() << "\n"
			 << "temperature = "
			 << get_cpu_temperature(fd) << "\n"
			 << "thread_count = "
			 << get_cpu_threads() << "\n"
			 << "hyper_threading = "
			 << get_cpu_ht(fd) << "\n"
			 << "logical_cores = " 
			 << get_cpu_cores().logical << "\n"
			 << "physical_cores = "
			 << get_cpu_cores().physical << "\n";
		file.close();
	}
}
