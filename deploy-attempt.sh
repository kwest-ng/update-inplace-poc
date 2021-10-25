#! /bin/bash -e

cross build --target x86_64-pc-windows-gnu
mkdir -p /mnt/hostshare/scratch
rm /mnt/hostshare/scratch/* || true
cp target/x86_64-pc-windows-gnu/debug/*.exe /mnt/hostshare/scratch/
cd /mnt/hostshare/scratch
cp new_version.exe replace.exe.new
cp replace.exe replace.exe.bak
