package recorder

import (
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"net/http"

	"github.com/rot1024/haken/core"
)

// EPGStation is a recorder
type EPGStation struct {
	endpoint string
}

type epgStationRule struct {
	Option epgStationOption `json:"option"`
	Search epgStationSearch `json:"search"`
}

type epgStationOption struct {
	Enable       bool `json:"enable"`
	AllowEndLack bool `json:"allowEndLack"`
}

type epgStationSearch struct {
	BS          bool   `json:"BS"`
	CS          bool   `json:"CS"`
	GR          bool   `json:"GR"`
	SKY         bool   `json:"SKY"`
	Description bool   `json:"description"`
	Genrelv1    int    `json:"genrelv1"`
	Genrelv2    int    `json:"genrelv2"`
	Keyword     string `json:"keyword"`
	Title       bool   `json:"title"`
	Week        int    `json:"week"`
}

func newEpgStationRule(rule core.Rule) epgStationRule {
	return epgStationRule{
		Option: epgStationOption{Enable: true, AllowEndLack: true},
		Search: epgStationSearch{
			BS:          true,
			CS:          true,
			GR:          true,
			SKY:         true,
			Description: true,
			Genrelv1:    7, //　国内アニメ
			Genrelv2:    0,
			Keyword:     rule.Keyword,
			Title:       true,
			Week:        127, // 全ての曜日
		},
	}
}

// NewEPGStation creates a new EPGStation
func NewEPGStation(endpoint string) EPGStation {
	return EPGStation{endpoint: endpoint}
}

// RegisterRule implements Recorder interface
func (e EPGStation) RegisterRule(rule core.Rule) error {
	url := e.endpoint + "/rules"
	body, err := json.Marshal(newEpgStationRule(rule))
	if err != nil {
		return errors.New("epgstation: invalid JSON")
	}

	res, err := http.Post(url, "application/json", bytes.NewBuffer(body))
	if err != nil {
		return errors.New("animemo: failed to post")
	}
	if res.StatusCode != http.StatusCreated {
		return fmt.Errorf("animemo: status code error: %d", res.StatusCode)
	}

	// OK
	return nil
}
