package main

import (
	"fmt"
	"net"
	"os"
	"os/user"
	"syscall"
)

func run() error {
	if len(os.Args) == 1 {
		return fmt.Errorf("usage: %s TARGET", os.Args[0])
	}
	target := os.Args[1]
	if err := syscall.Chroot(target); err != nil {
		return err
	}

	var errs []error

	redhat, err := net.LookupHost("redhat.com")
	if err != nil {
		errs = append(errs, err)
	} else {
		fmt.Printf("ok redhat %v\n", redhat)
	}

	localhost, err := net.LookupHost("localhost")
	if err != nil {
		errs = append(errs, err)
	} else {
		fmt.Printf("ok localhost %v\n", localhost)
	}

	root, err := user.Lookup("root")
	if err != nil {
		errs = append(errs, err)
	} else {
		fmt.Printf("ok root user %v\n", root)
	}

	if errs != nil {
		for _, err := range errs { 
			fmt.Fprintf(os.Stderr, "not ok %v\n", err)
		}
		return fmt.Errorf("failed test")
	}

	return nil
}

func main() {
	if err := run(); err != nil {
		fmt.Fprintf(os.Stderr, "%v\n", err)
		os.Exit(1)
	}
}
