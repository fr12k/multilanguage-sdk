package main

/*
#cgo LDFLAGS: -L./ -lrust_core
#include "./bindings.h"
#include <stdlib.h>
*/
import "C"
import "fmt"
import "unsafe"

func main() {
	p := C.gohttp()
	s := C.GoString(p)
	C.free(unsafe.Pointer(p))
	fmt.Println(s)

	res := C.add_numbers(100, 2)
	fmt.Println(res)
}
