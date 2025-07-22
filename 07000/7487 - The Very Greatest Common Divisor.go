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

	for i := 1; i <= n; i++ {
		a := new(big.Int)
		b := new(big.Int)
		fmt.Fscan(reader, a, b)

		gcd := new(big.Int).GCD(nil, nil, a, b)
		fmt.Fprintln(writer, gcd)
	}
}
