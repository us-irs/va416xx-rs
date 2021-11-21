#!/bin/sh

# Use installed tool by default
svd2rust_bin="svd2rust"
# Automates the steps specified in https://docs.rs/svd2rust/0.19.0/svd2rust/
if [ -f svd2rust ]; then
	# If the local directory contains svd2rust, use that version instead
	svd2rust_bin="./svd2rust"
elif [ -f ../svd2rust ]; then
	# Keeps the repository clean
	svd2rust_bin="../svd2rust"
fi
if [ -x "$(${svd2rust_bin} --version)" ]; then
	echo "No svd2rust found locally or installed." \
		"Install it with cargo install svd2rust"
	exit
fi
${svd2rust_bin} -i svd/va416xx-base.svd
rm -rf src
form -i lib.rs -o src/ && rm lib.rs
cargo fmt
