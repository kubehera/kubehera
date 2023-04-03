package options

import (
	"fmt"

	"github.com/spf13/pflag"
)

type GithubOptions struct {
	ClientID     string `json:"client_id" yaml:"client_id" mapstructure:"client_id"`
	ClientSecret string `json:"client_secret" yaml:"client_secret" mapstructure:"client_secret"`
	RedirectURI  string `json:"redirect_uri" yaml:"redirect_uri" mapstructure:"redirect_uri"`
}

func NewGithubOptions() *GithubOptions {
	// create default server run options
	s := GithubOptions{
		ClientID:     "",
		ClientSecret: "",
		RedirectURI:  "",
	}

	return &s
}

func (s *GithubOptions) Validate() []error {
	errs := []error{}

	if s.ClientID == "" {
		errs = append(errs, fmt.Errorf("client id can not be disabled"))
	}
	if s.ClientSecret == "" {
		errs = append(errs, fmt.Errorf("client secret can not be disabled"))
	}

	return errs
}

func (s *GithubOptions) AddFlags(fs *pflag.FlagSet, c *GithubOptions) {

	fs.StringVar(&s.ClientID, "client-id", c.ClientID, "github client id")
	fs.StringVar(&s.ClientSecret, "client-secret", c.ClientSecret, "github client secret")
}
