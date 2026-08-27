package main

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"os"
	"strings"
	"time"
)

const apiVersion = "2026-03-10"

type Result struct {
	Status     int    `json:"status"`
	Class      string `json:"class"`
	Repository string `json:"repository,omitempty"`
}

func classify(status int) string {
	switch status {
	case 200:
		return "ok"
	case 401:
		return "authentication_failed"
	case 403:
		return "permission_denied"
	case 404:
		return "not_found_or_inaccessible"
	case 422:
		return "validation_failed"
	case 429:
		return "rate_limited"
	}
	if status >= 500 {
		return "github_service_error"
	}
	if status >= 400 {
		return "request_rejected"
	}
	return "unexpected_status"
}

func get(url, token string) (Result, error) {
	req, err := http.NewRequest(http.MethodGet, url, nil)
	if err != nil {
		return Result{}, err
	}
	req.Header.Set("Accept", "application/vnd.github+json")
	req.Header.Set("X-GitHub-Api-Version", apiVersion)
	req.Header.Set("User-Agent", "regelverket-go-adapter-spike")
	if token != "" {
		req.Header.Set("Authorization", "Bearer "+token)
	}

	client := &http.Client{Timeout: 15 * time.Second}
	resp, err := client.Do(req)
	if err != nil {
		return Result{Class: "transport_error"}, err
	}
	defer resp.Body.Close()
	result := Result{Status: resp.StatusCode, Class: classify(resp.StatusCode)}
	if resp.StatusCode != http.StatusOK {
		return result, nil
	}
	body, err := io.ReadAll(io.LimitReader(resp.Body, 1<<20))
	if err != nil {
		return Result{}, err
	}
	if strings.Contains(string(body), "Avkroken/Regelverket") {
		result.Repository = "Avkroken/Regelverket"
	}
	return result, nil
}

func main() {
	if len(os.Args) != 2 {
		fmt.Fprintln(os.Stderr, "usage: github-adapter-go URL")
		os.Exit(2)
	}
	result, err := get(os.Args[1], os.Getenv("GITHUB_TOKEN"))
	if err != nil && result.Class != "transport_error" {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
	if err := json.NewEncoder(os.Stdout).Encode(result); err != nil {
		panic(err)
	}
}
