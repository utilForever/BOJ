package main

import (
	"bufio"
	"fmt"
	"math/big"
	"os"
)

func main() {
	var reader = bufio.NewReader(os.Stdin)

	var n int64
	fmt.Fscan(reader, &n)

	ret := new(big.Int)
	ret.MulRange(1, n)
	fmt.Println(ret)
}
