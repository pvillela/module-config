package tryout.moduleconfig.pushtovar.fs.cfgadapt

import tryout.moduleconfig.pushtovar.config.AppCfgInfo
import tryout.moduleconfig.pushtovar.fs.BarBfCfgInfo
import tryout.moduleconfig.pushtovar.fs.barBfCfgSrc
import tryout.moduleconfig.pushtovar.fwk.CfgSrcAdaptation

fun barBfCfgAdapter(appCfg: AppCfgInfo): BarBfCfgInfo {
	return BarBfCfgInfo(
        z = appCfg.y,
    )
}

val barBfCfgAdaptation = CfgSrcAdaptation(
	barBfCfgSrc,
	::barBfCfgAdapter,
)
