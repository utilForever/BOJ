package main

import (
	"bufio"
	"fmt"
	"math/big"
	"os"
)

func main() {
	var reader = bufio.NewReader(os.Stdin)

	for {
		var num big.Int
		fmt.Fscan(reader, &num)

		if num.Cmp(big.NewInt(0)) == 0 {
			break
		}

		if num.Mod(&num, big.NewInt(42)).Cmp(big.NewInt(0)) == 0 {
			fmt.Println("PREMIADO")
		} else {
			fmt.Println("TENTE NOVAMENTE")
		}
	}
}
