package main

import (
	"bufio"
	"fmt"
	"math/big"
	"os"
)

type tuple [3]*big.Int

func main() {
	var reader = bufio.NewReader(os.Stdin)

	var t int
	fmt.Fscan(reader, &t)

	for i := 0; i < t; i++ {
		var a big.Int
		var b big.Int
		fmt.Fscan(reader, &a)
		fmt.Fscan(reader, &b)

		var q big.Int
		var r big.Int

		fmt.Println(q.Div(&a, &b))
		fmt.Println(r.Mod(&a, &b))

		if i != t-1 {
			fmt.Println()
		}
	}
}
