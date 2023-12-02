package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func main() {
	file, err := os.Open("../input.txt")
	check(err)
	defer file.Close()

	calibrationSum := 0
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		calibrationSum += getCalibrationValue(scanner.Text())
	}
	fmt.Println(calibrationSum)
}

func getCalibrationValue(line string) int {
	var firstDigit string
	foundFirst := false
	var lastDigit string

	for _, ch := range strings.Split(line, "") {
		if isDigit(ch) {
			if !foundFirst {
				foundFirst = true
				firstDigit = ch
			}
			lastDigit = ch
		}
	}

	strVal := firstDigit + lastDigit
	int, err := strconv.Atoi(strVal)
	check(err)
	return int
}

var digits = []string{"0", "1", "2", "3", "4", "5", "6", "7", "8", "9"}

func isDigit(str string) bool {
	for _, digit := range digits {
		if digit == str {
			return true
		}
	}
	return false
}
