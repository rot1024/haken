package core

// Provider is an anime information provider
type Provider interface {
	// GetAnimes gets anime list for
	GetAnimes(Course) ([]Anime, error)
}
