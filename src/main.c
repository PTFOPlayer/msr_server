#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include "data_getters/msr_data.h"
#include "lib.c"
#include <stdint.h>
#include <pthread.h>
#define TIME_MUL 5

void *update_voltage(void *voltage)
{
	int fd = open_msr(0);
	while (true)
	{

		msleep(1000 / TIME_MUL);
		*((double *)voltage) = get_cpu_voltage(fd);
	}
}

void *update_power(void *package_power)
{
	int fd = open_msr(0);
	while (true)
	{
		*((double *)package_power) = get_cpu_power(fd, TIME_MUL);
	}
}


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

			double voltage = get_cpu_voltage(fd);
			// sleep depends on that
			double package_power = get_cpu_power(fd, TIME_MUL);

			pthread_t id_v;
			pthread_create(&id_v, NULL, update_voltage, &voltage);

			pthread_t id_p;
			pthread_create(&id_v, NULL, update_voltage, &voltage);
			
			server_rs(&voltage, &package_power, TIME_MUL);
		}
		if (strcmp(argv[1], "-o") == 0)
		{
			double voltage = get_cpu_voltage(fd);
			// sleep depends on that
			double package_power = get_cpu_power(fd, TIME_MUL);
			struct CoreStat cs = get_sys_utils_rs(TIME_MUL);
			print_toml_rs(&voltage, &package_power, TIME_MUL);
		}
		if (strcmp(argv[1], "-j") == 0)
		{
			double voltage = get_cpu_voltage(fd);
			// sleep depends on that
			double package_power = get_cpu_power(fd, TIME_MUL);
			print_json_rs(&voltage, &package_power, TIME_MUL);
		}
	}
	else
		printf("error: no provided arguments\n -f: writing to file `/msr_data.toml`\n -o: output to terminal in toml format\n -j: output to terminal in json format");
}
