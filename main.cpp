#include <fstream>
#include <stdlib.h>
#include <fcntl.h>
#include <errno.h>
#include <unistd.h>
#include <math.h>
#include <string.h>
#include <time.h>
#include <iostream>

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

    do {
        res = nanosleep(&ts, &ts);
    } while (res && errno == EINTR);

    return res;
}

static int open_msr(int core) {

	char msr_filename[BUFSIZ];
	int fd;

	sprintf(msr_filename, "/dev/cpu/%d/msr", core);
	fd = open(msr_filename, O_RDONLY);
	if ( fd < 0 ) {
		if ( errno == ENXIO ) {
			fprintf(stderr, "rdmsr: No CPU %d\n", core);
			exit(2);
		} else if ( errno == EIO ) {
			fprintf(stderr, "rdmsr: CPU %d doesn't support MSRs\n",
					core);
			exit(3);
		} else {
			perror("rdmsr:open");
			fprintf(stderr,"Trying to open %s\n",msr_filename);
			exit(127);
		}
	}

	return fd;
}

static long long read_msr(int fd, int which) {

	uint64_t data;

	if ( pread(fd, &data, sizeof data, which) != sizeof data ) {
		perror("rdmsr:pread");
		exit(127);
	}

	return (long long)data;
}

#define MSR_PKG_ENERGY_STATUS 0x611
#define MSR_RAPL_POWER_UNIT	0x606
#define MSR_VOLTAGE 0x198

int main(int argc, char const *argv[]) {
        

	
    int fd;
	long long result;
	double power_units,time_units;
	double cpu_energy_units;
    double package_before, package_after;
    double voltage;
    int time_mul = 10;

	while (true) {
    	fd=open_msr(0);

		result=read_msr(fd,MSR_RAPL_POWER_UNIT);
    	power_units=pow(0.5,(double)(result&0xf));
		cpu_energy_units=pow(0.5,(double)((result>>8)&0x1f));
		time_units=pow(0.5,(double)((result>>16)&0xf));

    	result=read_msr(fd,MSR_VOLTAGE);
    	voltage = (double)(result >> 32);

		result=read_msr(fd,MSR_PKG_ENERGY_STATUS);
		package_before=(double)result*cpu_energy_units;
    	close(fd);

    	msleep(1000/time_mul);

    	fd=open_msr(0);
    	result=read_msr(fd,MSR_PKG_ENERGY_STATUS);
		package_after =(double)result*cpu_energy_units;
    	close(fd);

		ofstream file;
		string fielpath = "/msr_data.toml";
		file.open(fielpath);
		file << "[keys]" << "\n" <<  
			"power = " << (package_after - package_before) * time_mul << "\n" 
			"voltage = " << voltage / 8192 << "\n";
		file.close();
	}
}
