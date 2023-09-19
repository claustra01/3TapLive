package main

import (
	"os"
	"time"

	rtctokenbuilder "github.com/AgoraIO/Tools/DynamicKey/AgoraDynamicKey/go/src/RtcTokenBuilder"
	"github.com/joho/godotenv"
	"github.com/labstack/echo/v4"
)

// roles
const (
	RoleAdmin = 101
)

func getToken(c echo.Context) error {
	// get params
	channelName := c.QueryParam("name")
	if channelName == "" {
		return c.JSON(400, map[string]string{
			"message": "name query is required",
		})
	}
	// get secrets
	godotenv.Load()
	appId := os.Getenv("AGORA_APP_ID")
	appCertificate := os.Getenv("AGORA_APP_CERTIFICATE")
	// generate token
	uid := uint32(0) // local user
	expire := uint32(time.Now().UTC().Unix()) + uint32(60*60)
	token, err := rtctokenbuilder.BuildTokenWithUID(appId, appCertificate, channelName, uid, rtctokenbuilder.RoleAdmin, expire)
	if err != nil {
		return c.JSON(500, map[string]string{
			"message": "failed to build token",
		})
	}
	// return token
	return c.JSON(200, map[string]string{
		"token": token,
	})
}

func main() {
	e := echo.New()
	e.GET("/token", getToken)
	e.Start(":8080")
}
