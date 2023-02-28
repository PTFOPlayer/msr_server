#include <iostream>

using namespace std;

void cpuID(unsigned i, unsigned regs[4])
{
	asm volatile("cpuid"
				 : "=a"(regs[0]), "=b"(regs[1]), "=c"(regs[2]), "=d"(regs[3])
				 : "a"(i), "c"(0));
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