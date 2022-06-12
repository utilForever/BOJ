package main

import (
	"bufio"
	"fmt"
	"math/big"
	"os"
)

func main() {
	var reader = bufio.NewReader(os.Stdin)

	var num_apple, diff big.Int
	fmt.Fscan(reader, &num_apple, &diff)

	apple_base := new(big.Int)
	apple_base = apple_base.Div(apple_base.Sub(&num_apple, &diff), big.NewInt(2))
	apple_remain := new(big.Int)
	apple_remain = apple_remain.Add(apple_base, &diff)

	fmt.Println(apple_remain)
	fmt.Println(apple_base)
}
