#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include "data_getters/main_data/msr_data.h"
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
	int fd = open_msr(0);
	if (argc >= 2)
	{
		if (strcmp(argv[1], "-r") == 0)
		{
			double voltage = get_cpu_voltage(fd);
			double package_power = get_cpu_power(fd, TIME_MUL);

			pthread_t id_v;
			pthread_create(&id_v, NULL, update_voltage, &voltage);

			pthread_t id_p;
			pthread_create(&id_p, NULL, update_power, &package_power);
			
			server_rs(&voltage, &package_power, TIME_MUL);
		}
		else if (strcmp(argv[1], "-t") == 0)
		{
			double voltage = get_cpu_voltage(fd);
			double package_power = get_cpu_power(fd, TIME_MUL);
			print_toml_rs(&voltage, &package_power, TIME_MUL);
		}
		else if (strcmp(argv[1], "-j") == 0)
		{
			double voltage = get_cpu_voltage(fd);
			double package_power = get_cpu_power(fd, TIME_MUL);
			print_json_rs(&voltage, &package_power, TIME_MUL);
		} else {
			printf("argument not recognized");
		}
	}
	else
		printf("error: no provided arguments\n -r: access via rest api\n -t: output to terminal in toml format\n -j: output to terminal in json format");
}
