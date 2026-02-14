package main

import (
	"encoding/json"
	"fmt"
	"io"
	"os"

	"github.com/WebReflection/flatted/golang/pkg/flatted"
)

func main() {
	stat, _ := os.Stdin.Stat()
	if (stat.Mode() & os.ModeCharDevice) != 0 {
		fmt.Fprintln(os.Stderr, "Usage: echo <circular-json> | flatted")
		os.Exit(1)
	}

	input, err := io.ReadAll(os.Stdin)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error reading input: %v\n", err)
		os.Exit(1)
	}

	// Parse the flatted input
	parsed, err := flatted.Parse(string(input), nil)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error parsing flatted data: %v\n", err)
		os.Exit(1)
	}

	// Convert to standard JSON for output.
	// Note: If the revived object is still circular, json.Marshal will error.
	output, err := json.MarshalIndent(parsed, "", "  ")
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error marshaling to JSON: %v\n", err)
		os.Exit(1)
	}

	fmt.Println(string(output))
}
