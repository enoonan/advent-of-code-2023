package util

import (
	"regexp"
	"slices"
	"strconv"
	"strings"
)

func Check(err error) {
	if err != nil {
		panic(err)
	}
}

type Bag struct {
	Red   int
	Green int
	Blue  int
}

func NewBag() Bag {
	return Bag{Red: 0, Green: 0, Blue: 0}
}

func (b Bag) CanContain(otherBag Bag) bool {
	return b.Red >= otherBag.Red && b.Green >= otherBag.Green && b.Blue >= otherBag.Blue
}

func (b Bag) ThePOWER() int {
	result := 1
	if b.Red > 0 {
		result *= b.Red
	}
	if b.Green > 0 {
		result *= b.Green
	}
	if b.Blue > 0 {
		result *= b.Blue
	}

	return result
}

func bagFromString(str string) Bag {
	sets := strings.Split(strings.Trim(str, " "), ", ")
	bag := Bag{}
	for _, set := range sets {
		vals := strings.Split(set, " ")
		switch vals[1] {
		case "red":
			red, err := strconv.Atoi(vals[0])
			Check(err)
			bag.Red = red
		case "green":
			green, err := strconv.Atoi(vals[0])
			Check(err)
			bag.Green = green
		case "blue":
			blue, err := strconv.Atoi(vals[0])
			Check(err)
			bag.Blue = blue
		}
	}
	return bag
}

func bagListFromString(str string) []Bag {
	sets := strings.Split(str, ";")
	var result []Bag
	for _, set := range sets {
		result = append(result, bagFromString(set))
	}

	return result
}

type RevealList struct {
	Bags []Bag
}

func (rl RevealList) MinBagNeeded() Bag {
	return Bag{
		Red:   slices.Max(rl.Reds()),
		Green: slices.Max(rl.Greens()),
		Blue:  slices.Max(rl.Blues()),
	}
}

func (rl RevealList) Reds() []int {
	var result []int
	for _, bag := range rl.Bags {
		result = append(result, bag.Red)
	}
	return result
}

func (rl RevealList) Greens() []int {
	var result []int
	for _, bag := range rl.Bags {
		result = append(result, bag.Green)
	}
	return result
}

func (rl RevealList) Blues() []int {
	var result []int
	for _, bag := range rl.Bags {
		result = append(result, bag.Blue)
	}
	return result
}

type Game struct {
	Id      string
	Reveals RevealList
}

func (g Game) IsPossible(b Bag) bool {
	for _, gameBag := range g.Reveals.Bags {
		if !b.CanContain(gameBag) {
			return false
		}
	}
	return true
}

var strNumRegex = regexp.MustCompile(`[0-9]+`)

func intFromString(str string) string {
	matches := strNumRegex.FindAllString(str, 1)
	return matches[0]
}

func NewGame(line string) Game {
	game := Game{}
	gameSplit := strings.Split(line, ":")
	game.Id = intFromString(gameSplit[0])
	game.Reveals.Bags = bagListFromString(gameSplit[1])
	return game
}
