src/lib.rs: ../vendor/fswatch/libfswatch/src/libfswatch/c/libfswatch.h
	bindgen $< -o $@ --whitelist-function "fsw_.*" --whitelist-type "FSW_.*" --whitelist-var "FSW_.*" --raw-line '#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]'
