struct core_stat {
    unsigned long long freq;
    double util;
    int threads;
    int cores;
    float temperature;
    unsigned long long mem_total;
    unsigned long long mem_free;
    unsigned long long mem_used;
};

char *  get_cpu_vendor_rs();
char *  get_cpu_name_rs();
 
struct core_stat get_sys_utils_rs(int time_mul);
