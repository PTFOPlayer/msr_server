#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include "data_getters/msr_data.h"
#include "lib.c"

#define TIME_MUL 5


int main(int argc, char const *argv[])
{
	char *vendor = get_cpu_vendor_rs();
	char *name = get_cpu_name_rs();
	int fd = open_msr(0);
	char *filepath = "/msr_data.toml";
	FILE *file;
	if (argc >= 2)
	{
		if (strcmp(argv[1], "-f") == 0)
		{
			while (1)
			{
				double voltage = get_cpu_voltage(fd);
				// sleep depends on that
				double package_power = get_cpu_power(fd, TIME_MUL);
				struct core_stat cs = get_sys_utils_rs(TIME_MUL);

				long frequency = cs.freq;
				double usage = cs.util;
				unsigned long long memory_total = cs.mem_total;
				unsigned long long memory_free = cs.mem_free;
				unsigned long long memory_used = cs.mem_used;
				float temperature = cs.temperature;
				int threads = cs.threads;
				int cores = cs.cores;

				file = fopen(filepath, "w");

				fprintf(file, "[cpu]\nvendor = \"%s\"\nname = \"%s\"\npower = %lf\nvoltage = %lf\ntemperature = %f\nfrequency = %ld\nusage = %lf\nlogical_cores = %d\nphysical_cores = %d\n[memory]\ntotal = %lu\navailable = %lu\nused = %lu\n", vendor, name, package_power, voltage, temperature, frequency, usage, threads, cores, memory_total / 1024 / 1024, memory_free / 1024 / 1024, memory_used / 1024 / 1024);
				fclose(file);
			}
		}
		if (strcmp(argv[1], "-o") == 0)
		{
			double voltage = get_cpu_voltage(fd);
			// sleep depends on that
			double package_power = get_cpu_power(fd, TIME_MUL);
			struct core_stat cs = get_sys_utils_rs(TIME_MUL);

			long frequency = cs.freq;
			double usage = cs.util;
			unsigned long long memory_total = cs.mem_total;
			unsigned long long memory_free = cs.mem_free;
			unsigned long long memory_used = cs.mem_used;
			float temperature = cs.temperature;
			int threads = cs.threads;
			int cores = cs.cores;

			printf("[cpu]\nvendor = \"%s\"\nname = \"%s\"\npower = %lf\nvoltage = %lf\ntemperature = %f\nfrequency = %ld\nusage = %lf\nlogical_cores = %d\nphysical_cores = %d\n[memory]\ntotal = %lu\navailable = %lu\nused = %lu\n", vendor, name, package_power, voltage, temperature, frequency, usage, threads, cores, memory_total / 1024 / 1024, memory_free / 1024 / 1024, memory_used / 1024 / 1024);
		}
		if (strcmp(argv[1], "-j") == 0)
		{
			double voltage = get_cpu_voltage(fd);
			// sleep depends on that
			double package_power = get_cpu_power(fd, TIME_MUL);
			struct core_stat cs = get_sys_utils_rs(TIME_MUL);

			long frequency = cs.freq;
			double usage = cs.util;
			unsigned long long memory_total = cs.mem_total;
			unsigned long long memory_free = cs.mem_free;
			unsigned long long memory_used = cs.mem_used;
			float temperature = cs.temperature;
			int threads = cs.threads;
			int cores = cs.cores;

			printf("{\n\t\"cpu\":{\n\t\t\"vendor\" : \"%s,\"\n\t\t\"name\" : \"%s,\"\n\t\t\"power\" : %lf,\n\t\t\"voltage\" : %lf,\n\t\t\"temperature\" : %f,\n\t\t\"frequency\" : %ld,\n\t\t\"usage\" : %lf,\n\t\t\"logical_cores\" : %d,\n\t\t\"physical_cores\" : %d\n\t},\n\t\"memory\":{\n\t\t\"total\" : %lu,\n\t\t\"available\" : %lu,\n\t\t\"used\" : %lu\n\t}\n}\n", vendor, name, package_power, voltage, temperature, frequency, usage, threads, cores,  memory_total / 1024 / 1024, memory_free / 1024 / 1024, memory_used / 1024 / 1024);
		}
	} else printf("error: no provided arguments\n -f: writing to file `/msr_data.toml`\n -o: output to terminal in toml format\n -j: output to terminal in json format");

}
