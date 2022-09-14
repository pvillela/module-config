/*
 *  Copyright © 2021 Paulo Villela. All rights reserved.
 *  Use of this source code is governed by the Apache 2.0 license
 *  that can be found in the LICENSE file.
 */

package tryout.moduleconfig.pushtofunction.fs

data class BarBfCfgInfo(
	val z: Int
)

data class BarBfCfgSrc(
	val get: () -> BarBfCfgInfo
)

typealias BarBfT = () -> Unit

fun barBfC(cfg: BarBfCfgSrc): BarBfT {
	return { println(cfg.get().z) }
}
