package main

type RowNum struct {
	Row   int
	Index int
	Value string
}

func (r *RowNum) ColRange() [2]int {
	var coords [2]int
	coords[0] = r.Index
	coords[1] = r.Index + len(r.Value) - 1
	return coords
}

func (r *RowNum) EndIndex() int {
	return r.Index + len(r.Value)
}

type InputMatrix [][]string

func (r *RowNum) IsToolInMatrix(i InputMatrix) bool {

	rowRange := makeRange(max(r.Row-1, 0), min(r.Row+1, len(i)-1))
	colRange := makeRange(max(r.Index-1, 0), min(r.EndIndex(), len(i[0])-1))
	for _, row := range rowRange {
		for _, col := range colRange {
			if isSymbol(i[row][col]) {
				return true
			}
		}
	}

	return false
}
