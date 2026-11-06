package main

import "testing"

func TestExactSumAndCandidateMutations(t *testing.T) {
	req := request{Version: 1, ID: "sum", Operation: "add", Inputs: []state{{"123", "2", true}, {"234", "2", true}}, Candidate: state{"357", "2", true}}
	if status, _ := verify(req); status != "verified" {
		t.Fatal(status)
	}
	for _, candidate := range []state{{"358", "2", true}, {"3570", "3", true}, {"357", "2", false}} {
		req.Candidate = candidate
		if status, _ := verify(req); status != "mismatch" {
			t.Fatal(status)
		}
	}
	req.Operation = "missing"
	if status, _ := verify(req); status != "unsupported" {
		t.Fatal(status)
	}
}
