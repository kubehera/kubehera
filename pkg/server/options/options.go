package options

import (
	"fmt"

	"github.com/spf13/pflag"
)

type ServerRunOptions struct {
	// server bind address
	BindAddress string

	// port number
	Port int

	SqliteDb string
}

func NewServerRunOptions() *ServerRunOptions {
	// create default server run options
	s := ServerRunOptions{
		BindAddress: "0.0.0.0",
		Port:        3000,
		SqliteDb:    "kubehera.db",
	}

	return &s
}

func (s *ServerRunOptions) Validate() []error {
	errs := []error{}

	if s.Port == 0 {
		errs = append(errs, fmt.Errorf("port can not be disabled"))
	}

	return errs
}

func (s *ServerRunOptions) AddFlags(fs *pflag.FlagSet, c *ServerRunOptions) {

	fs.StringVar(&s.BindAddress, "bind-address", c.BindAddress, "server bind address")
	fs.IntVar(&s.Port, "port", c.Port, "port number")
}
