package main

func sumForCard(location int, cards int, input []int) int {

	cards += 1 // count the card itself

	for i := 1; i <= input[location]; i++ {
		cards += sumForCard(location+i, 0, input) // sum up card's matches
	}

	return cards
}

func sumCards(input []int) int {
	sum := 0
	for cardLocation := range input {
		sum += sumForCard(cardLocation, 0, input)
	}
	return sum
}
