#!/bin/bash
if (which npm) && (which npx)
then
    echo "npm, npx: OK"
else
    
    echo "npm and/or npx not fount"
    exit 1
fi 

if (which node)
then
    echo "node: OK"
else
    
    echo "node not found"
    exit 1
fi 

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
node_server=$(find . | grep msr_server.ts)
binary=$(find . -depth -maxdepth 2 | grep /msr_gen)
lib=./target/release/libmsr_rs.so
service1=$(find . | grep msr_server.service)
service2=$(find . | grep msr_rest_server.service)
if (sudo cp $service1 /etc/systemd/system/) && (sudo cp $service2 /etc/systemd/system/) && (sudo cp $binary /usr/bin/) && (sudo cp $node_server /var/msr_server/) && (sudo cp $lib /usr/lib/) && (gcc -o msr_gen ./src/main.c -I . -l msr_rs -L /usr/lib/ -lm)
then
    if (sudo cp ./package.json /var/msr_server) && (sudo npm i --prefix /var/msr_server)
    then 
        sudo -s
        if (LD_LIBRARY_PATH=/usr/lib/libmsr_rs.so:$LD_LIBRARY_PATH && exit)
        then
            echo "succes, build ended"
        else
            echo "error building"
        fi
        echo "succes, build ended"
    else
        echo "error building"
    fi
else 
    echo "error building"
fi

#build gcc -o main ./src/main.c -I . -l msr_rs -L /usr/lib/ -lm


