package user

import (
	"net/http"

	"kubehera/pkg/utils/errors"

	"github.com/emicklei/go-restful"
	restfulspec "github.com/emicklei/go-restful-openapi"
	"gorm.io/gorm"

	"kubehera/pkg/apis"
	serverconfig "kubehera/pkg/server/config"
	"kubehera/pkg/server/runtime"
)

const userTag = "User"

func AddToContainer(container *restful.Container, config *serverconfig.Config, dbClient *gorm.DB) error {
	ws := runtime.NewWebService("users")
	handler := newUserHandler(config, dbClient)

	// users
	ws.Route(ws.GET("/oauth_url").
		To(handler.OauthUrl).
		Doc("Get github Oauth url.").
		Returns(http.StatusOK, apis.StatusOK, User{}).
		Metadata(restfulspec.KeyOpenAPITags, []string{userTag}))
	ws.Route(ws.POST("/login").
		To(handler.OauthLogin).
		Doc("Login by github Oauth.").
		Returns(http.StatusOK, apis.StatusOK, User{}).
		Metadata(restfulspec.KeyOpenAPITags, []string{userTag}))
	ws.Route(ws.GET("/info").
		To(handler.GetUserInfo).
		Doc("Get user info.").
		Returns(http.StatusOK, apis.StatusOK, User{}).
		Metadata(restfulspec.KeyOpenAPITags, []string{userTag}))
	ws.Route(ws.POST("/").
		To(handler.CreateUser).
		Doc("Create a global user account.").
		Returns(http.StatusOK, apis.StatusOK, User{}).
		Reads(User{}).
		Metadata(restfulspec.KeyOpenAPITags, []string{userTag}))
	ws.Route(ws.DELETE("/{user}").
		To(handler.DeleteUser).
		Doc("Delete the specified user.").
		Param(ws.PathParameter("user", "username")).
		Returns(http.StatusOK, apis.StatusOK, errors.None).
		Metadata(restfulspec.KeyOpenAPITags, []string{userTag}))
	ws.Route(ws.PUT("/{user}").
		To(handler.UpdateUser).
		Doc("Update user profile.").
		Reads(User{}).
		Param(ws.PathParameter("user", "username")).
		Returns(http.StatusOK, apis.StatusOK, User{}).
		Metadata(restfulspec.KeyOpenAPITags, []string{userTag}))
	ws.Route(ws.PUT("/{user}/password").
		To(handler.ModifyPassword).
		Doc("Reset password of the specified user.").
		Reads(PasswordReset{}).
		Param(ws.PathParameter("user", "username")).
		Returns(http.StatusOK, apis.StatusOK, errors.None).
		Metadata(restfulspec.KeyOpenAPITags, []string{userTag}))
	ws.Route(ws.GET("/{user}").
		To(handler.DescribeUser).
		Doc("Retrieve user details.").
		Param(ws.PathParameter("user", "username")).
		Returns(http.StatusOK, apis.StatusOK, User{}).
		Metadata(restfulspec.KeyOpenAPITags, []string{userTag}))
	ws.Route(ws.GET("/").
		To(handler.ListUsers).
		Doc("List all users.").
		Returns(http.StatusOK, apis.StatusOK, apis.ListResult{Items: []interface{}{User{}}}).
		Metadata(restfulspec.KeyOpenAPITags, []string{userTag}))

	container.Add(ws)
	return nil
}
