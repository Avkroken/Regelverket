package main

import "testing"

func TestDiffDeterministic(t *testing.T) {
	add, remove := diff([]string{"osv", "test", "scope-policy"}, []string{"dependency-review", "test", "osv", "scope-policy"})
	if len(add) != 1 || add[0] != "dependency-review" || len(remove) != 0 {
		t.Fatalf("unexpected diff: add=%v remove=%v", add, remove)
	}
}

func TestDigestOrderIndependent(t *testing.T) {
	a := semanticDigest("Avkroken/dumpen", []string{"test", "osv", "scope-policy"})
	b := semanticDigest("Avkroken/dumpen", []string{"scope-policy", "test", "osv"})
	if a != b {
		t.Fatalf("digest must ignore set order: %s != %s", a, b)
	}
}
