package main

import (
	"log"

	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/fiber/v2/middleware/filesystem"
)

func main() {
	app := fiber.New()

	// Use the filesystem middleware to serve static files from a "public" directory
	app.Use("/", filesystem.New(filesystem.Config{
		Root:   fiber.NewHTTPBox("./public"), // Assumes you have a "public" folder in your project directory
		Browse: true,                         // Enables directory listing
	}))

	app.Get("/", func(c *fiber.Ctx) error {
		return c.SendString("Welcome to the static file server!")
	})

	log.Fatal(app.Listen(":3000"))
}
