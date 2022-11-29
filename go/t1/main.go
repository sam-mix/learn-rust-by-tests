package main

import "fmt"

type usber interface {
	start()
	stop()
}

type computer struct {
}

type phone struct {
}

func (*computer) start() {
	fmt.Println("computer start")
}

func (*computer) stop() {
	fmt.Println("computer stop")
}

func (*phone) start() {
	fmt.Println("phone start")
}

func (*phone) stop() {
	fmt.Println("phone stop")
}

func compute(u usber) {
	u.start()
	u.stop()
}

func test1() {
	var arr [3]int
	fmt.Println(arr)

	var bools [3]bool
	fmt.Println(bools)

	arr1 := make([]int, 4)
	fmt.Println(arr1)
	fmt.Println(len(arr1))
	fmt.Println(cap(arr1))

	m := make(map[string]string)
	// m = map[string]string{"hello": "world"}
	m["hello"] = "world"
	mx := &m
	(*mx)["hello"] = "nihao"
	fmt.Println(*mx)
}

func testUsber() {
	c := &computer{}
	p := &phone{}
	compute(c)
	compute(p)
}

func main() {
	// test1()
	testUsber()
}
