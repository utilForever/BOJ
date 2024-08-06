package main

import (
	"bufio"
	"fmt"
	"math/big"
	"os"
)

func main() {
	var reader = bufio.NewReader(os.Stdin)

	var a, b, c big.Int
	fmt.Fscan(reader, &a)
	fmt.Fscan(reader, &b)
	fmt.Fscan(reader, &c)

	ret := new(big.Int)
	ret = ret.Sub(&b, &c)
	ret = ret.Div(ret, &a)

	fmt.Println(ret)
}
