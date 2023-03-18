#!/bin/bash
mkdir build 
cd build
cmake .. 
make
cd ..

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

sudo npm install ts-node -g
sudo npm install 

sudo mkdir /var/msr_server
sudo cp ./src/msr_server.ts /var/msr_server/
sudo cp ./build/msr_gen /usr/bin/
if (sudo cp ./msr_server.service /etc/systemd/system/) && (sudo cp ./msr_web_server.service /etc/systemd/system/)
then
    echo "succes, build ended"
else 
    if (sudo cp ../msr_server.service /etc/systemd/system/) && (sudo cp ../msr_web_server.service /etc/systemd/system/)
    then 
        echo "succes, build ended"
    else 
        echo "error building"
    fi
fi
