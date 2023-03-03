mkdir build 
cd build
cmake .. 
make
cd ..

sudo cp ./build/msr_gen /usr/bin/
if (sudo cp ../msr_server.service /etc/systemd/system/) || (sudo cp ./msr_server.service /etc/systemd/system/)
then
    echo "succes, build ended"
fi
