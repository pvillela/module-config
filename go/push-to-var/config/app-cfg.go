package config

import (
	"github.com/pvillela/module-config/go/push-to-var/fwk"
)

type AppCfgInfo struct {
	X string
	Y int
}

type AppCfgSrc = fwk.CfgSrc[AppCfgInfo]

func GetAppConfiguration() AppCfgInfo {
	return AppCfgInfo{
		X: "xxx",
		Y: 42,
	}
}
