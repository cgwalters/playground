package main

import (
	"fmt"
	"io"
	"os"

	"github.com/klauspost/compress/zstd"
)

func run() error {
	encoder, err := zstd.NewWriter(os.Stdout)
	if err != nil {
		return err
	}
	f, err := os.Open(os.Args[1])
	if err != nil {
		return err
	}
	defer f.Close()

	if _, err := io.Copy(encoder, f); err != nil {
		return err
	}
	return encoder.Flush()
}

func main() {
	if err := run(); err != nil {
		fmt.Fprintf(os.Stderr, "error: %v\n", err)
		os.Exit(1)
	}
}
