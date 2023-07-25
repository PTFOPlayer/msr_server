
struct CoreStat {
    unsigned long long freq;
    double util;
    int threads;
    int cores;
    float temperature;
    unsigned long long mem_total;
    unsigned long long mem_free;
    unsigned long long mem_used;
    unsigned long long * per_core_freq;
};

char *  get_cpu_vendor_rs();
char *  get_cpu_name_rs();

void print_json_rs(double* voltage, double* package_power, int time_mul);
void print_toml_rs(double* voltage, double* package_power, int time_mul);
void toml_to_file_rs(double* voltage, double* package_power, int time);

struct CoreStat get_sys_utils_rs(int time_mul);
