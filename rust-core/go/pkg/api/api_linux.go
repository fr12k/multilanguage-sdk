package api

/*
#cgo CFLAGS: -I./
#cgo linux,amd64,musl LDFLAGS: -L./ -lrust_core_linux_amd64_musl
#cgo linux,amd64,gnu LDFLAGS: -L./ -lrust_core_linux_amd64_gnu
#cgo linux,arm64,musl LDFLAGS: -L./ -lrust_core_linux_arm64_musl
#cgo linux,arm64,gnu LDFLAGS: -L./ -lrust_core_linux_arm64_gnu
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
