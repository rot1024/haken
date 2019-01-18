package core

const (
	// Winter is a kind of season: January - March
	Winter Season = iota
	// Spring is a kind of season: April - June
	Spring
	// Summer is a kind of season: July - September
	Summer
	// Autumn is a kind of season: October - December
	Autumn
)

// Year is a year
type Year int

// Season is a season
type Season int

// MonthRange returns a month at which the course begins and ends
func (s Season) MonthRange() (int, int) {
	switch s {
	case Winter:
		return 1, 3
	case Spring:
		return 4, 6
	case Summer:
		return 7, 9
	case Autumn:
		return 10, 12
	}
	return 0, 0
}

// Name returns a name
func (s Season) Name() string {
	switch s {
	case Winter:
		return "Winter"
	case Spring:
		return "Spring"
	case Summer:
		return "Summer"
	case Autumn:
		return "Autumn"
	}
	return ""
}

// Course is a course
type Course struct {
	Year   Year
	Season Season
}

// Anime is an anime
type Anime struct {
	Course Course
	Title  string
}

// Rule is a rule to record animes
type Rule struct {
	Keyword string
}
