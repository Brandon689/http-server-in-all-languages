package main

import (
	"net/http"
)

func main() {
	// Create a file server handler for the "static" directory.
	fs := http.FileServer(http.Dir("static"))

	// Handle the "/" route and serve the index.html file.
	http.Handle("/", fs)

	// Start the HTTP server on port 8080.
	port := ":8080"
	println("Server listening on", port)
	err := http.ListenAndServe(port, nil)
	if err != nil {
		panic(err)
	}
}
