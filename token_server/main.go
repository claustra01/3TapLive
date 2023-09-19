package main

import (
	"github.com/labstack/echo/v4"
)

func GetToken(c echo.Context) error {
	return c.String(200, "Hello, World!")
}

func main() {
	e := echo.New()
	e.GET("/", GetToken)
	e.Start(":8080")
}
