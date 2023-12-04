package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strings"
	"time"
)

type RowParts struct {
	Parts []RowNum
	Index int
}

func (rp *RowParts) AddPart(n RowNum) {
	rp.Parts = append(rp.Parts, n)
}

var re = regexp.MustCompile("[0-9]+")
var rowNumList = []RowNum{}
var rowPartList []RowParts
var asterisksList []Asterisk

func main() {
	start := time.Now()

	file, err := os.Open("input")
	check(err)
	defer file.Close()

	scanner := bufio.NewScanner(file)
	var inputMatrix [][]string
	row := 0

	for scanner.Scan() {
		line := scanner.Text()
		rowPartList = append(rowPartList, RowParts{Index: row})
		inputMatrix = append(inputMatrix, strings.Split(line, ""))
		rowNums := re.FindAllStringSubmatchIndex(line, -1)

		for _, val := range rowNums {
			rowNum := RowNum{
				Row:   row,
				Value: line[val[0]:val[1]],
				Index: val[0],
			}

			rowNumList = append(rowNumList, rowNum)
		}

		asterisksList = append(asterisksList, getAsterisksFromRow(row, line)...)
		row++
	}

	for _, p := range rowNumList {
		if p.IsToolInMatrix(inputMatrix) {
			rowPartList[p.Row].AddPart(p)
		}
	}

	part1(rowNumList, inputMatrix)
	part2(rowPartList, asterisksList)

	end := time.Now()
	fmt.Println(end.UnixMicro() - start.UnixMicro())
}
