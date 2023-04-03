package user

import (
	"kubehera/pkg/models/user"
	"net/http"
	"time"

	"github.com/emicklei/go-restful"
	"gorm.io/gorm"

	serverconfig "kubehera/pkg/server/config"
)

type User struct {
	OauthType   string      `json:"oauth_type"`
	GithubUser  *GithubUser `json:"github_user"`
	AccessToken string      `json:"access_token"`
}

type GithubUser struct {
	ID        int64  `json:"id"`
	Login     string `json:"login"`
	Name      string `json:"name"`
	Email     string `json:"email"`
	AvatarUrl string `json:"avatar_url"`
}

type PasswordReset struct {
	CurrentPassword string `json:"currentPassword"`
	Password        string `json:"password"`
}

type userHandler struct {
	ServerConfig *serverconfig.Config
	DbClient     *gorm.DB
}

func newUserHandler(config *serverconfig.Config, dbClient *gorm.DB) *userHandler {
	return &userHandler{config, dbClient}
}

func (h *userHandler) OauthUrl(request *restful.Request, response *restful.Response) {
	githubLoginUrl := h.GetGithubLoginUrl()
	response.WriteEntity(githubLoginUrl)
}

const githubCookieName = "ACCESS_TOKEN"

func (h *userHandler) OauthLogin(request *restful.Request, response *restful.Response) {

	var code struct {
		Code string `json:"code"`
	}
	request.ReadEntity(&code)
	respUser := h.GithubLoginUser(code.Code)
	maxAge := (time.Hour * 24 * 7).Seconds()
	cookie := http.Cookie{
		Name:     githubCookieName,
		Value:    respUser.AccessToken,
		Path:     "/",
		HttpOnly: true,
		MaxAge:   int(maxAge),
	}
	http.SetCookie(response, &cookie)
	modelUser := user.User{
		ID:        respUser.GithubUser.ID,
		OauthType: respUser.OauthType,
		Login:     respUser.GithubUser.Login,
		Name:      respUser.GithubUser.Name,
		Email:     respUser.GithubUser.Email,
		AvatarUrl: respUser.GithubUser.AvatarUrl,
	}

	h.DbClient.AutoMigrate(&modelUser)
	dest := &user.User{}
	h.DbClient.First(dest, "login = ?", respUser.GithubUser.Login)
	if dest.ID == 0 {
		h.DbClient.Create(&modelUser)
	}
}
func (h *userHandler) GetUserInfo(request *restful.Request, response *restful.Response) {
	cookie, _ := request.Request.Cookie(githubCookieName)
	ak := cookie.Value
	githubUser := getGithubUserMessage(ak)
	dbUser := user.User{}
	h.DbClient.First(&dbUser, githubUser.GithubUser.ID)
	response.WriteEntity(dbUser)
}

func (h *userHandler) CreateUser(request *restful.Request, response *restful.Response) {
	/*
		username := request.PathParameter("user")

		user, err := h.im.DescribeUser(username)
		if err != nil {
			apis.HandleInternalError(response, request, err)
			return
		}

		response.WriteEntity(user)
	*/
}

func (h *userHandler) DeleteUser(request *restful.Request, response *restful.Response) {
}

func (h *userHandler) UpdateUser(request *restful.Request, response *restful.Response) {
}

func (h *userHandler) ModifyPassword(request *restful.Request, response *restful.Response) {
}

func (h *userHandler) DescribeUser(request *restful.Request, response *restful.Response) {
	/*username := request.PathParameter("user")

	user, err := h.im.DescribeUser(username)
	if err != nil {
		apis.HandleInternalError(response, request, err)
		return
	}

	globalRole, err := h.am.GetGlobalRoleOfUser(username)
	// ignore not found error
	if err != nil && !errors.IsNotFound(err) {
		apis.HandleInternalError(response, request, err)
		return
	}
	response.WriteEntity(user)
	*/
}

func (h *userHandler) ListUsers(request *restful.Request, response *restful.Response) {
}
