#! /bin/bash -e

cross build --target x86_64-pc-windows-gnu
mkdir -p /mnt/hostshare/scratch
rm -vf /mnt/hostshare/scratch/*
cp -v target/x86_64-pc-windows-gnu/debug/*.exe /mnt/hostshare/scratch/
cd /mnt/hostshare/scratch
echo "Setting up minimal conditions"
mv -v replace.exe replace.exe.bak
