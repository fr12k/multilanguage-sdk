package api

/*
#cgo CFLAGS: -I./
#cgo darwin,arm64 LDFLAGS: -L./ -lrust_core_darwin_arm64 -framework Security -framework SystemConfiguration
#cgo darwin,amd64 LDFLAGS: -L./ -lrust_core_darwin_amd64 -framework Security -framework SystemConfiguration
#include "./bindings.h"
#include <stdlib.h>
*/
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
