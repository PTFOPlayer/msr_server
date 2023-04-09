#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include "data_getters/msr_data.h"
#include "lib.c"

#define TIME_MUL 5

int main(int argc, char const *argv[])
{
	int fd = open_msr(0);
	char *filepath = "/msr_data.toml";
	FILE *file;
	if (argc >= 2)
	{
		if (strcmp(argv[1], "-f") == 0)
		{
			while (1)
			{
				char *vendor = get_cpu_vendor_rs();
				char *name = get_cpu_name_rs();
				double voltage = get_cpu_voltage(fd);
				float temperature = get_cpu_temp_rs();
				int threads = get_cpu_threads_rs();
				int cores = get_cpu_cores_rs();
				// sleep depends on that
				double package_power = get_cpu_power(fd, TIME_MUL);
				struct core_stat cs = get_cpu_utils_rs(TIME_MUL);

				long frequency = cs.freq;
				double usage = cs.util;

				unsigned long long memory_total = get_mem_total();
				unsigned long long memory_free = get_mem_free();
				unsigned long long memory_used = get_mem_used();

				file = fopen(filepath, "w");

				fprintf(file, "[cpu]\n");
				fprintf(file, "vendor = \"%s\" \n", vendor);
				fprintf(file, "name = \"%s\" \n", name);
				fprintf(file, "power = %lf\n", package_power);
				fprintf(file, "voltage = %lf\n", voltage);
				fprintf(file, "temperature = %f\n", temperature);
				fprintf(file, "frequency = %ld\n", frequency);
				fprintf(file, "usage = %lf\n", usage);
				fprintf(file, "logical_cores = %d\n", threads);
				fprintf(file, "physical_cores = %d\n", cores);
				fprintf(file, "[memory]\n");
				fprintf(file, "total = %lu\n", memory_total / 1024 / 1024);
				fprintf(file, "available = %lu\n", memory_free / 1024 / 1024);
				fprintf(file, "used = %lu\n", memory_used / 1024 / 1024);

				fclose(file);
			}
		}
		if (strcmp(argv[1], "-o") == 0)
		{
			char *vendor = get_cpu_vendor_rs();
			char *name = get_cpu_name_rs();
			double voltage = get_cpu_voltage(fd);
			float temperature = get_cpu_temp_rs();
			int threads = get_cpu_threads_rs();
			int cores = get_cpu_cores_rs();
			// sleep depends on that
			double package_power = get_cpu_power(fd, TIME_MUL);
			struct core_stat cs = get_cpu_utils_rs(TIME_MUL);

			long frequency = cs.freq;
			double usage = cs.util;

			unsigned long long memory_total = get_mem_total();
			unsigned long long memory_free = get_mem_free();
			unsigned long long memory_used = get_mem_used();

			printf("[cpu]\n");
			printf("vendor = \"%s\" \n", vendor);
			printf("name = \"%s\" \n", name);
			printf("power = %lf\n", package_power);
			printf("voltage = %lf\n", voltage);
			printf("temperature = %f\n", temperature);
			printf("frequency = %ld\n", frequency);
			printf("usage = %lf\n", usage);
			printf("logical_cores = %d\n", threads);
			printf("physical_cores = %d\n", cores);
			printf("[memory]\n");
			printf("total = %lu\n", memory_total / 1024 / 1024);
			printf("available = %lu\n", memory_free / 1024 / 1024);
			printf("used = %lu\n", memory_used / 1024 / 1024);
		}
		if (strcmp(argv[1], "-j") == 0)
		{

			char *vendor = get_cpu_vendor_rs();
			char *name = get_cpu_name_rs();
			double voltage = get_cpu_voltage(fd);
			float temperature = get_cpu_temp_rs();
			int threads = get_cpu_threads_rs();
			int cores = get_cpu_cores_rs();
			// sleep depends on that
			double package_power = get_cpu_power(fd, TIME_MUL);
			struct core_stat cs = get_cpu_utils_rs(TIME_MUL);

			long frequency = cs.freq;
			double usage = cs.util;

			unsigned long long memory_total = get_mem_total();
			unsigned long long memory_free = get_mem_free();
			unsigned long long memory_used = get_mem_used();

			printf("{\n\t\"cpu\":{\n");
			printf("\t\t\"vendor\" : \"%s,\" \n", vendor);
			printf("\t\t\"name\" : \"%s,\" \n", name);
			printf("\t\t\"power\" : %lf,\n", package_power);
			printf("\t\t\"voltage\" : %lf,\n", voltage);
			printf("\t\t\"temperature\" : %f,\n", temperature);
			printf("\t\t\"frequency\" : %ld,\n", frequency);
			printf("\t\t\"usage\" : %lf,\n", usage);
			printf("\t\t\"logical_cores\" : %d,\n", threads);
			printf("\t\t\"physical_cores\" : %d\n", cores);
			printf("\t},\n");
			printf("\t\"memory\":{\n");
			printf("\t\t\"total\" : %lu,\n", memory_total / 1024 / 1024);
			printf("\t\t\"available\" : %lu,\n", memory_free / 1024 / 1024);
			printf("\t\t\"used\" : %lu\n", memory_used / 1024 / 1024);
			printf("\t}\n");
			printf("}\n");
		}
	} else {
		printf("error: no provided arguments\n -f: writing to file `/msr_data.toml`\n -o: output to terminal in toml format\n -j: output to terminal in json format");
	}
}
