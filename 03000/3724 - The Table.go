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

	var t int
	fmt.Fscan(reader, &t)

	for i := 0; i < t; i++ {
		var m, n int
		fmt.Fscan(reader, &m, &n)

		table := make([]*big.Int, m)

		for j := 0; j < m; j++ {
			table[j] = big.NewInt(1)
		}

		for j := 0; j < n; j++ {
			for k := 0; k < m; k++ {
				var val int64
				fmt.Fscan(reader, &val)

				table[k].Mul(table[k], big.NewInt(val))
			}
		}

		ret := big.NewInt(0).Set(table[0])
		index := 1

		for j := 1; j < m; j++ {
			if table[j].Cmp(ret) >= 0 {
				ret.Set(table[j])
				index = j + 1
			}
		}

		fmt.Fprintln(writer, index)
	}
}
