package main

import (
	"bufio"
	"fmt"
	"math/big"
	"os"
	"sort"
)

type tuple [3]*big.Int

func main() {
	var reader = bufio.NewReader(os.Stdin)

	var k int64
	var n int
	fmt.Fscan(reader, &k)
	fmt.Fscan(reader, &n)

	visited := make(map[string]bool)
	record := make(map[[3]string]bool)
	queue := make([][3]*big.Int, 0)
	cnt := 0

	visited[big.NewInt(0).String()] = true
	record[[3]string{big.NewInt(0).String(), big.NewInt(0).String(), big.NewInt(1).String()}] = true
	queue = append(queue, tuple{big.NewInt(0), big.NewInt(0), big.NewInt(1)})

	for i := 0; i < len(queue); i++ {
		a, b, c := queue[i][0], queue[i][1], queue[i][2]

		for t := 0; t < 3; t++ {
			tmp1 := new(big.Int)
			tmp1.Mul(big.NewInt(k), b)
			tmp2 := new(big.Int)
			tmp2.Mul(big.NewInt(k), c)

			tmp := new(big.Int)
			tmp.Add(tmp1, tmp2)
			tmp.Sub(tmp, a)

			if tmp.Cmp(big.NewInt(0)) == 1 {
				elem_new := tuple{tmp, b, c}
				sort.Slice(elem_new[:], func(i, j int) bool {
					return elem_new[i].Cmp(elem_new[j]) == -1
				})

				if _, exist := record[[3]string{elem_new[0].String(), elem_new[1].String(), elem_new[2].String()}]; !exist {
					record[[3]string{elem_new[0].String(), elem_new[1].String(), elem_new[2].String()}] = true
					queue = append(queue, elem_new)
				}
			}

			a, b, c = b, c, a
		}

		if !visited[a.String()] && !visited[b.String()] && !visited[c.String()] && a.Cmp(b) != 0 && b.Cmp(c) != 0 {
			visited[a.String()] = true
			visited[b.String()] = true
			visited[c.String()] = true

			fmt.Println(a, b, c)

			cnt++

			if cnt == n {
				break
			}
		}
	}
}
