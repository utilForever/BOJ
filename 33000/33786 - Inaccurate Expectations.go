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

	ret := big.NewInt(0)

	for i := 1; i <= n; i++ {
		prev := new(big.Int).Set(ret)

		val := big.NewInt(int64(i))
		val.Mul(val, prev)

		val.Add(val, big.NewInt(int64(i)))

		ret = val
	}

	fmt.Fprintln(writer, ret)
}
