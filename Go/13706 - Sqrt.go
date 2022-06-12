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

	sqrt := new(big.Int)
	sqrt = sqrt.Sqrt(&n)

	fmt.Println(sqrt)
}
