package flatted

import (
	"encoding/json"
	"os"
	"reflect"
	"regexp"
	"testing"
	"time"
)

func TestFlatted(t *testing.T) {
	t.Run("Primitives", func(t *testing.T) {
		cases := []any{"a", 1.0, true, false, nil}
		for _, c := range cases {
			s, _ := Stringify(c, nil, nil)
			p, _ := Parse(s, nil)
			if p != c {
				t.Errorf("Expected %v, got %v", c, p)
			}
		}
	})

	t.Run("Array", func(t *testing.T) {
		a := []any{"a", 1.0, "b"}
		s, _ := Stringify(a, nil, nil)
		p, _ := Parse(s, nil)
		if !reflect.DeepEqual(a, p) {
			t.Errorf("Expected %v, got %v", a, p)
		}
	})

	t.Run("Object", func(t *testing.T) {
		o := map[string]any{"a": "a", "n": 1.0, "b": "b"}
		s, _ := Stringify(o, nil, nil)
		p, _ := Parse(s, nil)
		if !reflect.DeepEqual(o, p) {
			t.Errorf("Expected %v, got %v", o, p)
		}
	})

	t.Run("ToFromJSON", func(t *testing.T) {
		a := &[]any{map[string]any{}}
		(*a)[0].(map[string]any)["a"] = a

		jsonObj, err := ToJSON(a)
		if err != nil {
			t.Fatal(err)
		}

		back, err := FromJSON(jsonObj)
		if err != nil {
			t.Fatal(err)
		}

		// Verify recursion is preserved
		m := back.([]any)[0].(map[string]any)
		// In Go, maps and slices are not comparable via == or !=.
		// We check their pointers to verify that the circular reference is maintained.
		if reflect.ValueOf(m["a"]).Pointer() != reflect.ValueOf(back).Pointer() {
			t.Error("Recursion lost in ToJSON/FromJSON roundtrip")
		}
	})
}

func TestJSParity(t *testing.T) {
	t.Run("MultipleNulls", func(t *testing.T) {
		s, _ := Stringify([]any{nil, nil}, nil, nil)
		if s != "[[null,null]]" {
			t.Errorf("Expected [[null,null]], got %s", s)
		}
	})

	t.Run("EmptyCollections", func(t *testing.T) {
		s1, _ := Stringify([]any{}, nil, nil)
		if s1 != "[[]]" {
			t.Errorf("Expected [[]], got %s", s1)
		}
		s2, _ := Stringify(map[string]any{}, nil, nil)
		if s2 != "[{}]" {
			t.Errorf("Expected [{}], got %s", s2)
		}
	})

	t.Run("RecursiveArray", func(t *testing.T) {
		a := &[]any{}
		*a = append(*a, a)
		s, _ := Stringify(a, nil, nil)
		if s != "[[\"0\"]]" {
			t.Errorf("Expected [[\"0\"]], got %s", s)
		}
		p, _ := Parse(s, nil)
		pa := p.([]any)
		if reflect.ValueOf(pa).Pointer() != reflect.ValueOf(pa[0]).Pointer() {
			t.Error("Recursive array reference lost")
		}
	})

	t.Run("RecursiveObject", func(t *testing.T) {
		o := map[string]any{}
		o["o"] = o
		s, _ := Stringify(o, nil, nil)
		if s != "[{\"o\":\"0\"}]" {
			t.Errorf("Expected [{\"o\":\"0\"}], got %s", s)
		}
		p, _ := Parse(s, nil)
		po := p.(map[string]any)
		if reflect.ValueOf(po).Pointer() != reflect.ValueOf(po["o"]).Pointer() {
			t.Error("Recursive object reference lost")
		}
	})

	t.Run("SpecialStrings", func(t *testing.T) {
		special := "\\x7e"
		o := map[string]any{"a": special}
		s, _ := Stringify(o, nil, nil)
		p, _ := Parse(s, nil)
		if p.(map[string]any)["a"] != special {
			t.Errorf("Expected %s, got %v", special, p.(map[string]any)["a"])
		}
	})

	t.Run("DateRevival", func(t *testing.T) {
		d := time.Date(2023, 10, 27, 10, 0, 0, 0, time.UTC)
		s, _ := Stringify(d, nil, nil)

		dateRegex := regexp.MustCompile(`^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}`)
		reviver := func(key string, value any) any {
			if str, ok := value.(string); ok && dateRegex.MatchString(str) {
				t, _ := time.Parse(time.RFC3339, str)
				return t
			}
			return value
		}

		p, _ := Parse(s, reviver)
		if !p.(time.Time).Equal(d) {
			t.Errorf("Expected %v, got %v", d, p)
		}
	})

	t.Run("ReplacerWhitelist", func(t *testing.T) {
		o := map[string]any{"a": 1.0, "b": 2.0}
		s, _ := Stringify(o, []string{"b"}, nil)
		if s != `[{"b":2}]` {
			t.Errorf("Expected [{\"b\":2}], got %s", s)
		}
	})

	t.Run("Indentation", func(t *testing.T) {
		o := map[string]any{"a": 1.0}
		s, _ := Stringify(o, nil, 2)
		expected := "[\n  {\n    \"a\": 1\n  }\n]"
		if s != expected {
			t.Errorf("Indentation mismatch.\nExpected:\n%s\nGot:\n%s", expected, s)
		}
	})
}

