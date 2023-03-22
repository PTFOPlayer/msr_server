#include <fstream>
#include <iostream>
#include <math.h>

#include "data_getters/cpuid_data.hpp"
#include "data_getters/msr_data.hpp"
#include "data_getters/system_files_data.hpp"
#include "data_getters/memory_data.hpp"

#define TIME_MUL 10

using namespace std;

int main(int argc, char const *argv[])
{
	int fd = msr_data::open_msr(0);
	string filepath = "/msr_data.toml";
	ofstream file;
	//cout << get_mem_total();
	if (argc >= 2)
	{
		if (strcmp(argv[1], "-f") == 0)
		{
			while (true)
			{
				string vendor = cpuid_data::get_cpu_vendor();
				string name = cpuid_data::get_cpu_name();
				double voltage = msr_data::get_cpu_voltage(fd);
				double usage = get_cpu_usage();
				double temperature = get_cpu_temperature_non_msr();
				bool ht = msr_data::get_cpu_ht(fd);
				auto cores_thread = cpuid_data::get_cpu_cores();
				// sleep depends on that
				double package_power = msr_data::get_cpu_power(fd, TIME_MUL);

				string memory_total = get_mem_total();
				string memory_available = get_mem_available();
				long memory_used = stoi(memory_total) - stoi(memory_available); 

				file.open(filepath);
				file << "[cpu]"
					 << "\n"
					 << "vendor = \"" << vendor << "\"\n"
					 << "name = \"" << name << "\"\n"
					 << "power = " << package_power << "\n"
					 << "voltage = " << voltage << "\n"
					 << "usage = " << usage << "\n"
					 << "temperature = " << temperature << "\n"
					 << "hyper_threading = " << ht << "\n"
					 << "logical_cores = " << cores_thread.logical << "\n"
					 << "physical_cores = " << cores_thread.physical << "\n"
					 << "[memory]" 
					 << "\n"
					 << "total = " << memory_total << "\n"
					 << "available = " << memory_available << "\n"
					 << "used = " << memory_used << "\n";

				file.close();
			}
		}
		if (strcmp(argv[1], "-o") == 0)
		{
			string vendor = cpuid_data::get_cpu_vendor();
			string name = cpuid_data::get_cpu_name();
			double voltage = msr_data::get_cpu_voltage(fd);
			double usage = get_cpu_usage();
			double temperature = get_cpu_temperature_non_msr();
			bool ht = msr_data::get_cpu_ht(fd);
			auto cores_thread = cpuid_data::get_cpu_cores();
			// sleep depends on that
			auto package_power = msr_data::get_cpu_power(fd, TIME_MUL);

			string memory_total = get_mem_total();
			string memory_available = get_mem_available();
			long memory_used = stoi(memory_total) - stoi(memory_available);

			cout << "[cpu]"
				 << "\n"
				 << "vendor = \"" << vendor << "\"\n"
				 << "name = \"" << name << "\"\n"
				 << "power = " << package_power << "\n"
				 << "voltage = " << voltage << "\n"
				 << "usage = " << usage << "\n"
				 << "temperature = " << temperature << "\n"
				 << "hyper_threading = " << ht << "\n"
				 << "logical_cores = " << cores_thread.logical << "\n"
				 << "physical_cores = " << cores_thread.physical << "\n"
				 << "[memory]" 
				 << "\n"
				 << "total = " << memory_total << "\n"
				 << "available = " << memory_available << "\n"
				 << "used = " << memory_used << "\n";
		}
		if (strcmp(argv[1], "-j") == 0)
		{
			string vendor = cpuid_data::get_cpu_vendor();
			string name = cpuid_data::get_cpu_name();
			double voltage = msr_data::get_cpu_voltage(fd);
			double usage = get_cpu_usage();
			double temperature = get_cpu_temperature_non_msr();
			bool ht = msr_data::get_cpu_ht(fd);
			auto cores_thread = cpuid_data::get_cpu_cores();
			// sleep depends on that
			auto package_power = msr_data::get_cpu_power(fd, TIME_MUL);

			auto memory_total = get_mem_total();
			auto memory_available = get_mem_available();
			long memory_used = stoi(memory_total) - stoi(memory_available); 

			cout << "{\"cpu\":{"
				 << "\"vendor\":\"" << vendor << "\","
				 << "\"name\":\"" << name << "\","
				 << "\"power\":" << package_power << ","
				 << "\"voltage\":" << voltage << ","
				 << "\"usage\":" << usage << ","
				 << "\"temperature\":" << temperature << ","
				 << "\"hyper_threading\":" << ht << ","
				 << "\"logical_cores\":" << cores_thread.logical << ","
				 << "\"physical_cores\":" << cores_thread.physical << "},"
				 << "\"memory\":{"
				 << "\"total\":" << memory_total << ","
				 << "\"available\":" << memory_available << ","
				 << "\"used\":" << memory_used << "}"
				 << "}";
		}
	}
}
