package main

import (
	"fmt"

	"example.com/world/test"
	"golang.org/x/example/stringutil"
)

func main() {
	fmt.Println(stringutil.Upper("hello"))
	test.Say()
}
