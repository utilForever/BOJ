package main

import (
	"bufio"
	"fmt"
	"math/big"
	"os"
)

func main() {
	var reader = bufio.NewReader(os.Stdin)

	var a, b big.Int
	fmt.Fscan(reader, &a)
	fmt.Fscan(reader, &b)

	div := new(big.Int)
	mod := new(big.Int)

	div = div.Div(&a, &b)
	mod = mod.Mod(&a, &b)

	fmt.Println(div)
	fmt.Println(mod)
}
