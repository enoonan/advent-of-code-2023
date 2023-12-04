package main

func makeRange(start int, end int) []int {
	var result []int

	for i := start; i <= end; i++ {
		result = append(result, i)
	}

	return result
}

func check(err error) {
	if err != nil {
		panic(err)
	}
}

func isDigit(ch string) bool {
	for _, val := range []string{"0", "1", "2", "3", "4", "5", "6", "7", "8", "9"} {
		if ch == val {
			return true
		}
	}
	return false
}

func isSymbol(ch string) bool {
	return ch != "." && !isDigit(ch)
}
