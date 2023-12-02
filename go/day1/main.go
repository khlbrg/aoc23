package main

import (
	"bufio"
	"bytes"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {

	file := loadFile("input")
	scanner := bufio.NewScanner(file)
	scanner.Split(bufio.ScanLines)

	defer file.Close()

	res := 0
	for scanner.Scan() {
		res += getNumberFromLine(replaceTextCharWithDigit(scanner.Text()))
	}
	// Done, res:  55686
	fmt.Println("Done, res: ", res)
}

func loadFile(path string) *os.File {
	content, err := os.Open("input")
	if err != nil {
		panic(err)
	}
	return content

}

type textDigits map[string]byte

func getTextDigit() textDigits {
	return map[string]byte{
		"one":   '1',
		"two":   '2',
		"three": '3',
		"four":  '4',
		"five":  '5',
		"six":   '6',
		"seven": '7',
		"eight": '8',
		"nine":  '9',
	}
}

func (t textDigits) getDigits() []byte {
	result := []byte{}
	for _, d := range t {
		result = append(result, d)
	}
	return result
}

// replaceTextCharWithDigit transforms all "text numbers" to digits
// while also remove all non digits
func replaceTextCharWithDigit(l string) []byte {
	result := []byte{}
	for i := 0; i < len(l); i++ {
		for key, textDigit := range getTextDigit() {
			if strings.HasPrefix(l[i:], key) {
				result = append(result, textDigit)
				break
			}
		}
		if bytes.ContainsAny(getTextDigit().getDigits(), string(l[i])) {
			result = append(result, l[i])

		}
	}
	return result
}

func getNumberFromLine(l []byte) int {
	numbers := getTextDigit().getDigits()
	var leftRes, rightRes string

	for i := 0; i <= len(l); i++ {
		left := l[i]
		right := l[len(l)-i-1]

		if bytes.ContainsAny(numbers, string(left)) && leftRes == "" {
			leftRes = string(left)
		}

		if bytes.LastIndexAny(numbers, string(right)) > -1 && rightRes == "" {
			rightRes = string(right)
		}

		if leftRes != "" && rightRes != "" {
			break
		}
	}
	result := fmt.Sprintf("%s%s", leftRes, rightRes)
	intResult, _ := strconv.Atoi(result)
	return intResult
}
