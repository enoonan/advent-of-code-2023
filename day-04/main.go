package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

const INPUT_FILE = "input.txt"

func check(err error) {
	if err != nil {
		panic(err)
	}
}

func trimItems(items []string) []string {
	r := []string{}
	for _, s := range items {
		r = append(r, strings.ReplaceAll(s, " ", ""))
	}
	return r
}

func parseGame(line string) ([]string, []string) {
	line = strings.ReplaceAll(line, "  ", " ")
	nums := strings.Split(line, ":")
	game := strings.Split(nums[1], "|")
	winners := trimItems(strings.Split(strings.Trim(game[0], " "), " "))
	numsWeHave := trimItems(strings.Split(strings.Trim(game[1], " "), " "))

	return winners, numsWeHave
}

func listIntersect(list1 []string, list2 []string) []string {
	result := []string{}
	for _, l1 := range list1 {
		for _, l2 := range list2 {
			if l1 == l2 {
				result = append(result, l1)
			}
		}
	}
	return result
}

func main() {
	part1(INPUT_FILE)

	file, err := os.Open(INPUT_FILE)
	check(err)
	defer file.Close()

	scanner := bufio.NewScanner(file)
	matchList := []int{}

	for scanner.Scan() {
		matches := listIntersect(parseGame(scanner.Text()))
		matchList = append(matchList, len(matches))
	}

	cards := sumCards(matchList)

	fmt.Printf("Part 2: %d \n", cards)
}
