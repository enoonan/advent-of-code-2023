package part1

import (
	"bufio"
	"day2/util"
	"fmt"
	"os"
	"strconv"
)

func Run() {
	file, err := os.Open("input.txt")
	util.Check(err)
	defer file.Close()

	TestBag := util.Bag{Red: 12, Green: 13, Blue: 14}
	scanner := bufio.NewScanner(file)
	resultSum := 0

	for scanner.Scan() {
		game := util.NewGame(scanner.Text())
		if game.IsPossible(TestBag) {
			id, err := strconv.Atoi(game.Id)
			util.Check(err)
			resultSum += id
		}
	}

	fmt.Println(resultSum)
}
