package tryout.moduleconfig.pullwithpushoverride.fs

import tryout.moduleconfig.pullwithpushoverride.config.makeCfgSrc

data class BazCfgInfo(
	val w: String
)

var bazCfgSrc = makeCfgSrc<BazCfgInfo>(null)

fun baz() {
	println(bazCfgSrc().w.length);
}
