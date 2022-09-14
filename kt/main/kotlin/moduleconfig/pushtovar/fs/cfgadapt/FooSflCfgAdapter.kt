package tryout.moduleconfig.pushtovar.fs.cfgadapt

import tryout.moduleconfig.pushtovar.config.AppCfgInfo
import tryout.moduleconfig.pushtovar.fs.FooSflCfgInfo
import tryout.moduleconfig.pushtovar.fs.fooSflCfgSrc
import tryout.moduleconfig.pushtovar.fwk.CfgSrcAdaptation

fun fooSflCfgAdapter(appCfg: AppCfgInfo): FooSflCfgInfo {
	return FooSflCfgInfo(
        x = appCfg.x,
    )
}

val fooSflCfgAdaptation = CfgSrcAdaptation(
	fooSflCfgSrc,
	::fooSflCfgAdapter,
)
