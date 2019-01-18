package provider

import (
	"fmt"

	"github.com/PuerkitoBio/goquery"
	"github.com/rot1024/haken/core"
)

// Animemo is a provider
type Animemo struct{}

// GetAnimes implements Provider interface
func (Animemo) GetAnimes(course core.Course) (animes []core.Anime, err error) {
	monthStart, monthEnd := course.Season.MonthRange()
	url := fmt.Sprintf("http://animemo.jp/post/%04d%02d%02d", course.Year, monthStart, monthEnd)

	doc, err := getDocument(url)
	if err != nil {
		return nil, fmt.Errorf("animemo: %s", err)
	}

	animes = []core.Anime{}

	doc.Find("h4 > a").Each(func(i int, s *goquery.Selection) {
		title := s.Text()

		animes = append(animes, core.Anime{
			Course: course,
			Title:  title,
		})
	})

	return
}
