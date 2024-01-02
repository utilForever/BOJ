package main

import (
	"bufio"
	"fmt"
	"math/big"
	"os"
)

// Reference: https://en.wikipedia.org/wiki/Powerful_number
func main() {
	var reader = bufio.NewReader(os.Stdin)

	var n big.Int
	fmt.Fscan(reader, &n)

	var x = big.NewInt(3)
	var y = big.NewInt(1)

	for {
		if new(big.Int).Exp(x, big.NewInt(2), nil).Cmp(new(big.Int).Exp(big.NewInt(100), &n, nil)) > 0 {
			fmt.Println(y, 2)
			fmt.Println(x, 1)
			break
		} else {
			var new_x = new(big.Int).Add(new(big.Int).Mul(big.NewInt(3), x), new(big.Int).Mul(big.NewInt(8), y))
			var new_y = new(big.Int).Add(new(big.Int).Mul(big.NewInt(3), y), x)

			x = new_x
			y = new_y
		}
	}
}
