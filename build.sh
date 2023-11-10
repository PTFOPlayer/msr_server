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

service1=$(find . | grep msr_server.service)
modules=$(find . | grep modules.json)
binary=./target/release/msr_server
cp $binary .
mv msr_server msr_gen
if (sudo cp $service1 /etc/systemd/system/) && (sudo cp $modules /var/msr_server) && (sudo cp ./msr_gen /usr/bin/msr_gen)
then    
    echo "succes, build ended"
else
    echo "error building"
fi


