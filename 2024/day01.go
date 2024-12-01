package main

import (
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func part1(data string) {
	var left []int
	var right []int

	lines := strings.Split(strings.TrimSpace(data), "\n")
	for _, value := range lines {
		split := strings.Split(value, "   ")
		ln, _ := strconv.Atoi(split[0])
		rn, _ := strconv.Atoi(split[1])
		left = append(left, ln)
		right = append(right, rn)
	}

	sort.Ints(left)
	sort.Ints(right)

	var diff int
	for i := 0; i < len(lines); i++ {
		d := left[i] - right[i]
		if d < 0 {
			d *= -1
		}

		diff += d
	}

	fmt.Println(diff)
}

func part2(data string) {
	var left []int
	right := make(map[int]int)

	lines := strings.Split(strings.TrimSpace(data), "\n")
	for _, value := range lines {
		split := strings.Fields(value)
		ln, _ := strconv.Atoi(split[0])
		rn, _ := strconv.Atoi(split[1])
		left = append(left, ln)
		right[rn]++
	}

	var sim int
	for i := 0; i < len(left); i++ {
		sim += left[i] * right[left[i]]
	}

	fmt.Println(sim)
}

func main() {
	data, err := os.ReadFile("./input.txt")
	if err != nil {
		panic(err)
	}

	part1(string(data))
	part2(string(data))
}
