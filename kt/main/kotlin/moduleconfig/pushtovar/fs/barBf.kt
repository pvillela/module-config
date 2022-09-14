/*
 *  Copyright © 2021 Paulo Villela. All rights reserved.
 *  Use of this source code is governed by the Apache 2.0 license
 *  that can be found in the LICENSE file.
 */

package tryout.moduleconfig.pushtovar.fs

import tryout.moduleconfig.pushtovar.fwk.CfgSrc

data class BarBfCfgInfo(
	val z: Int
)

val barBfCfgSrc = CfgSrc<BarBfCfgInfo>()

fun barBf() {
	println(barBfCfgSrc().z)
}
