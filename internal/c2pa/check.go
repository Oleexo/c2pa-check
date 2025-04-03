package c2pa

import (
	"fmt"
	"log/slog"
	"os"
	"os/exec"
)

var c2paToolPath = "/home/maxime/Downloads/c2patool/c2patool"

func init() {
	fromEnv := os.Getenv("C2PA_TOOL_PATH")
	if fromEnv != "" {
		c2paToolPath = fromEnv
	}
}

func Check(path string) (string, error) {
	slog.Info(fmt.Sprintf("Run %s %s", c2paToolPath, path))
	cmd := exec.Command(c2paToolPath, path)
	output, err := cmd.CombinedOutput()
	if err != nil {
		slog.Warn(string(output))
		return "", err
	}
	return string(output), nil
}
