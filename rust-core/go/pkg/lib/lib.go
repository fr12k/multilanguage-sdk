package main

import (
	"embed"
	"fmt"
	"log"
	"os"
	"runtime"
)

//go:embed librust_core.a
var rustLib embed.FS

//go:embed bindings.h
var header embed.FS

func main() {
    // Extract the embedded library
    data, err := rustLib.ReadFile("librust_core.a")
    if err != nil {
        log.Fatal(err)
    }

    // Save it to a temporary location
    tmpFile, err := os.Create("librust_core.a")
    if err != nil {
        log.Fatal(err)
    }
    defer tmpFile.Close()

    _, err = tmpFile.Write(data)
    if err != nil {
        log.Fatal(err)
    }

    fmt.Println("Embedded Rust library extracted and saved as librust_core.a")

    // Now proceed with cgo linking
    // You can run cgo to link this static library into your Go application at build time
    runtime.Goexit()
}
