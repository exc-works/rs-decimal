package main

import "testing"

func TestExactSumAndCandidateMutations(t *testing.T) {
	req := request{Version: 1, ID: "sum", Operation: "add", Inputs: []state{{"123", "2", true}, {"234", "2", true}}, Candidate: state{"357", "2", true}}
	if status, _ := verify(req); status != "verified" {
		t.Fatal(status)
	}
	for _, candidate := range []state{{"358", "2", true}, {"3570", "3", true}, {"357", "2", false}, {"+0357", "02", true}, {"357", "02", true}, {"+357", "2", true}} {
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

func TestRootCellsAndMutations(t *testing.T) {
	for _, tc := range []struct{ input, scale, degree, prec, coefficient string }{
		{"225", "2", "2", "0", "2"}, {"625", "2", "2", "0", "2"},
		{"-3375", "3", "3", "0", "-2"}, {"2", "2", "2", "2", "14"},
	} {
		req := request{Version: 1, ID: "root", Operation: "root_prec", Inputs: []state{{tc.input, tc.scale, true}}, Params: map[string]string{"degree": tc.degree, "precision": tc.prec}, Candidate: state{tc.coefficient, tc.prec, true}}
		if status, detail := verify(req); status != "verified" {
			t.Fatal(tc, status, detail)
		}
		for _, candidate := range []state{{"99", tc.prec, true}, {tc.coefficient, tc.prec, false}, {tc.coefficient, "3", true}, {"+" + tc.coefficient, tc.prec, true}} {
			req.Candidate = candidate
			if status, _ := verify(req); status != "mismatch" {
				t.Fatal(candidate, status)
			}
		}
	}
}
