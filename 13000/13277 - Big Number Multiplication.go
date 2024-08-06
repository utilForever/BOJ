package main

import (
	"bufio"
	"fmt"
	"math/big"
	"os"
)

func main() {
	var reader = bufio.NewReader(os.Stdin)

	var a, b big.Int
	fmt.Fscan(reader, &a, &b)

	ret := a.Mul(&a, &b)

	fmt.Println(ret)
}
