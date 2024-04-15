#!/bin/sh

cargo \
	build \
	--profile release-wasm \
	--features=wasm \
	--target wasm32-unknown-unknown
