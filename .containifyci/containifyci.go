//go:generate bash -c "if [ ! -f go.mod ]; then echo 'Initializing go.mod...'; go mod init .containifyci; else echo 'go.mod already exists. Skipping initialization.'; fi"
//go:generate go get github.com/containifyci/engine-ci/protos2
//go:generate go get github.com/containifyci/engine-ci/client
//go:generate go mod tidy

package main

import (
	"fmt"
	"os"
	"runtime"

	"github.com/containifyci/engine-ci/client/pkg/build"
)

func main() {
	os.Chdir("../")
	opts := build.NewGoServiceBuild("go-app")
	opts.Verbose = false
	opts.File = "main.go"
	opts.Folder = "go-app"
	opts.Image = ""
	//TODO: adjust the registry to your own container registry
	opts.Registry = "containifyci"
	opts.Properties = map[string]*build.ListValue{
		"cgo":       build.NewList("1"),
		"from":      build.NewList("debiancgo"),
		"tags":      build.NewList("gnu"),
		"platforms": build.NewList(fmt.Sprintf("linux/%s", runtime.GOARCH)),
	}
	build.Serve(opts)
}
