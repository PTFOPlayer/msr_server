#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <pthread.h>
#include <string.h>
#include "./lib.c"
#define TIME_MUL 5


int main(int argc, char const *argv[])
{
	if (argc >= 2)
	{
		if (strcmp(argv[1], "-r") == 0)
			server_rs();
		else if (strcmp(argv[1], "-t") == 0)
			print_toml_rs();
		else if (strcmp(argv[1], "-j") == 0)
		{
			print_json_rs();
		} else {
			printf("argument not recognized");
		}
	}
	else
		printf("error: no provided arguments\n -r: access via rest api\n -t: output to terminal in toml format\n -j: output to terminal in json format");
}
