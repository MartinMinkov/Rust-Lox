#!/bin/bash
FILES="./test/*.lox"
for f in $FILES; do
	echo "Running file $f"
	RUSTFLAGS=-Awarnings cargo run $f 
	echo "------------------------------"
done