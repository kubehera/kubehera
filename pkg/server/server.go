package server

import (
	"bytes"
	"context"
	"fmt"
	"io"
	"net"
	"net/http"
	rt "runtime"
	"time"

	//"github.com/cilium/cilium/pkg/k8s"
	"github.com/emicklei/go-restful"
	"google.golang.org/grpc"
	"gorm.io/gorm"
	"k8s.io/apiserver/pkg/endpoints/handlers/responsewriters"
	"k8s.io/klog"

	"kubehera/pkg/apis/user"
	"kubehera/pkg/proto/echo"
	"kubehera/pkg/utils/iputil"

	urlruntime "k8s.io/apimachinery/pkg/util/runtime"
	runtimecache "sigs.k8s.io/controller-runtime/pkg/cache"
	runtimeclient "sigs.k8s.io/controller-runtime/pkg/client"

	serverconfig "kubehera/pkg/server/config"
)

type APIServer struct {
	// number of kubesphere apiserver
	ServerCount int

	Server *http.Server

	Config *serverconfig.Config

	// webservice container, where all webservice defines
	container *restful.Container

	// kubeClient is a collection of all kubernetes(include CRDs) objects clientset
	//	KubernetesClient k8s.Client
	DbClient *gorm.DB

	// controller-runtime cache
	RuntimeCache runtimecache.Cache

	// controller-runtime client
	RuntimeClient runtimeclient.Client
}

func (s *APIServer) PrepareRun(stopCh <-chan struct{}) error {
	s.container = restful.NewContainer()
	s.container.Filter(logRequestAndResponse)
	s.container.Router(restful.CurlyRouter{})
	s.container.RecoverHandler(func(panicReason interface{}, httpWriter http.ResponseWriter) {
		logStackOnRecover(panicReason, httpWriter)
	})
	s.installKubeheraAPIs()

	for _, ws := range s.container.RegisteredWebServices() {
		klog.V(2).Infof("%s", ws.RootPath())
	}

	s.Server.Handler = s.container

	s.buildHandlerChain(stopCh)

	return nil
}

// Install all kubesphere api groups
// Installation happens before all informers start to cache objects, so
//   any attempt to list objects using listers will get empty results.
func (s *APIServer) installKubeheraAPIs() {
	urlruntime.Must(user.AddToContainer(s.container, s.Config, s.DbClient))
}

type EchoServiceImpl struct{}

func (p *EchoServiceImpl) BidirectionalStreamingEcho(stream echo.Echo_BidirectionalStreamingEchoServer) error {
	for {
		args, err := stream.Recv()
		if err != nil {
			if err == io.EOF {
				return nil
			}
			return err
		}

		reply := &echo.EchoResponse{Message: "hello:" + args.GetMessage()}

		err = stream.Send(reply)
		if err != nil {
			return err
		}
	}
}

func (s *APIServer) runGrpcServer() {
	grpcServer := grpc.NewServer()
	echo.RegisterEchoServer(grpcServer, new(EchoServiceImpl))

	klog.V(0).Infof("Start grpc server listening on tcp:1234")
	lis, err := net.Listen("tcp", ":1234")
	if err != nil {
		klog.Fatal(err)
	}
	grpcServer.Serve(lis)
}

func (s *APIServer) Run(ctx context.Context) (err error) {

	err = s.waitForResourceSync(ctx)
	if err != nil {
		return err
	}

	shutdownCtx, cancel := context.WithCancel(context.Background())
	defer cancel()

	go func() {
		<-ctx.Done()
		_ = s.Server.Shutdown(shutdownCtx)
	}()

	go s.runGrpcServer()

	klog.V(0).Infof("Start listening on %s", s.Server.Addr)
	err = s.Server.ListenAndServe()

	return err
}

func (s *APIServer) buildHandlerChain(stopCh <-chan struct{}) {
	handler := s.Server.Handler
	s.Server.Handler = handler
}

