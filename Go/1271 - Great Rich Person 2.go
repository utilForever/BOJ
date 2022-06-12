package main

import (
	"bufio"
	"fmt"
	"math/big"
	"os"
)

func main() {
	var reader = bufio.NewReader(os.Stdin)

	var n, m big.Int
	fmt.Fscan(reader, &n, &m)

	ret1 := new(big.Int)
	ret1 = ret1.Div(&n, &m)
	ret2 := new(big.Int)
	ret2 = ret2.Mod(&n, &m)

	fmt.Println(ret1)
	fmt.Println(ret2)
}
