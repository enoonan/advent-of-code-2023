package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
)

func part1(fileName string) {
	file, err := os.Open(fileName)
	check(err)
	defer file.Close()
	scanner := bufio.NewScanner(file)
	var score float64

	for scanner.Scan() {
		line := scanner.Text()
		winners, ourNums := parseGame(line)
		matches := listIntersect(winners, ourNums)
		matchNum := len(matches)
		if matchNum > 0 {
			score += math.Pow(2, float64(matchNum-1))
			// fmt.Println(line)
			// fmt.Printf("%d matches, subtracting 1. 2^%d = %f, score is now %f \n", matchNum, matchNum-1, math.Pow(2, float64(matchNum-1)), score)
		}
	}
	fmt.Printf("Part 1: %d \n", int(score))
}
