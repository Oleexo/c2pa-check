package main

import (
	"fmt"
	"log/slog"
	"net/http"
	"os"
	"path/filepath"

	"github.com/gin-gonic/gin"
	"github.com/google/uuid"

	"github.com/Oleexo/c2pa-check/internal/c2pa"
)

func getFileExtension(filename string) string {
	if filename == "" {
		return ".jpg"
	}

	return filepath.Ext(filename)
}

func main() {
	router := gin.Default()
	// Set a lower memory limit for multipart forms (default is 32 MiB)
	router.MaxMultipartMemory = 8 << 20 // 8 MiB
	router.POST("/check", func(c *gin.Context) {
		// single file
		file, _ := c.FormFile("file")
		fileExtension := getFileExtension(file.Filename)
		filename := uuid.New().String()
		fullPath := "/tmp/" + filename + fileExtension
		if err := c.SaveUploadedFile(file, fullPath); err != nil {
			slog.Error(err.Error())
			c.String(http.StatusBadRequest, fmt.Sprintf("Cannot save the file to '%s'", fullPath))
			return
		}
		defer func() {
			os.Remove(fullPath)
		}()

		result, err := c2pa.Check(fullPath)
		if err != nil {
			slog.Error(err.Error())
			c.String(http.StatusBadRequest, fmt.Sprintf("Running c2pa check failed on the file. %s", fullPath))
			return
		}

		c.Data(http.StatusOK, "application/json", []byte(result))
	})
	if err := router.Run(":8080"); err != nil {
		panic(err)
	}
}
