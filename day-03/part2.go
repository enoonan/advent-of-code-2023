package main

import (
	"fmt"
	"strconv"
)

type Asterisk struct {
	Row           int
	Col           int
	AdjacentParts []RowNum
}

func (a *Asterisk) IsGear() bool {
	return len(a.AdjacentParts) == 2
}

func (a *Asterisk) GearRatio() int {
	if !a.IsGear() {
		return 0
	}
	part1Val, err := strconv.Atoi(a.AdjacentParts[0].Value)
	check(err)
	part2Val, err := strconv.Atoi(a.AdjacentParts[1].Value)
	check(err)
	return part1Val * part2Val
}

func (a *Asterisk) isAdjacentTo(p RowNum) bool {
	return (a.Col >= (p.ColRange()[0] - 1)) && (a.Col <= (p.ColRange()[1] + 1))
}

func (a *Asterisk) AddPartLocation(p RowNum) {
	a.AdjacentParts = append(a.AdjacentParts, p)
}

func getAsterisksFromRow(rowNum int, rowStr string) []Asterisk {
	var result []Asterisk
	for col, rune := range rowStr {
		if rune == '*' {
			result = append(result, Asterisk{Row: rowNum, Col: col})
		}
	}
	return result
}

func part2(rowParts []RowParts, asterisks []Asterisk) {
	var result int
	for _, a := range asterisks {
		rowsToCheck := makeRange(max(a.Row-1, 0), min(a.Row+1, len(rowParts)))
		for _, row := range rowsToCheck {
			for _, p := range rowParts[row].Parts {
				if a.isAdjacentTo(p) {
					a.AddPartLocation(p)
				}
			}
		}

		if a.IsGear() {
			result += a.GearRatio()
		}
	}

	fmt.Printf("Part 2: %d", result)
}
