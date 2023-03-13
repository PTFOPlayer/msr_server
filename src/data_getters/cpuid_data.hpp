#include <iostream>
#include <string>
#include <algorithm>

unsigned regs[4];

typedef struct cpu_cores
{
	unsigned logical;
	unsigned physical;
} cpu_cores;

class cpuid_data
{
public:
	static std::string get_cpu_vendor()
	{

		char vendor[12];
		cpuID(0, regs);
		((unsigned *)vendor)[0] = regs[1]; // EBX
		((unsigned *)vendor)[1] = regs[3]; // EDX
		((unsigned *)vendor)[2] = regs[2]; // ECX
		return std::string(vendor, 12);
	}

	static std::string get_cpu_name()
	{
		cpuID(0x80000002, regs);
		std::string cpu_name = "";
		char temp[16];
		((unsigned *)temp)[0] = regs[0]; // EAX
		((unsigned *)temp)[1] = regs[1]; // EBX
		((unsigned *)temp)[2] = regs[2]; // ECX
		((unsigned *)temp)[3] = regs[3]; // EDX
		cpu_name += std::string(temp, 16);

		cpuID(0x80000003, regs);
		((unsigned *)temp)[0] = regs[0]; // EAX
		((unsigned *)temp)[1] = regs[1]; // EBX
		((unsigned *)temp)[2] = regs[2]; // ECX
		((unsigned *)temp)[3] = regs[3]; // EDX
		cpu_name += std::string(temp, 16);

		cpuID(0x80000004, regs);
		((unsigned *)temp)[0] = regs[0]; // EAX
		((unsigned *)temp)[1] = regs[1]; // EBX
		((unsigned *)temp)[2] = regs[2]; // ECX
		((unsigned *)temp)[3] = regs[3]; // EDX
		cpu_name += std::string(temp, 16);
		std::string cpu_name_filtrated;

		std::copy_if(std::begin(cpu_name), std::end(cpu_name), std::back_inserter(cpu_name_filtrated),
					 [](const auto c)
					 { return static_cast<unsigned char>(c) <= 0x7F && static_cast<unsigned char>(c) > 0; });

		return cpu_name_filtrated;
	}
	static cpu_cores get_cpu_cores()
	{
		std::string cpuVendor = get_cpu_vendor();

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

private:
	static void cpuID(unsigned i, unsigned regs[4])
	{
		asm volatile("cpuid"
					 : "=a"(regs[0]), "=b"(regs[1]), "=c"(regs[2]), "=d"(regs[3])
					 : "a"(i), "c"(0));
	}
};