func TestComplexStructure(t *testing.T) {
	unique := map[string]any{"a": "sup"}
	nested := map[string]any{
		"prop": map[string]any{"value": 123.0},
		"a": []any{
			map[string]any{},
			map[string]any{
				"b": []any{
					map[string]any{
						"a": 1.0,
						"d": 2.0,
						"c": unique,
						"z": map[string]any{
							"g": 2.0,
							"a": unique,
							"b": map[string]any{
								"r": 4.0,
								"u": unique,
								"c": 5.0,
							},
							"f": 6.0,
						},
						"h": 1.0,
					},
				},
			},
		},
		"b": map[string]any{
			"e": "f",
			"t": unique,
			"p": 4.0,
		},
	}

	s, _ := Stringify(nested, nil, nil)
	p, _ := Parse(s, nil)

	// Verify structural integrity and shared references
	res := p.(map[string]any)
	resB := res["b"].(map[string]any)
	resA := res["a"].([]any)[1].(map[string]any)["b"].([]any)[0].(map[string]any)

	if !reflect.DeepEqual(resB["t"], resA["c"]) {
		t.Error("Shared object reference values differ")
	}

	if reflect.ValueOf(resB["t"]).Pointer() != reflect.ValueOf(resA["c"]).Pointer() {
		t.Error("Shared object reference identity lost")
	}
}

func TestEmptyKeys(t *testing.T) {
	inner := map[string]any{"d": 1.0}
	emptyKeyMap := map[string]any{"c": inner}
	bMap := map[string]any{"": emptyKeyMap}
	a := map[string]any{
		"b":         bMap,
		"_circular": emptyKeyMap,
	}

	s, _ := Stringify(a, nil, nil)
	p, _ := Parse(s, nil)

	res := p.(map[string]any)
	if reflect.ValueOf(res["_circular"]).Pointer() != reflect.ValueOf(res["b"].(map[string]any)[""]).Pointer() {
		t.Error("Circular reference via empty key lost")
	}
}

func TestCircularReference(t *testing.T) {
	// Use a pointer to the slice to ensure a stable reference
	// that survives append operations and matches JS reference behavior.
	a := &[]any{map[string]any{}}
	(*a)[0].(map[string]any)["a"] = a
	*a = append(*a, a)

	s, err := Stringify(a, nil, nil)
	if err != nil {
		t.Fatal(err)
	}

	if s != `[["1","0"],{"a":"0"}]` {
		t.Errorf("Unexpected stringify output: %s", s)
	}
}

func TestLargeFiles(t *testing.T) {
	files := []string{"65515.json", "65518.json"}
	for _, fileName := range files {
		t.Run(fileName, func(t *testing.T) {
			// Path relative to the golang directory where tests are executed
			data, err := os.ReadFile("../test/" + fileName)
			if err != nil {
				t.Skipf("Skipping %s: %v (file might not be present in all environments)", fileName, err)
				return
			}

			var raw map[string]any
			if err := json.Unmarshal(data, &raw); err != nil {
				t.Fatalf("Failed to unmarshal %s: %v", fileName, err)
			}

			toolData := raw["toolData"]
			b, _ := json.Marshal(toolData)
			res, err := Parse(string(b), nil)
			if err != nil {
				t.Errorf("Parse failed for %s: %v", fileName, err)
			}
			if res == nil {
				t.Errorf("Parse returned nil for %s", fileName)
			}
		})
	}
}

func TestConventionalComparison(t *testing.T) {
	t.Run("CircularFailure", func(t *testing.T) {
		type Item struct {
			Self *Item
		}
		item := &Item{}
		item.Self = item

		// Conventional serialization fails on circular references
		_, err := json.Marshal(item)
		if err == nil {
			t.Error("Expected encoding/json to fail on circular reference")
		}

		// Flatted handles it
		_, err = Stringify(item, nil, nil)
		if err != nil {
			t.Errorf("Expected flatted to handle circular reference, got error: %v", err)
		}
	})

	t.Run("OutputDifference", func(t *testing.T) {
		data := map[string]string{"key": "value"}
		std, _ := json.Marshal(data)
		flat, _ := Stringify(data, nil, nil)

		if string(std) == flat {
			t.Error("Flatted output should be distinct from standard JSON (it flattens into an array)")
		}
	})
}
