package main

import (
	"encoding/json"
	"flag"
	"fmt"
	"io"
	"os"

	"github.com/WebReflection/flatted/golang/pkg/flatted"
)

// flatten reads standard JSON from r and writes flatted JSON to w.
func flatten(r io.Reader, w io.Writer) error {
	input, err := io.ReadAll(r)
	if err != nil {
		return err
	}
	var data any
	if err := json.Unmarshal(input, &data); err != nil {
		return fmt.Errorf("invalid JSON input: %w", err)
	}
	s, err := flatted.Stringify(data, nil, nil)
	if err != nil {
		return err
	}
	fmt.Fprintln(w, s)
	return nil
}

// unflatten reads flatted JSON from r and writes standard JSON to w.
func unflatten(r io.Reader, w io.Writer) error {
	input, err := io.ReadAll(r)
	if err != nil {
		return err
	}
	parsed, err := flatted.Parse(string(input), nil)
	if err != nil {
		return fmt.Errorf("invalid flatted input: %w", err)
	}
	output, err := json.MarshalIndent(parsed, "", "  ")
	if err != nil {
		return err
	}
	fmt.Fprintln(w, string(output))
	return nil
}

func main() {
	var decompress bool
	flag.BoolVar(&decompress, "d", false, "decompress (unflatten)")
	flag.BoolVar(&decompress, "decompress", false, "decompress (unflatten)")
	flag.BoolVar(&decompress, "unflatten", false, "decompress (unflatten)")

	flag.Usage = func() {
		fmt.Fprintf(os.Stderr, "Usage: %s [OPTION]... [FILE]\n", os.Args[0])
		fmt.Fprintln(os.Stderr, "Flatten or unflatten circular JSON structures.")
		fmt.Fprintln(os.Stderr, "")
		fmt.Fprintln(os.Stderr, "Options:")
		flag.PrintDefaults()
		fmt.Fprintln(os.Stderr, "")
		fmt.Fprintln(os.Stderr, "If no FILE is provided, or if FILE is -, read from standard input.")
	}

	flag.Parse()

	var r io.Reader = os.Stdin
	if flag.NArg() > 0 && flag.Arg(0) != "-" {
		f, err := os.Open(flag.Arg(0))
		if err != nil {
			fmt.Fprintf(os.Stderr, "%s: %v\n", os.Args[0], err)
			os.Exit(1)
		}
		defer f.Close()
		r = f
	}

	var err error
	if decompress {
		err = unflatten(r, os.Stdout)
	} else {
		err = flatten(r, os.Stdout)
	}

	if err != nil {
		fmt.Fprintf(os.Stderr, "%s: %v\n", os.Args[0], err)
		os.Exit(1)
	}
}
