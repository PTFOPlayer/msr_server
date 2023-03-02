# msr_server

msr_server is a program that collects data from CPUID, model specyfic register and some system files, then parses that data and writes it to single toml file.

msr in name is short for "modular statistic reader"

# build

first create folder named build and go in it
```
    mkdir build && cd build
```

then build it using cmake and make

```
cmake .. && make
```