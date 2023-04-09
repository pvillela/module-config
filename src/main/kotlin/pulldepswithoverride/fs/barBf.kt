/*
 *  Copyright © 2022 Paulo Villela. All rights reserved.
 *  Use of this source code is governed by the MIT license
 *  that can be found in the LICENSE file.
 */

package pulldepswithoverride.fs

import pulldepswithoverride.config.AppCfgInfo
import pulldepswithoverride.config.CfgSrc
import pulldepswithoverride.config.makeCfgSrc

data class BarBfCfgInfo(
	val z: Int
)

var barBfCfgSrc: CfgSrc<BarBfCfgInfo> =
//	makeCfgSrc(null) // this is used before barBfCfgAdapter and/or the app config source are defined
	makeCfgSrc(::barBfCfgAdapter) // replace above with this after barBfCfgAdapter has been created

fun barBf() {
	println(barBfCfgSrc().z)
}

// This doesn't necessarily exist initially and may be added later, after the
// app configuration source has been created.
fun barBfCfgAdapter(appCfg: AppCfgInfo): BarBfCfgInfo {
	return BarBfCfgInfo(
		z = appCfg.y,
	)
}
