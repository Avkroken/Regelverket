package main

import (
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"sort"

	"gopkg.in/yaml.v3"
)

type Config struct {
	Repository struct {
		Target string `yaml:"target"`
	} `yaml:"repository"`
	Intent struct {
		Checks struct {
			Required []string `yaml:"required_contexts"`
		} `yaml:"checks"`
	} `yaml:"intent"`
}

type Observed struct {
	Repository struct {
		GitHub struct {
			FullName string `yaml:"full_name"`
		} `yaml:"github"`
	} `yaml:"repository"`
	Rulesets []struct {
		ID    string `yaml:"id"`
		Rules []struct {
			Type       string `yaml:"type"`
			Parameters struct {
				Checks []struct {
					Context string `yaml:"context"`
				} `yaml:"checks"`
			} `yaml:"parameters"`
		} `yaml:"rules"`
	} `yaml:"rulesets"`
}

type ResolvedPolicy struct {
	Schema         string   `json:"schema"`
	Repository     string   `json:"repository"`
	RequiredChecks []string `json:"required_checks"`
	Digest         string   `json:"semantic_digest"`
}

type Operation struct {
	Action   string   `json:"action"`
	Resource string   `json:"resource"`
	Add      []string `json:"add,omitempty"`
	Remove   []string `json:"remove,omitempty"`
}

type Plan struct {
	Schema     string      `json:"schema"`
	Repository string      `json:"repository"`
	NoChanges  bool        `json:"no_changes"`
	Operations []Operation `json:"operations"`
}

func readYAML(path string, out any) error {
	b, err := os.ReadFile(path)
	if err != nil {
		return err
	}
	return yaml.Unmarshal(b, out)
}

func normalized(values []string) []string {
	set := make(map[string]struct{}, len(values))
	for _, value := range values {
		set[value] = struct{}{}
	}
	out := make([]string, 0, len(set))
	for value := range set {
		out = append(out, value)
	}
	sort.Strings(out)
	return out
}

func observedChecks(o Observed) ([]string, error) {
	for _, ruleset := range o.Rulesets {
		if ruleset.ID != "ruleset.main" {
			continue
		}
		for _, rule := range ruleset.Rules {
			if rule.Type != "required_status_checks" {
				continue
			}
			checks := make([]string, 0, len(rule.Parameters.Checks))
			for _, check := range rule.Parameters.Checks {
				checks = append(checks, check.Context)
			}
			return normalized(checks), nil
		}
	}
	return nil, errors.New("ruleset.main required_status_checks not found")
}

func semanticDigest(repository string, checks []string) string {
	payload := struct {
		Repository     string   `json:"repository"`
		RequiredChecks []string `json:"required_checks"`
	}{repository, normalized(checks)}
	b, _ := json.Marshal(payload)
	sum := sha256.Sum256(b)
	return "sha256:" + hex.EncodeToString(sum[:])
}

func diff(current, desired []string) (add, remove []string) {
	currentSet, desiredSet := map[string]bool{}, map[string]bool{}
	for _, value := range current {
		currentSet[value] = true
	}
	for _, value := range desired {
		desiredSet[value] = true
	}
	for value := range desiredSet {
		if !currentSet[value] {
			add = append(add, value)
		}
	}
	for value := range currentSet {
		if !desiredSet[value] {
			remove = append(remove, value)
		}
	}
	sort.Strings(add)
	sort.Strings(remove)
	return add, remove
}

func run(observedPath, configPath string) (any, error) {
	var observed Observed
	var config Config
	if err := readYAML(observedPath, &observed); err != nil {
		return nil, err
	}
	if err := readYAML(configPath, &config); err != nil {
		return nil, err
	}
	if observed.Repository.GitHub.FullName != config.Repository.Target {
		return nil, errors.New("repository target mismatch")
	}
	current, err := observedChecks(observed)
	if err != nil {
		return nil, err
	}
	desired := normalized(config.Intent.Checks.Required)
	add, remove := diff(current, desired)
	plan := Plan{Schema: "regelverket.plan/v0-spike", Repository: config.Repository.Target, NoChanges: len(add) == 0 && len(remove) == 0, Operations: []Operation{}}
	if !plan.NoChanges {
		plan.Operations = append(plan.Operations, Operation{Action: "update", Resource: "ruleset.main.required_status_checks", Add: add, Remove: remove})
	}
	return struct {
		Resolved ResolvedPolicy `json:"resolved_policy"`
		Plan     Plan           `json:"plan"`
	}{
		Resolved: ResolvedPolicy{Schema: "regelverket.resolved-policy/v0-spike", Repository: config.Repository.Target, RequiredChecks: desired, Digest: semanticDigest(config.Repository.Target, desired)},
		Plan:     plan,
	}, nil
}

func main() {
	if len(os.Args) != 3 {
		fmt.Fprintln(os.Stderr, "usage: regelverket-go-spike OBSERVED CONFIG")
		os.Exit(2)
	}
	out, err := run(os.Args[1], os.Args[2])
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(out); err != nil {
		panic(err)
	}
}
