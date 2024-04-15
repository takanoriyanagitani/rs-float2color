#!/bin/sh

cargo \
	clippy \
	--all-features \
	--target wasm32-unknown-unknown
