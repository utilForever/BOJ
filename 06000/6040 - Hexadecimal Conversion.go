package main

import (
	"bufio"
	"fmt"
	"math/big"
	"os"
)

func main() {
	var reader = bufio.NewReader(os.Stdin)

	var n string
	fmt.Fscan(reader, &n)

	decimal := new(big.Int)
	decimal.SetString(n, 16)

	octal := decimal.Text(8)

	fmt.Println(octal)
}
