package main

import (
	"bufio"
	"fmt"
	"math/big"
	"os"
)

func main() {
	var reader = bufio.NewReader(os.Stdin)
	var writer = bufio.NewWriter(os.Stdout)
	defer writer.Flush()

	var n int
	fmt.Fscan(reader, &n)

	fact := big.NewInt(1)

	for i := 1; i <= n; i++ {
		fact.Mul(fact, big.NewInt(int64(i)))
	}

	var count int

	for fact.Cmp(big.NewInt(0)) > 0 {
		val := big.NewInt(0)
		val.Set(fact)

		if val.Mod(val, big.NewInt(10)).Cmp(big.NewInt(0)) == 0 {
			count++
		}

		fact.Div(fact, big.NewInt(10))
	}

	fmt.Fprintln(writer, count)
}
