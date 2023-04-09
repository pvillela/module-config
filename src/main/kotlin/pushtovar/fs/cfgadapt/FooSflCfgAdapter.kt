package pushtovar.fs.cfgadapt

import pushtovar.config.AppCfgInfo
import pushtovar.fs.FooSflCfgInfo
import pushtovar.fs.fooSflCfgSrc
import pushtovar.fwk.CfgSrcAdaptation

fun fooSflCfgAdapter(appCfg: AppCfgInfo): FooSflCfgInfo {
	return FooSflCfgInfo(
        x = appCfg.x,
    )
}

val fooSflCfgAdaptation = CfgSrcAdaptation(
	fooSflCfgSrc,
	::fooSflCfgAdapter,
)
