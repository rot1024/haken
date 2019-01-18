package main

import (
	"errors"
	"fmt"
	"log"
	"time"

	"github.com/rot1024/haken/core"
	"github.com/rot1024/haken/provider"
	"github.com/rot1024/haken/recorder"
	"github.com/spf13/viper"
	survey "gopkg.in/AlecAivazis/survey.v1"
)

var providers = map[string](func(config map[string]string) core.Provider){
	"animemo": func(_ map[string]string) core.Provider {
		return provider.Animemo{}
	},
}

var recorders = map[string](func(config map[string]string) core.Recorder){
	"epgstation": func(confing map[string]string) core.Recorder {
		url := confing["epgstation_endpoint"]
		if url == "" {
			log.Fatal(errors.New("epgstation_endpoint is required"))
		}
		return recorder.NewEPGStation(url)
	},
}

func main() {
	config, err := getConfig()
	if err != nil {
		log.Fatal(err)
	}

	providerName, ok := config["provider"]
	if !ok {
		log.Fatal(errors.New("provider is required"))
	}
	recorderName, ok := config["recorder"]
	if !ok {
		log.Fatal(errors.New("recorder is required"))
	}

	providerFn, ok := providers[providerName]
	if !ok {
		log.Fatal(errors.New("provider is invalid"))
	}
	recorderFn, ok := recorders[recorderName]
	if !ok {
		log.Fatal(errors.New("recorder is invalid"))
	}

	provider := providerFn(config)
	recorder := recorderFn(config)

	course := getCurrentCourse()

	fmt.Printf("Provider: %s\nRecorder: %s\n", providerName, recorderName)
	fmt.Printf("Year: %04d\nSeason: %s\n\n", course.Year, course.Season.Name())

	result, err := provider.GetAnimes(course)
	if err != nil {
		log.Fatal(err)
	}

	selected := selectAnimes(result)
	rules := createRulesFromAnimes(selected)
	err = registerRules(recorder, rules)

	if err != nil {
		log.Fatal(err)
	}

	count := len(rules)
	if count == 0 {
		println("No new rule registered")
	} else {
		fmt.Printf("%d rules are registerd!\n", len(rules))
	}
}

func getConfig() (map[string]string, error) {
	viper.SetConfigName("config")
	viper.AddConfigPath("/etc/haken")
	viper.AddConfigPath("$HOME/.config/haken")
	viper.AddConfigPath("$HOME/.haken")
	viper.AddConfigPath(".")

	err := viper.ReadInConfig()
	if err != nil {
		return nil, err
	}

	config := map[string]string{}
	viper.Unmarshal(&config)

	return config, nil
}

func getCurrentCourse() (course core.Course) {
	now := time.Now()
	year := now.Year()
	month := now.Month()

	if month == time.December {
		course.Year = core.Year(year + 1)
	} else {
		course.Year = core.Year(year)
	}

	switch month {
	case time.December, time.January, time.February:
		course.Season = core.Winter
	case time.March, time.April, time.May:
		course.Season = core.Spring
	case time.June, time.July, time.August:
		course.Season = core.Summer
	default:
		course.Season = core.Autumn
	}

	return
}

func selectAnimes(animes []core.Anime) []core.Anime {
	selected := []string{}
	options := make([]string, len(animes))

	for i, a := range animes {
		options[i] = a.Title
	}

	prompt := &survey.MultiSelect{
		Message:  "Select anime you want to record:",
		Options:  options,
		PageSize: 20,
	}
	survey.AskOne(prompt, &selected, nil)

	selectedAnime := make([]core.Anime, len(selected))
	for i, s := range selected {
		for _, a := range animes {
			if a.Title == s {
				selectedAnime[i] = a
			}
		}
	}

	return selectedAnime
}

func createRulesFromAnimes(animes []core.Anime) []core.Rule {
	rules := make([]core.Rule, len(animes))

	for i, a := range animes {
		rules[i] = core.Rule{
			Keyword: a.Title,
		}
	}

	return rules
}

func registerRules(rec core.Recorder, rules []core.Rule) error {
	for _, rule := range rules {
		err := rec.RegisterRule(rule)
		if err != nil {
			return err
		}
	}
	return nil
}
