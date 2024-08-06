package main

import (
	"bufio"
	"fmt"
	"math/big"
	"os"
)

func main() {
	var reader = bufio.NewReader(os.Stdin)

	var a big.Int
	var b big.Int
	fmt.Fscan(reader, &a)
	fmt.Fscan(reader, &b)

	fmt.Println(&a)
	fmt.Println(&b)

	var multiplier = big.NewInt(1)
	var sum = big.NewInt(0)

	for b.Cmp(big.NewInt(0)) > 0 {
		var num = big.NewInt(0)
		num = num.Mod(&b, big.NewInt(10))
		num = num.Mul(&a, num)
		fmt.Println(num)

		sum.Add(sum, num.Mul(num, multiplier))
		multiplier.Mul(multiplier, big.NewInt(10))
		b.Div(&b, big.NewInt(10))
	}

	fmt.Println(sum)
}
