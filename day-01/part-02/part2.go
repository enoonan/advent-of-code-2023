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
	digitKeys := getDigitKeys()
	fmt.Println(digitKeys)

	for scanner.Scan() {
		calibrationSum += getCalibrationVal(scanner.Text(), digitKeys)
	}

	fmt.Println(calibrationSum)
}

type NumVal struct {
	CurrentKey   string
	CurrentIndex int
	DigitMap     *map[string]string
}

func NewNum() NumVal {
	return NumVal{DigitMap: &digits, CurrentIndex: -1, CurrentKey: ""}
}

func getCalibrationVal(line string, digitKeys []string) int {
	firstNum := NewNum()
	lastNum := NewNum()
	lastCharIndex := len(line) - 1

	for _, digitStr := range digitKeys {
		if firstNum.CurrentIndex == 0 && lastNum.CurrentIndex == lastCharIndex {
			// we already have both values, see
			break
		}

		if firstNum.CurrentIndex != 0 {
			firstDigitStrIndex := strings.Index(line, digitStr)

			if firstNum.CurrentIndex == -1 || (firstDigitStrIndex != -1 && firstDigitStrIndex < firstNum.CurrentIndex) {
				firstNum.CurrentIndex = firstDigitStrIndex
				firstNum.CurrentKey = digitStr
			}
		}

		if lastNum.CurrentIndex != lastCharIndex {
			lastDigitStrIndex := strings.LastIndex(line, digitStr)
			if lastDigitStrIndex > lastNum.CurrentIndex {
				lastNum.CurrentIndex = lastDigitStrIndex
				lastNum.CurrentKey = digitStr
			}
		}
	}

	calibrationValStr := digits[firstNum.CurrentKey] + digits[lastNum.CurrentKey]

	calVal, err := strconv.Atoi(calibrationValStr)
	check(err)
	return calVal
}

var digits = map[string]string{
	"1":     "1",
	"2":     "2",
	"3":     "3",
	"4":     "4",
	"5":     "5",
	"6":     "6",
	"7":     "7",
	"8":     "8",
	"9":     "9",
	"one":   "1",
	"two":   "2",
	"three": "3",
	"four":  "4",
	"five":  "5",
	"six":   "6",
	"seven": "7",
	"eight": "8",
	"nine":  "9",
}

func getDigitKeys() []string {
	var digitKeys []string

	for k := range digits {
		digitKeys = append(digitKeys, k)
	}
	return digitKeys
}
