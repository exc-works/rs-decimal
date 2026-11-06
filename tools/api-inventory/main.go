// Export public Go declarations without loading dependencies or filtering build tags.
package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"go/ast"
	"go/parser"
	"go/printer"
	"go/token"
	"os"
	"path/filepath"
	"sort"
	"strings"
)

type entry struct {
	ID        string `json:"id"`
	Kind      string `json:"kind"`
	Signature string `json:"signature"`
	File      string `json:"file"`
	Line      int    `json:"line"`
	Build     string `json:"build"`
}

func main() {
	if len(os.Args) != 2 {
		panic("usage: go run tools/api-inventory/main.go GO_SOURCE_DIRECTORY")
	}
	files, err := filepath.Glob(filepath.Join(os.Args[1], "*.go"))
	if err != nil {
		panic(err)
	}
	fset := token.NewFileSet()
	var entries []entry
	for _, path := range files {
		if strings.HasSuffix(path, "_test.go") {
			continue
		}
		f, err := parser.ParseFile(fset, path, nil, parser.ParseComments)
		if err != nil {
			panic(err)
		}
		if f.Name.Name != "decimal" {
			continue
		}
		build := ""
		for _, cg := range f.Comments {
			for _, c := range cg.List {
				if strings.HasPrefix(c.Text, "//go:build ") {
					build = strings.TrimPrefix(c.Text, "//go:build ")
				}
			}
		}
		add := func(id, kind string, n ast.Node) {
			var buf bytes.Buffer
			printNode := n
			if field, ok := n.(*ast.Field); ok {
				for _, name := range field.Names {
					buf.WriteString(name.Name + " ")
				}
				printNode = field.Type
			}
			if err := printer.Fprint(&buf, fset, printNode); err != nil {
				panic(err)
			}
			entries = append(entries, entry{id, kind, strings.Join(strings.Fields(buf.String()), " "), filepath.Base(path), fset.Position(n.Pos()).Line, build})
		}
		for _, d := range f.Decls {
			switch d := d.(type) {
			case *ast.FuncDecl:
				if !d.Name.IsExported() {
					continue
				}
				id := d.Name.Name
				if d.Recv != nil {
					var b bytes.Buffer
					if err := printer.Fprint(&b, fset, d.Recv.List[0].Type); err != nil {
						panic(err)
					}
					receiver := strings.TrimPrefix(b.String(), "*")
					if !ast.IsExported(receiver) {
						continue
					}
					id = receiver + "." + id
				}
				d.Body = nil
				d.Doc = nil
				add(id, "func", d)
			case *ast.GenDecl:
				for _, s := range d.Specs {
					switch s := s.(type) {
					case *ast.TypeSpec:
						if !s.Name.IsExported() {
							continue
						}
						s.Doc = nil
						s.Comment = nil
						add(s.Name.Name, "type", s)
						if st, ok := s.Type.(*ast.StructType); ok {
							for _, field := range st.Fields.List {
								for _, name := range field.Names {
									if name.IsExported() {
										add(s.Name.Name+"."+name.Name, "field", field)
									}
								}
							}
						}
					case *ast.ValueSpec:
						for _, name := range s.Names {
							if name.IsExported() {
								s.Doc = nil
								s.Comment = nil
								add(name.Name, d.Tok.String(), s)
							}
						}
					}
				}
			}
		}
	}
	if len(entries) == 0 {
		panic("no public declarations found")
	}
	sort.Slice(entries, func(i, j int) bool { return entries[i].ID < entries[j].ID })
	for i := 1; i < len(entries); i++ {
		if entries[i-1].ID == entries[i].ID {
			panic(fmt.Sprintf("duplicate public ID %s", entries[i].ID))
		}
	}
	enc := json.NewEncoder(os.Stdout)
	enc.SetIndent("", "  ")
	if err := enc.Encode(entries); err != nil {
		panic(err)
	}
}
