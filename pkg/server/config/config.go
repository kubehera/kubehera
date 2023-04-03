package config

import (
	"fmt"
	"reflect"
	"strings"

	"kubehera/pkg/server/options"

	"github.com/spf13/viper"
	"k8s.io/klog/v2"
)

const (
	// DefaultConfigurationName is the default name of configuration
	defaultConfigurationName = "kubehera"

	// DefaultConfigurationPath the default location of the configuration file
	defaultConfigurationPath = "/etc/kubehera"
)

// Config defines everything needed for apiserver to deal with external services
type Config struct {
	GithubOptions *options.GithubOptions `json:"github,omitempty" yaml:"github,omitempty" mapstructure:"github,omitempty"`
}

// newConfig creates a default non-empty Config
func New() *Config {
	return &Config{
		GithubOptions: options.NewGithubOptions(),
		//		KubernetesOptions:     k8s.NewKubernetesOptions(),
		//		AuthenticationOptions: authentication.NewOptions(),
		//		AuthorizationOptions:  authorization.NewOptions(),
	}
}

// TryLoadFromDisk loads configuration from default location after server startup
// return nil error if configuration file not exists
func TryLoadFromDisk() (*Config, error) {
	viper.SetConfigName(defaultConfigurationName)
	viper.AddConfigPath(defaultConfigurationPath)

	// Load from current working directory, only used for debugging
	viper.AddConfigPath(".")

	// Load from Environment variables
	viper.SetEnvPrefix("kubehera")
	viper.AutomaticEnv()
	viper.SetEnvKeyReplacer(strings.NewReplacer(".", "_"))

	if err := viper.ReadInConfig(); err != nil {
		if _, ok := err.(viper.ConfigFileNotFoundError); ok {
			klog.Warningf("%s, and while use the default config", err)
		} else {
			return nil, fmt.Errorf("error parsing configuration file %s", err)
		}
	}

	conf := New()

	if err := viper.Unmarshal(conf); err != nil {
		return nil, err
	}

	return conf, nil
}

// convertToMap simply converts config to map[string]bool
// to hide sensitive information
func (conf *Config) ToMap() map[string]bool {
	conf.stripEmptyOptions()
	result := make(map[string]bool, 0)

	if conf == nil {
		return result
	}

	c := reflect.Indirect(reflect.ValueOf(conf))

	for i := 0; i < c.NumField(); i++ {
		name := strings.Split(c.Type().Field(i).Tag.Get("json"), ",")[0]
		if strings.HasPrefix(name, "-") {
			continue
		}

		if c.Field(i).IsNil() {
			result[name] = false
		} else {
			result[name] = true
		}
	}

	return result
}

// Remove invalid options before serializing to json or yaml
func (conf *Config) stripEmptyOptions() {
}
