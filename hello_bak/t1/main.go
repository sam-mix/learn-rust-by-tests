package main

/*
#include <stdio.h>
int num = 42;
float f = 3.14;
char c = 'N';
void say() {
	printf("hello world\n");
}
*/
import "C"
import (
	"fmt"
	"math/rand"
	"sync"
	"time"
)

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

func testStr() {
	s1 := "jel"
	s2 := "还好我；，；，,不知道"
	fmt.Println(len(s1))
	fmt.Println(len(s2))
}

var lock sync.Mutex

var repo = make([]int, 0)
var pt int = 1
var ct int = 1
var pc int = 1
var cc int = 1

func doProducter() {
	lock.Lock()
	defer lock.Unlock()
	r := rand.Int()
	fmt.Printf("生产: 第%d次, %d\n", pt, r)
	repo = append(repo, r)
	time.Sleep(500 * time.Millisecond)
	pt++
}

func doCustomer() {
	lock.Lock()
	defer lock.Unlock()

	if len(repo) < 1 {
		return
	}
	r := repo[len(repo)-1]
	repo = repo[:len(repo)-1]
	fmt.Printf("消费: 第%d次, %d\n", ct, r)
	time.Sleep(500 * time.Millisecond)
	ct++

}

func testPC() {
	go customer()
	go producter()
	for {
	}
}

func producter() {
	for {
		doProducter()
	}
}

func customer() {
	for {
		doCustomer()
	}
}

func printer(str string) {
	lock.Lock()
	defer lock.Unlock()

	for _, v := range str {
		fmt.Printf("%c", v)
		time.Sleep(500 * time.Millisecond)
	}
}

func hello() {
	printer("hello")
}

func world() {
	printer(" world")
}

func testLock() {
	go hello()
	go world()

	for {

	}
}

func testChannel() {

	ch := make(chan int, 0)
	// go func() {
	// 	for {
	// 		println(<-ch)
	// 	}
	// }()
	ch <- 1
	ch <- 1
	ch <- 1
	close(ch)
	ch <- 1
	ch <- 1
	for {
	}
}

func main() {
	// test1()
	// testUsber()
	// testStr()
	// C.say()
	// fmt.Println(C.num)
	// fmt.Println(C.f)
	// fmt.Println(byte(C.c))
	// testLock()
	// testPC()
	testChannel()
}
