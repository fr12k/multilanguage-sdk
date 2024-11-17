package api

/*
#cgo CFLAGS: -I./
#cgo linux,amd64,musl LDFLAGS: -L./ -lrust_core_x86_64-unknown-linux-musl
#cgo linux,arm64,musl LDFLAGS: -L./ -lrust_core_aarch64-unknown-linux-musl
#cgo linux,amd64,gnu LDFLAGS: -L./ -lrust_core_x86_64-unknown-linux-gnu
#cgo linux,arm64,gnu LDFLAGS: -L./ -lrust_core_aarch64-unknown-linux-gnu
#include "./bindings.h"
#include <stdlib.h>
*/
import "C"
import "C"
import (
	"fmt"
	"unsafe"
)

func Http() (string, int) {
	p := C.gohttp()
	s := C.GoString(p)
	C.free(unsafe.Pointer(p))
	fmt.Println(s)

	res := C.add_numbers(100, 2)
	fmt.Println(res)
	return s, int(res)
}
