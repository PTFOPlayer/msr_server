#!/bin/bash
if (which g++) && (which cmake) && (which make)
then
    echo "g++, cmake, make: OK"
else
    
    echo "g++, cmake or make not found"
    exit 1
fi

cargo build --release 
cdir=`pwd`

if !(ls /var | grep msr_server)
then
    sudo mkdir /var/msr_server
fi

lib=./target/release/libmsr_rs.so
service1=$(find . | grep msr_server.service)

if (sudo cp $service1 /etc/systemd/system/) && (sudo cp $lib /usr/lib/libmsr_rs.so)
then
    if (gcc -o msr_gen ./src/main.c -I . -l msr_rs -L /usr/lib/ -lm)
    then 
        binary=$(find . -depth -maxdepth 2 | grep /msr_gen)
        if (sudo LD_LIBRARY_PATH=/usr/lib/libmsr_rs.so:$LD_LIBRARY_PATH bash -c 'echo $LD_LIBRARY_PATH') && (sudo cp $binary /usr/bin/msr_gen)
        then
            echo "succes, build ended"
        else
            echo "error building"
        fi
    else
        echo "error building"
    fi
else 
    echo "error building"
fi

#build gcc -o main ./src/main.c -I . -l msr_rs -L /usr/lib/ -lm


