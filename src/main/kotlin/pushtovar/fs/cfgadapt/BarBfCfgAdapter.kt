package pushtovar.fs.cfgadapt

import pushtovar.config.AppCfgInfo
import pushtovar.fs.BarBfCfgInfo
import pushtovar.fs.barBfCfgSrc
import pushtovar.fwk.CfgSrcAdaptation

fun barBfCfgAdapter(appCfg: AppCfgInfo): BarBfCfgInfo {
	return BarBfCfgInfo(
        z = appCfg.y,
    )
}

val barBfCfgAdaptation = CfgSrcAdaptation(
	barBfCfgSrc,
	::barBfCfgAdapter,
)
