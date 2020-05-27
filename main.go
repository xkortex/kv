package main

import (
	"github.com/xkortex/kv/cmd"
)

var Version = "dev"


func main() {
	cmd.Version = Version
	cmd.Execute()
}
