/*
 *  Copyright © 2021 Paulo Villela. All rights reserved.
 *  Use of this source code is governed by the Apache 2.0 license
 *  that can be found in the LICENSE file.
 */

package tryout.moduleconfig.pullwithpushoverride.fs

import tryout.moduleconfig.pullwithpushoverride.config.CfgSrc
import tryout.moduleconfig.pullwithpushoverride.config.makeCfgSrc

data class BarBfCfgInfo(
	val z: Int
)

var barBfCfgSrc: CfgSrc<BarBfCfgInfo> = makeCfgSrc(::barBfCfgAdapter)

fun barBf() {
	println(barBfCfgSrc().z)
}
