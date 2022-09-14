package tryout.moduleconfig.pushtofunction.config

import java.util.concurrent.atomic.AtomicReference

data class AppCfgInfo (
	val x: String,
	val y: Int,
)

// Additional complexity added here on purpuse to enable dynamic changes to
// app configuration properties.
val appCfgInfo = AtomicReference(
	AppCfgInfo(
		"xxx",
		42,
	)
)

fun getAppConfiguration(): AppCfgInfo {
	return appCfgInfo.get()
}
