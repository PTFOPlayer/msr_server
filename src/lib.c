struct core_stat {
    unsigned long long freq;
    double util;
};

char *  get_cpu_vendor_rs();
char *  get_cpu_name_rs();
int get_cpu_threads_rs();
int get_cpu_cores_rs();
struct core_stat get_cpu_utils_rs(int time_mul);
float get_cpu_temp_rs();

unsigned long long get_mem_total();
unsigned long long get_mem_free();
unsigned long long get_mem_used();