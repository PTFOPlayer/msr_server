#include <fstream>
#include <iostream>
#include <math.h>

#include "data_getters/cpuid_data.hpp"
#include "data_getters/msr_data.hpp"

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

int main(int argc, char const *argv[])
{

	int fd;
	long long result;
	double cpu_energy_units, package_before, package_after;
	double package_power;
	fd = open_msr(0);

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
			 << "vendor = \""
			 << get_cpu_vendor() << "\"\n"
			 << "power = "
			 << package_power << "\n"
			 << "voltage = "
			 << get_cpu_voltage(fd) << "\n"
			 << "usage = "
			 << get_cpu_usage() << "\n"
			 << "temperature = "
			 << get_cpu_temperature(fd) << "\n"
			 << "hyper_threading = "
			 << get_cpu_ht(fd) << "\n"
			 << "logical_cores = " 
			 << get_cpu_cores().logical << "\n"
			 << "physical_cores = "
			 << get_cpu_cores().physical << "\n";
		file.close();
	}
}
