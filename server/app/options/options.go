package options

import (
	"flag"
	"fmt"
	"net/http"

	"gorm.io/driver/sqlite"
	"gorm.io/gorm"
	cliflag "k8s.io/component-base/cli/flag"
	"k8s.io/klog"

	//"kubehera/pkg/apis"
	"kubehera/pkg/server"
	serverconfig "kubehera/pkg/server/config"

	genericoptions "kubehera/pkg/server/options"

	"strings"
)

type ServerRunOptions struct {
	ConfigFile              string
	GenericServerRunOptions *genericoptions.ServerRunOptions
	*serverconfig.Config

	DebugMode bool
}

func NewServerRunOptions() *ServerRunOptions {
	s := &ServerRunOptions{
		GenericServerRunOptions: genericoptions.NewServerRunOptions(),
		Config:                  serverconfig.New(),
	}

	return s
}

func (s *ServerRunOptions) Flags() (fss cliflag.NamedFlagSets) {
	fs := fss.FlagSet("generic")
	fs.BoolVar(&s.DebugMode, "debug", false, "Don't enable this if you don't know what it means.")
	s.GenericServerRunOptions.AddFlags(fs, s.GenericServerRunOptions)
	s.GithubOptions.AddFlags(fss.FlagSet("github"), s.GithubOptions)
	//s.KubernetesOptions.AddFlags(fss.FlagSet("kubernetes"), s.KubernetesOptions)
	//s.AuthorizationOptions.AddFlags(fss.FlagSet("authorization"), s.AuthorizationOptions)

	fs = fss.FlagSet("klog")
	local := flag.NewFlagSet("klog", flag.ExitOnError)
	klog.InitFlags(local)
	local.VisitAll(func(fl *flag.Flag) {
		fl.Name = strings.Replace(fl.Name, "_", "-", -1)
		fs.AddGoFlag(fl)
	})

	return fss
}

const fakeInterface string = "FAKE"

// NewAPIServer creates an APIServer instance using given options
func (s *ServerRunOptions) NewAPIServer(stopCh <-chan struct{}) (*server.APIServer, error) {
	apiServer := &server.APIServer{
		Config: s.Config,
	}

	dbClient, err := gorm.Open(sqlite.Open(s.GenericServerRunOptions.SqliteDb), &gorm.Config{})
	if err != nil {
		panic("failed to connect sqlite database")
	}
	apiServer.DbClient = dbClient

	server := &http.Server{
		Addr: fmt.Sprintf("%s:%d", s.GenericServerRunOptions.BindAddress, s.GenericServerRunOptions.Port),
	}

	apiServer.Server = server
	/*
		kubernetesClient, err := k8s.NewKubernetesClient(s.KubernetesOptions)
		if err != nil {
			return nil, err
		}
		apiServer.KubernetesClient = kubernetesClient

		informerFactory := informers.NewInformerFactories(kubernetesClient.Kubernetes(), kubernetesClient.KubeSphere(),
			kubernetesClient.Istio(), kubernetesClient.Snapshot(), kubernetesClient.ApiExtensions(), kubernetesClient.Prometheus())
		apiServer.InformerFactory = informerFactory


		server := &http.Server{
			Addr: fmt.Sprintf(":%d", s.GenericServerRunOptions.InsecurePort),
		}

		if s.GenericServerRunOptions.SecurePort != 0 {
			certificate, err := tls.LoadX509KeyPair(s.GenericServerRunOptions.TlsCertFile, s.GenericServerRunOptions.TlsPrivateKey)
			if err != nil {
				return nil, err
			}

			server.TLSConfig = &tls.Config{
				Certificates: []tls.Certificate{certificate},
			}
			server.Addr = fmt.Sprintf(":%d", s.GenericServerRunOptions.SecurePort)
		}

	*/
	//sch := scheme.Scheme
	//if err := apis.AddToScheme(sch); err != nil {
	//	klog.Fatalf("unable add APIs to scheme: %v", err)
	//}
	/*apiServer.RuntimeCache, err := runtimecache.New(apiServer.KubernetesClient.Config(), runtimecache.Options{Scheme: sch})
	if err != nil {
		klog.Fatalf("unable to create controller runtime cache: %v", err)
	}

	apiServer.RuntimeClient, err := runtimeclient.New(apiServer.KubernetesClient.Config(), runtimeclient.Options{Scheme: sch})
	if err != nil {
		klog.Fatalf("unable to create controller runtime client: %v", err)
	}

	apiServer.Server = server
	*/
	return apiServer, nil
}
