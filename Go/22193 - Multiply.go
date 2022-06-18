package main

import (
	"bufio"
	"fmt"
	"math/big"
	"os"
)

func main() {
	var reader = bufio.NewReader(os.Stdin)

	var n big.Int
	var m big.Int
	fmt.Fscan(reader, &n, &m)

	var a big.Int
	var b big.Int
	fmt.Fscan(reader, &a, &b)

	ret := new(big.Int)
	ret = ret.Mul(&a, &b)

	fmt.Println(ret)
}
