#include <fstream>
#include <vector>
#include <sstream>
#include <algorithm>

std::string get_mem_total() {
    std::ifstream file;
    std::string l, null;
    file.open("/proc/meminfo");
    file >> null >> l;
    return l;
}

std::string get_mem_available() {
    std::ifstream file;
    std::string l, null;
    file.open("/proc/meminfo");
    file >> null >> null >> null
    >> null >> null >> null
    >> null >> l >> null;
    return l;

}