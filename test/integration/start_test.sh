#!/bin/sh

cargo build
output="test/integration/output"
golden="test/integration/golden.output"
rm $output
touch $output
bin_path="./target/debug/rorshach"

test_dir="./test/integration/test_data"
config_file="./test/integration/.rorshach.conf"


$bin_path -f $test_dir -c $config_file &
pid="$!"
sleep 2

touch $test_dir/new
sleep 2

rm $test_dir/new
sleep 2

echo "}" >> $test_dir/test.cpp
sleep 5

rm $test_dir/test
sleep 2

mv $test_dir/test.cpp $test_dir/temp.cpp
sleep 2

sed '$d' $test_dir/temp.cpp > $test_dir/test.cpp
sleep 2

rm $test_dir/temp.cpp
sleep 4

kill -9 $pid
cmp --silent $golden $output || echo "Test Failed"