func (s *APIServer) waitForResourceSync(ctx context.Context) error {
	klog.V(0).Info("Start cache objects")

	//stopCh := ctx.Done()

	/*
		discoveryClient := s.KubernetesClient.Kubernetes().Discovery()
		_, apiResourcesList, err := discoveryClient.ServerGroupsAndResources()
		if err != nil {
			return err
		}

		isResourceExists := func(resource schema.GroupVersionResource) bool {
			for _, apiResource := range apiResourcesList {
				if apiResource.GroupVersion == resource.GroupVersion().String() {
					for _, rsc := range apiResource.APIResources {
						if rsc.Name == resource.Resource {
							return true
						}
					}
				}
			}
			return false
		}*/

	// resources we have to create informer first
	/*	k8sGVRs := []schema.GroupVersionResource{
			{Group: "", Version: "v1", Resource: "namespaces"},
			{Group: "", Version: "v1", Resource: "nodes"},
			{Group: "", Version: "v1", Resource: "resourcequotas"},
			{Group: "", Version: "v1", Resource: "pods"},
			{Group: "", Version: "v1", Resource: "services"},
			{Group: "", Version: "v1", Resource: "persistentvolumeclaims"},
			{Group: "", Version: "v1", Resource: "persistentvolumes"},
			{Group: "", Version: "v1", Resource: "secrets"},
			{Group: "", Version: "v1", Resource: "configmaps"},
			{Group: "", Version: "v1", Resource: "serviceaccounts"},

			{Group: "rbac.authorization.k8s.io", Version: "v1", Resource: "roles"},
			{Group: "rbac.authorization.k8s.io", Version: "v1", Resource: "rolebindings"},
			{Group: "rbac.authorization.k8s.io", Version: "v1", Resource: "clusterroles"},
			{Group: "rbac.authorization.k8s.io", Version: "v1", Resource: "clusterrolebindings"},
			{Group: "apps", Version: "v1", Resource: "deployments"},
			{Group: "apps", Version: "v1", Resource: "daemonsets"},
			{Group: "apps", Version: "v1", Resource: "replicasets"},
			{Group: "apps", Version: "v1", Resource: "statefulsets"},
			{Group: "apps", Version: "v1", Resource: "controllerrevisions"},
			{Group: "storage.k8s.io", Version: "v1", Resource: "storageclasses"},
			{Group: "batch", Version: "v1", Resource: "jobs"},
			{Group: "batch", Version: "v1beta1", Resource: "cronjobs"},
			{Group: "networking.k8s.io", Version: "v1", Resource: "ingresses"},
			{Group: "autoscaling", Version: "v2beta2", Resource: "horizontalpodautoscalers"},
			{Group: "networking.k8s.io", Version: "v1", Resource: "networkpolicies"},
		}

		for _, gvr := range k8sGVRs {
			if !isResourceExists(gvr) {
				klog.Warningf("resource %s not exists in the cluster", gvr)
			} else {
				_, err := s.InformerFactory.KubernetesSharedInformerFactory().ForResource(gvr)
				if err != nil {
					klog.Errorf("cannot create informer for %s", gvr)
					return err
				}
			}
	*/
	//}

	//s.InformerFactory.KubernetesSharedInformerFactory().Start(stopCh)
	//s.InformerFactory.KubernetesSharedInformerFactory().WaitForCacheSync(stopCh)

	// controller runtime cache for resources
	//go s.RuntimeCache.Start(ctx)
	//s.RuntimeCache.WaitForCacheSync(ctx)

	klog.V(0).Info("Finished caching objects")

	return nil

}

func logStackOnRecover(panicReason interface{}, w http.ResponseWriter) {
	var buffer bytes.Buffer
	buffer.WriteString(fmt.Sprintf("recover from panic situation: - %v\r\n", panicReason))
	for i := 2; ; i += 1 {
		_, file, line, ok := rt.Caller(i)
		if !ok {
			break
		}
		buffer.WriteString(fmt.Sprintf("    %s:%d\r\n", file, line))
	}
	klog.Errorln(buffer.String())

	headers := http.Header{}
	if ct := w.Header().Get("Content-Type"); len(ct) > 0 {
		headers.Set("Accept", ct)
	}

	w.WriteHeader(http.StatusInternalServerError)
	w.Write([]byte("Internal server error"))
}

func logRequestAndResponse(req *restful.Request, resp *restful.Response, chain *restful.FilterChain) {
	start := time.Now()
	chain.ProcessFilter(req, resp)

	// Always log error response
	logWithVerbose := klog.V(4)
	if resp.StatusCode() > http.StatusBadRequest {
		logWithVerbose = klog.V(0)
	}

	logWithVerbose.Infof("%s - \"%s %s %s\" %d %d %dms",
		iputil.RemoteIp(req.Request),
		req.Request.Method,
		req.Request.URL,
		req.Request.Proto,
		resp.StatusCode(),
		resp.ContentLength(),
		time.Since(start)/time.Millisecond,
	)
}

type errorResponder struct{}

func (e *errorResponder) Error(w http.ResponseWriter, req *http.Request, err error) {
	klog.Error(err)
	responsewriters.InternalError(w, req, err)
}
