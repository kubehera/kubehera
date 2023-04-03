package user

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"kubehera/pkg/utils/strutil"
	"net/http"
	"net/url"
)

func (h *userHandler) GetGithubLoginUrl() string {
	state := strutil.RandStr(10)
	gconf := h.ServerConfig.GithubOptions
	return "https://github.com/login/oauth/authorize?client_id=" + gconf.ClientID + "&redirect_uri=" + gconf.RedirectURI + "&state=" + state
}

func (h *userHandler) GithubLoginUser(code string) *User {

	// get access_token
	gconf := h.ServerConfig.GithubOptions
	loginUrl := "https://github.com/login/oauth/access_token?client_id=" + gconf.ClientID + "&client_secret=" + gconf.ClientSecret + "&code=" + code

	response, err := http.PostForm(loginUrl, url.Values{
		"client_id":     {gconf.ClientID},
		"client_secret": {gconf.ClientSecret},
		"code":          {code},
	})

	if err != nil {
		fmt.Println("post error!", err.Error())
		return nil
	}
	defer response.Body.Close()

	resp, _ := ioutil.ReadAll(response.Body)
	respMap := strutil.ConvertToMap(string(resp))

	ak := respMap["access_token"]

	return getGithubUserMessage(ak)

}

// get user data
func getGithubUserMessage(accessToken string) *User {
	// 	Authorization: Bearer OAUTH-TOKEN
	// GET https://api.github.com/user
	githubUser := &GithubUser{}

	client := &http.Client{}
	reqest, err := http.NewRequest("GET", "https://api.github.com/user", nil)

	if err != nil {
		panic(err)
	}

	reqest.Header.Add("Authorization", "token "+accessToken)
	response, _ := client.Do(reqest)

	if err != nil {
		fmt.Println("GetMessage Err", err.Error())
		return nil
	}

	defer response.Body.Close()
	resp, _ := ioutil.ReadAll(response.Body)

	_ = json.Unmarshal(resp, &githubUser)
	user := &User{
		OauthType:   "github",
		GithubUser:  githubUser,
		AccessToken: accessToken,
	}

	return user
}
