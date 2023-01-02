package pullwithpushoverride.fs

import pullwithpushoverride.config.makeCfgSrc

data class BazCfgInfo(
	val w: String
)

var bazCfgSrc = makeCfgSrc<BazCfgInfo>(null)

fun baz() {
	println(bazCfgSrc().w.length);
}
