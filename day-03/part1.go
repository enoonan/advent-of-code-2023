package main

import (
	"fmt"
	"strconv"
)

func part1(rowNumList []RowNum, inputMatrix InputMatrix) {

	var numSum int

	for _, rowNum := range rowNumList {
		if rowNum.IsToolInMatrix(inputMatrix) {
			intVal, err := strconv.Atoi(rowNum.Value)
			check(err)
			numSum += intVal
		}
	}

	fmt.Printf("Part 1: %d\n", numSum)
}
