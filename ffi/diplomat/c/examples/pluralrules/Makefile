# This file is part of ICU4X. For terms of use, please see the file
# called LICENSE at the top level of the ICU4X source tree
# (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

.DEFAULT_GOAL := test
.PHONY: build test

ALL_HEADERS := $(wildcard ../../include/*.h)
ALL_RUST := $(wildcard ../../../src/*.rs)

$(ALL_RUST):

$(ALL_HEADERS):


../../../../../target/debug/libicu_capi_staticlib_test.a: $(ALL_RUST)
	cargo build -p icu_capi_staticlib --features cpp_default
	mv ../../../../../target/debug/libicu_capi_staticlib.a ../../../../../target/debug/libicu_capi_staticlib_test.a

a.out: ../../../../../target/debug/libicu_capi_staticlib_test.a $(ALL_HEADERS) test.c
	gcc test.c ../../../../../target/debug/libicu_capi_staticlib_test.a -ldl -lpthread -lm -g

build: a.out

test: build
	./a.out