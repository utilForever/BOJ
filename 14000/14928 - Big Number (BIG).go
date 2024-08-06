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
	fmt.Fscan(reader, &n)

	divisor := big.NewInt(20000303)
	ret := new(big.Int)
	ret = ret.Mod(&n, divisor)

	fmt.Println(ret)
}
