#!/bin/sh

cargo \
	check \
	--features wasm \
	--target wasm32-unknown-unknown
