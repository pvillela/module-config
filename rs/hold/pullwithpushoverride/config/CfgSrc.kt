package pullwithpushoverride.config

import pullwithpushoverride.fwk.ConfigurationException

typealias CfgSrc<T> = () -> T

private fun <T>nilCfgSrc(): T {
	throw ConfigurationException("Module used before being initialized")
}

fun <T>makeCfgSrc(adapter: ((AppCfgInfo) -> T)?): CfgSrc<T> {
	if (adapter == null) {
		return ::nilCfgSrc
	}
	return { adapter(getAppConfiguration()) }
}
