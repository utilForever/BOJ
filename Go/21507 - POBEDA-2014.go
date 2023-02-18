package main

import (
	"bufio"
	"fmt"
	"math/big"
	"os"
)

func main() {
	var reader = bufio.NewReader(os.Stdin)

	var a1, a2, a3, a4 big.Int
	fmt.Fscan(reader, &a1)
	fmt.Fscan(reader, &a2)
	fmt.Fscan(reader, &a3)
	fmt.Fscan(reader, &a4)

	var ret1 big.Int
	if a1.Cmp(&a2) > 0 {
		ret1 = a2
	} else {
		ret1 = a1
	}

	var ret2 big.Int
	if a3.Cmp(&a4) > 0 {
		ret2 = a4
	} else {
		ret2 = a3
	}

	ret := big.NewInt(0)
	ret = ret.Add(&ret1, &ret2)
	ret = ret.Sqrt(ret)

	fmt.Println(ret)
}
