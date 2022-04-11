package main

import (
	"bufio"
	"fmt"
	"math/big"
	"os"
)

// Reference: https://proofwiki.org/wiki/Lifting_The_Exponent_Lemma
func main() {
	var reader = bufio.NewReader(os.Stdin)

	var d, k big.Int
	fmt.Fscan(reader, &d, &k)

	a, b := new(big.Int), new(big.Int)
	a = a.Set(&d)
	b = b.Set(&k)

	ret := new(big.Int)

	for true {
		ret := ret.GCD(nil, nil, &d, b)
		if ret.Cmp(big.NewInt(1)) == 0 {
			break
		}

		a, b = a.Mul(a, ret), b.Div(b, ret)
	}

	ret_a, ret_b := new(big.Int), new(big.Int)
	ret_a = ret_a.Mod(&d, big.NewInt(4))
	ret_b = ret_b.Mod(&k, big.NewInt(2))

	if ret_a.Cmp(big.NewInt(2)) == 0 && ret_b.Cmp(big.NewInt(0)) == 0 && k.Cmp(big.NewInt(4)) >= 0 {
		a = a.Mul(a, big.NewInt(2))
	}

	fmt.Println(a)
}
