#include <fstream>
#include <iostream>
#include <math.h>
#include <thread>

#include "data_getters/cpuid_data.hpp"
#include "data_getters/msr_data.hpp"
#include "data_getters/system_files_data.hpp"
#include "algorithms/msleep.hpp"

#define TIME_MUL 10

using namespace std;

void start_node() {
	cout << "server starting..." << endl;
	system("node ./src/msr_server.js &");
}

int main(int argc, char const *argv[])
{
	int fd;
	long long result;
	double cpu_energy_units, package_before, package_after;
	double package_power;

	fd = msr_data::open_msr(0);
	string filepath = "/msr_data.toml";
	ofstream file;

	if (argc >= 2) {
		if (strcmp(argv[1], "-w") == 0) {
			thread t(start_node);
			t.join();
		}
	}

	cout << "starting data parsers..." << endl;
	
	while (true)
	{
		result = msr_data::read_msr(fd, MSR_POWER_UNIT);
		cpu_energy_units = pow(0.5, (double)((result >> 8) & 0x1f));

		package_before = msr_data::get_package_power_before(fd, cpu_energy_units);
		msleep(1000 / TIME_MUL);
		package_after = msr_data::get_package_power_after(fd, cpu_energy_units);

		package_power = (package_after - package_before) * TIME_MUL;

		string vendor = cpuid_data::get_cpu_vendor();
		string name = cpuid_data::get_cpu_name();
		double voltage = msr_data::get_cpu_voltage(fd);
		double usage = get_cpu_usage();
		double temperature = get_cpu_temperature_non_msr();
		bool ht = msr_data::get_cpu_ht(fd);
		auto cores_thread = cpuid_data::get_cpu_cores();
		
		file.open(filepath);
		file << "[cpu]"
			 << "\n"
			 << "vendor = \""
			 << vendor << "\"\n"
			 << "name = \""
			 << name << "\"\n"
			 << "power = "
			 << package_power << "\n"
			 << "voltage = "
			 << voltage << "\n"
			 << "usage = "
			 << usage << "\n"
			 << "temperature = "
			 << temperature << "\n"
			 << "hyper_threading = "
			 << ht << "\n"
			 << "logical_cores = "
			 << cores_thread.logical << "\n"
			 << "physical_cores = "
			 << cores_thread.physical << "\n";

		file.close();
	}
}
