package main

import "testing"

func TestReplace(t *testing.T) {

	tests := []struct {
		input    []byte
		expected []byte
	}{
		{
			input:    []byte("zoneight234"),
			expected: []byte("18234"),
		},
		{
			input:    []byte("46nine"),
			expected: []byte("469"),
		},
		{
			input:    []byte("xtwone3four"),
			expected: []byte("2134"),
		},

		{
			input:    []byte("two1nine"),
			expected: []byte("219"),
		},
		{
			input:    []byte("7pqrstsixteen"),
			expected: []byte("76"),
		},
		{
			input:    []byte("zoneight234"),
			expected: []byte("18234"),
		},
		{
			input:    []byte("4nineeightseven2"),
			expected: []byte("49872"),
		},
		{
			input:    []byte("xtwone3four"),
			expected: []byte("2134"),
		},
		{
			input:    []byte("nine9two7xkjdlrrpgxbcfmpfmzsevenkkzxnxbfour"),
			expected: []byte("992774"),
		},
		{
			input:    []byte("eightwothree"),
			expected: []byte("823"),
		},
	}

	for _, test := range tests {
		if got := replaceTextCharWithDigit(string(test.input)); string(got) != string(test.expected) {
			t.Errorf("%s failed: wanted: %s %s  got: %s",
				t.Name(),
				string(test.input),
				string(test.expected),
				string(got))
		}
	}
}

func TestGetNumberFromLine(t *testing.T) {

	tests := []struct {
		input    []byte
		expected int
	}{
		{
			input:    []byte("46nine"),
			expected: 49,
		},
		{
			input:    []byte("1abc2"),
			expected: 12,
		},
		{
			input:    []byte("pqr3stu8vwx"),
			expected: 38,
		},
		{
			input:    []byte("a1b2c3d4e5f"),
			expected: 15,
		},
		{
			input:    []byte("treb7uchet"),
			expected: 77,
		},
		{
			input:    []byte("554"),
			expected: 54,
		},

		{
			input:    []byte("pmsixgr1"),
			expected: 61,
		},
		{
			input:    []byte("two1nine"),
			expected: 29,
		},
		{
			input:    []byte("4nineeightseven2"),
			expected: 42,
		},
		{
			input:    []byte("7pqrstsixteen"),
			expected: 76,
		},
		{
			input:    []byte("zoneight234"),
			expected: 14,
		},
		{
			input:    []byte("4nineeightseven2"),
			expected: 42,
		},
		{
			input:    []byte("xtwone3four"),
			expected: 24,
		},
		{
			input:    []byte("nine9two7xkjdlrrpgxbcfmpfmzsevenkkzxnxbfour"),
			expected: 94,
		},
		{
			input:    []byte("eightwothree"),
			expected: 83,
		},
		{
			input:    []byte("34292"),
			expected: 32,
		},
		{
			input:    []byte("nc7"),
			expected: 77,
		},
		{
			input:    []byte("six28four3"),
			expected: 63,
		},
		{
			input:    []byte("cntwo15"),
			expected: 25,
		},
		{
			input:    []byte("13twotwonec"),
			expected: 11,
		},
		{
			input:    []byte("ninethree9ljdjqpbbptwo"),
			expected: 92,
		},
		{
			input:    []byte("3twotwo"),
			expected: 32,
		},
	}

	for _, test := range tests {
		if got := getNumberFromLine(replaceTextCharWithDigit(string(test.input))); got != test.expected {
			t.Errorf("%s failed: wanted: %d  got: %d", t.Name(), test.expected, got)
		}
	}

}
