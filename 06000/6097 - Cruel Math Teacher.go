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

	var n, p int64
	fmt.Fscan(reader, &n)
	fmt.Fscan(reader, &p)

	n_big := big.NewInt(n)
	p_big := big.NewInt(p)
	ret := n_big.Exp(n_big, p_big, nil)
	ret_string := ret.String()
	idx := 0

	for _, v := range ret_string {
		idx += 1
		fmt.Fprintf(writer, "%c", v)

		if idx == 70 {
			idx = 0
			fmt.Fprintln(writer)
		}
	}
}
