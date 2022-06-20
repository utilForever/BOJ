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

	if n.Cmp(big.NewInt(1)) == 0 || n.Cmp(big.NewInt(2)) == 0 {
		fmt.Println(1)
		return
	}

	var a = big.NewInt(1)
	var b = big.NewInt(1)
	var c = big.NewInt(0)
	var i = big.NewInt(3)

	for {
		if i.Cmp(&n) > 0 {
			break
		}

		c.Set(big.NewInt(0))
		c.Add(a, b)

		a.Set(b)
		b.Set(c)

		i.Add(i, big.NewInt(1))
	}

	fmt.Println(c.String())
}
