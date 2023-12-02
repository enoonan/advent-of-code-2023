package part2

import (
	"bufio"
	"day2/util"
	"fmt"
	"os"
)

func Run() {
	file, err := os.Open("input.txt")
	util.Check(err)
	defer file.Close()

	scanner := bufio.NewScanner(file)
	resultSum := 0

	for scanner.Scan() {
		game := util.NewGame(scanner.Text())
		resultSum += game.Reveals.MinBagNeeded().ThePOWER()
	}

	fmt.Println(resultSum)
}
