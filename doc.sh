#!/bin/sh

cargo \
	doc \
	--all-features \
	--target wasm32-unknown-unknown
