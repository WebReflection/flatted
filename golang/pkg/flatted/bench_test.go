package flatted

import (
	"encoding/json"
	"os"
	"testing"
)

func BenchmarkFlatted(b *testing.B) {
	// Create a circular structure for benchmarking
	a := &[]any{map[string]any{}}
	for i := 0; i < 10; i++ {
		*a = append(*a, map[string]any{"id": i, "ref": a})
	}
	(*a)[0].(map[string]any)["root"] = a

	s, _ := Stringify(a, nil, nil)

	b.Run("Stringify", func(b *testing.B) {
		b.ReportAllocs()
		for i := 0; i < b.N; i++ {
			_, err := Stringify(a, nil, nil)
			if err != nil {
				b.Fatal(err)
			}
		}
	})

	b.Run("Parse", func(b *testing.B) {
		b.ReportAllocs()
		for i := 0; i < b.N; i++ {
			_, _ = Parse(s, nil)
		}
	})
}

func BenchmarkCompare(b *testing.B) {
	// Non-circular data for comparison
	data := map[string]any{
		"name": "test",
		"list": []any{1, 2, 3, 4, 5},
		"nested": map[string]any{
			"x": 1.0,
			"y": 2.0,
		},
	}

	b.Run("Flatted", func(b *testing.B) {
		for i := 0; i < b.N; i++ {
			_, _ = Stringify(data, nil, nil)
		}
	})

	b.Run("JSON", func(b *testing.B) {
		for i := 0; i < b.N; i++ {
			_, _ = json.Marshal(data)
		}
	})
}

func BenchmarkLargeFile(b *testing.B) {
	data, err := os.ReadFile("../test/65515.json")
	if err != nil {
		b.Skip("Skipping large file benchmark: file not found")
	}
	var raw map[string]any
	_ = json.Unmarshal(data, &raw)
	toolData := raw["toolData"]
	strBytes, _ := json.Marshal(toolData)
	str := string(strBytes)
	obj, _ := Parse(str, nil)

	b.Run("Stringify", func(b *testing.B) {
		for i := 0; i < b.N; i++ {
			_, _ = Stringify(obj, nil, nil)
		}
	})
	b.Run("Parse", func(b *testing.B) {
		for i := 0; i < b.N; i++ {
			_, _ = Parse(str, nil)
		}
	})
}
