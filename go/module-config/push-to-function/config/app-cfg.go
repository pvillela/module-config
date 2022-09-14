package config

type AppCfgInfo struct {
	X string
	Y int
}

type AppCfgSrc = func() AppCfgInfo

func GetAppConfiguration() AppCfgInfo {
	return AppCfgInfo{
		X: "xxx",
		Y: 42,
	}
}
